//! The media pump (sb C3). On each render it checks cheaply whether the shown stream, the
//! cache or the privacy preference changed; when they did it plans which link previews to
//! fetch and which images to scan, and starts a few. Fetches run on their own threads (a slow
//! site never holds the executor), scans on the background executor. Results land in the
//! catalog and the vault, and the cards re-render with them.

use super::super::media_state::MediaWatch;
use super::super::view::AppView;
use crate::domain::media::catalog::ImageScan;
use crate::domain::media::{media_wants, MediaWants, PreviewEntry};
use crate::domain::Post;
use crate::media::{fetch_preview_blocking, scan_image, PreviewJob};
use gpui_kit::*;
use std::path::PathBuf;
use std::rc::Rc;

/// Previews fetched and images scanned at the same time, at most.
const MAX_FETCHES: usize = 4;
const MAX_SCANS: usize = 2;
const MEDIA_THREAD_NAME: &str = "simulacra-media";

fn unix_now() -> i64 {
    jiff::Timestamp::now().as_second()
}

impl AppView {
    /// Starts whatever previews and scans the posts on screen still need.
    pub(in crate::ui::features::shell) fn sync_media(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.load_media_cache();
        let watch = self.media_watch();
        if self.media.watched.as_ref() == Some(&watch) {
            return;
        }
        self.media.watched = Some(watch);
        let wants = self.media_wants();
        let (fetching, scanning) = self.state.media.in_flight();
        let urls = self.state.media.previews_due(wants.urls(), unix_now(), MAX_FETCHES.saturating_sub(fetching));
        let paths = self.state.media.images_due(wants.images.iter().map(String::as_str), MAX_SCANS.saturating_sub(scanning));
        if urls.is_empty() && paths.is_empty() {
            return;
        }
        self.state.media = std::mem::take(&mut self.state.media).started(&urls, &paths);
        let media_dir = self.state.media_dir();
        for url in urls {
            let job = PreviewJob { allow_private: wants.allows_private(&url), url, media_dir: media_dir.clone() };
            self.start_preview_fetch(job, window, cx);
        }
        for path in paths {
            self.start_image_scan(path, media_dir.clone(), window, cx);
        }
    }

    /// Reads the cache from the vault once.
    fn load_media_cache(&mut self) {
        if std::mem::replace(&mut self.media.loaded, true) {
            return;
        }
        if let Some(catalog) = self.state.db.as_ref().and_then(|vault| vault.load_media_catalog().ok()) {
            self.state.media = catalog;
        }
    }

    fn media_watch(&self) -> MediaWatch {
        let rows = self.feed.stream.rows();
        MediaWatch {
            rows: Rc::as_ptr(&rows.positions) as *const usize as usize,
            shown: rows.shown,
            revision: self.state.media.revision(),
            previews_enabled: self.state.preferences().privacy.link_previews,
            selected: self.feed.selected_thread_post_id.clone(),
        }
    }

    /// The links and images of the shown stream rows and the inspected thread.
    fn media_wants(&self) -> MediaWants {
        let state = &self.state;
        let rows = self.feed.stream.rows();
        let shown = rows.positions.iter().take(rows.shown).filter_map(|&at| state.posts.get(at));
        let selected = self.feed.selected_thread_post_id.as_ref();
        let thread = selected.and_then(|id| state.get_post(id)).into_iter();
        let replies = selected.into_iter().flat_map(|id| state.index.reply_positions(id)).filter_map(|at| state.posts.get(at));
        let posts = shown.chain(thread).chain(replies).map(|post| self.shown_post(post));
        media_wants(posts.map(|post| (post, state.author_of(post).is_human_persona())), state.preferences().privacy.link_previews)
    }

    /// The post a card shows for `post` (an instant repost shows its original).
    fn shown_post<'a>(&'a self, post: &'a Post) -> &'a Post {
        match post.is_instant_repost() {
            true => self.state.get_post(&self.state.action_target(&post.id)).unwrap_or(post),
            false => post,
        }
    }

    /// Fetches one preview on its own thread and stores the result.
    fn start_preview_fetch(&mut self, job: PreviewJob, window: &mut Window, cx: &mut Context<Self>) {
        let (reply, answer) = tokio::sync::oneshot::channel();
        let spawned = std::thread::Builder::new().name(MEDIA_THREAD_NAME.into()).spawn(move || {
            let _ = reply.send(fetch_preview_blocking(&job, unix_now()));
        });
        if spawned.is_err() {
            return;
        }
        cx.spawn_in(window, async move |this, cx| {
            let Ok(entry) = answer.await else { return };
            let _ = this.update(cx, |this, cx| this.finish_preview(entry, cx));
        })
        .detach();
    }

    fn finish_preview(&mut self, entry: PreviewEntry, cx: &mut Context<Self>) {
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_link_preview(entry.clone());
        }
        self.state.media = std::mem::take(&mut self.state.media).with_preview(entry);
        cx.notify();
    }

    /// Scans one image on the background executor and stores the result.
    fn start_image_scan(&mut self, path: String, media_dir: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        let source = path.clone();
        let task = cx.background_executor().spawn(async move { scan_image(&source, &media_dir) });
        cx.spawn_in(window, async move |this, cx| {
            let scan = task.await;
            let _ = this.update(cx, |this, cx| this.finish_image_scan(path, scan, cx));
        })
        .detach();
    }

    fn finish_image_scan(&mut self, path: String, scan: ImageScan, cx: &mut Context<Self>) {
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_media_image(path.clone(), scan.clone(), unix_now());
        }
        self.state.media = std::mem::take(&mut self.state.media).with_image(path, scan);
        cx.notify();
    }
}
