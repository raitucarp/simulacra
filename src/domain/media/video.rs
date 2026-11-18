//! Video links: the watch, short, live and embed URL forms of the major video host (and its
//! short-link and privacy-enhanced domains), plus links straight to a video file. The host is
//! recognised here in code only; cards say "Video" or show the domain.

use url::Url;

/// Hosts serving `/watch?v=`, `/shorts/<id>`, `/embed/<id>`, `/live/<id>` and `/v/<id>`.
const WATCH_HOSTS: [&str; 6] = [
    "youtube.com", "www.youtube.com", "m.youtube.com", "music.youtube.com", "youtube-nocookie.com", "www.youtube-nocookie.com",
];
/// The short-link host (`/<id>`).
const SHORT_HOST: &str = "youtu.be";
/// Path forms that carry the id as their second segment.
const ID_PATHS: [&str; 4] = ["shorts", "embed", "live", "v"];
/// The watch path and its id query parameter.
const WATCH_PATH: &str = "watch";
const WATCH_ID_PARAM: &str = "v";
/// Length and alphabet of a video id.
const VIDEO_ID_LEN: usize = 11;
/// Still frame served for every hosted video (4:3 with bars; cards crop it to 16:9).
const THUMBNAIL_TEMPLATE: (&str, &str) = ("https://i.ytimg.com/vi/", "/hqdefault.jpg");
/// File extensions of links straight to a video.
const VIDEO_EXTENSIONS: [&str; 7] = ["mp4", "webm", "mov", "m4v", "mkv", "ogv", "avi"];
/// Shape of a video card's frame.
pub const VIDEO_ASPECT: f32 = 16.0 / 9.0;
const SECONDS_PER_MINUTE: u32 = 60;
const SECONDS_PER_HOUR: u32 = 3600;

/// Where a video link points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoSource {
    /// A video on the major video host, by its id.
    Hosted { id: String },
    /// A link straight to a video file.
    File,
}

/// A recognised video link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoLink {
    pub url: String,
    pub source: VideoSource,
}

impl VideoLink {
    /// The still frame of a hosted video (files have none without decoding them).
    pub fn thumbnail_url(&self) -> Option<String> {
        match &self.source {
            VideoSource::Hosted { id } => Some(format!("{}{id}{}", THUMBNAIL_TEMPLATE.0, THUMBNAIL_TEMPLATE.1)),
            VideoSource::File => None,
        }
    }

    /// Whether the page behind the link can be read for a title (a file has no page).
    pub fn has_page(&self) -> bool {
        matches!(self.source, VideoSource::Hosted { .. })
    }
}

/// `url` as a video link, if it is one.
pub fn recognise_video(url: &str) -> Option<VideoLink> {
    let parsed = Url::parse(url.trim()).ok().filter(|u| matches!(u.scheme(), "http" | "https"))?;
    let host = parsed.host_str()?.to_ascii_lowercase();
    let segments: Vec<&str> = parsed.path_segments().map(|s| s.filter(|s| !s.is_empty()).collect()).unwrap_or_default();
    let hosted_id = if host == SHORT_HOST {
        segments.first().map(|id| id.to_string())
    } else if WATCH_HOSTS.contains(&host.as_str()) {
        hosted_path_id(&parsed, &segments)
    } else {
        None
    };
    let source = match hosted_id.filter(|id| is_video_id(id)) {
        Some(id) => VideoSource::Hosted { id },
        None if is_video_file(&segments) => VideoSource::File,
        None => return None,
    };
    Some(VideoLink { url: parsed.to_string(), source })
}

/// The id of a watch-host URL: `?v=` on `/watch`, else the segment after a known path form.
fn hosted_path_id(url: &Url, segments: &[&str]) -> Option<String> {
    match segments {
        [WATCH_PATH, ..] => url.query_pairs().find(|(key, _)| key == WATCH_ID_PARAM).map(|(_, id)| id.into_owned()),
        [form, id, ..] if ID_PATHS.contains(form) => Some(id.to_string()),
        _ => None,
    }
}

fn is_video_id(id: &str) -> bool {
    id.len() == VIDEO_ID_LEN && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn is_video_file(segments: &[&str]) -> bool {
    let extension = segments.last().and_then(|name| name.rsplit_once('.')).map(|(_, ext)| ext.to_ascii_lowercase());
    extension.is_some_and(|ext| VIDEO_EXTENSIONS.contains(&ext.as_str()))
}

/// `4:05`, or `1:02:03` past an hour.
pub fn format_duration(total_seconds: u32) -> String {
    let (hours, rest) = (total_seconds / SECONDS_PER_HOUR, total_seconds % SECONDS_PER_HOUR);
    let (minutes, seconds) = (rest / SECONDS_PER_MINUTE, rest % SECONDS_PER_MINUTE);
    match hours {
        0 => format!("{minutes}:{seconds:02}"),
        _ => format!("{hours}:{minutes:02}:{seconds:02}"),
    }
}
