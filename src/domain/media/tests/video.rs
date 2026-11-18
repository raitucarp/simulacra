use crate::domain::media::opengraph::parse_iso_duration;
use crate::domain::media::{format_duration, recognise_video, VideoSource};

const ID: &str = "dQw4w9WgXcQ";

fn hosted_id(url: &str) -> Option<String> {
    match recognise_video(url)?.source {
        VideoSource::Hosted { id } => Some(id),
        VideoSource::File => None,
    }
}

#[test]
fn recognises_watch_short_embed_and_live_forms() {
    let forms = [
        format!("https://www.youtube.com/watch?v={ID}"),
        format!("https://youtube.com/watch?feature=share&v={ID}&t=42"),
        format!("https://m.youtube.com/watch?v={ID}"),
        format!("https://music.youtube.com/watch?v={ID}"),
        format!("https://youtu.be/{ID}"),
        format!("https://youtu.be/{ID}?si=tracking"),
        format!("https://www.youtube.com/shorts/{ID}"),
        format!("https://www.youtube.com/embed/{ID}?start=10"),
        format!("https://www.youtube-nocookie.com/embed/{ID}"),
        format!("https://www.youtube.com/live/{ID}"),
        format!("HTTPS://WWW.YOUTUBE.COM/watch?v={ID}"),
    ];
    for url in forms {
        assert_eq!(hosted_id(&url).as_deref(), Some(ID), "{url}");
    }
}

#[test]
fn hosted_videos_get_a_still_frame_url() {
    let video = recognise_video(&format!("https://youtu.be/{ID}")).expect("video");
    assert_eq!(video.thumbnail_url().as_deref(), Some("https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg"));
    assert!(video.has_page());
}

#[test]
fn rejects_lookalikes_and_bad_ids() {
    for url in [
        "https://www.youtube.com/",
        "https://www.youtube.com/channel/UCabcdefghijk",
        "https://www.youtube.com/watch?v=short",
        "https://www.youtube.com/watch?v=has spaces!",
        "https://notyoutube.com/watch?v=dQw4w9WgXcQ",
        "https://youtube.com.evil.example/watch?v=dQw4w9WgXcQ",
        "ftp://youtu.be/dQw4w9WgXcQ",
        "https://example.org/watch?v=dQw4w9WgXcQ",
    ] {
        assert!(recognise_video(url).is_none(), "{url}");
    }
}

#[test]
fn links_to_video_files_are_videos_without_a_page() {
    for url in ["https://cdn.example.org/talks/lecture.MP4", "http://127.0.0.1:8000/clip.webm?dl=1", "https://a.org/v/movie.mov"] {
        let video = recognise_video(url).expect(url);
        assert_eq!(video.source, VideoSource::File, "{url}");
        assert!(video.thumbnail_url().is_none() && !video.has_page());
    }
    assert!(recognise_video("https://a.org/photo.jpg").is_none());
    assert!(recognise_video("https://a.org/mp4").is_none());
}

#[test]
fn formats_durations() {
    assert_eq!(format_duration(0), "0:00");
    assert_eq!(format_duration(253), "4:13");
    assert_eq!(format_duration(3723), "1:02:03");
}

#[test]
fn parses_iso_durations() {
    assert_eq!(parse_iso_duration("PT4M13S"), Some(253));
    assert_eq!(parse_iso_duration("PT1H2M3S"), Some(3723));
    assert_eq!(parse_iso_duration("P0DT0H3M1.5S"), Some(181));
    assert_eq!(parse_iso_duration("P1D"), Some(86_400));
    for bad in ["", "P", "PT", "4M13S", "P1M", "PT4X", "PTMS", "PT99999999999H"] {
        assert_eq!(parse_iso_duration(bad), None, "{bad:?}");
    }
}
