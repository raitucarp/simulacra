//! Native image file dialog shared by the composer modal and the agent editor avatar picker.

/// Image extensions accepted by the picker.
const IMAGE_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "webp", "gif", "bmp"];

/// Opens a native "pick an image" dialog titled `title`; `None` when cancelled.
pub async fn pick_image_path(title: &str) -> Option<String> {
    let filter_name = rust_i18n::t!("picker.image_filter").to_string();
    let file = rfd::AsyncFileDialog::new()
        .set_title(title)
        .add_filter(filter_name, &IMAGE_EXTENSIONS)
        .pick_file()
        .await?;
    Some(file.path().to_string_lossy().to_string())
}
