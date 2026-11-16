//! The `theme_tokens!` generator behind [`ThemeTokens`](super::ThemeTokens).

/// Generates `ThemeTokens` from field lists grouped by value kind.
///
/// Field order (and therefore the serialized JSON layout) follows the lists in order:
/// typed enums, strings, colors (`0xRRGGBBAA` numbers), pixels (`f32` numbers), then floats.
macro_rules! theme_tokens {
    (
        typed: [$($typed:ident: $ty:ty),* $(,)?],
        strings: [$($string:ident),* $(,)?],
        colors: [$($color:ident),* $(,)?],
        pixels: [$($pixel:ident),* $(,)?],
        floats: [$($float:ident),* $(,)?] $(,)?
    ) => {
        /// Structured theme tokens for the application interface.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        pub struct ThemeTokens {
            $(pub $typed: $ty,)*
            $(pub $string: String,)*
            $(#[serde(with = "rgba_serde")] pub $color: Rgba,)*
            $(#[serde(with = "px_serde")] pub $pixel: Pixels,)*
            $(pub $float: f32,)*
        }

        impl ThemeTokens {
            /// Zero-valued tokens; palettes fill every field on top of this.
            pub(crate) fn blank() -> Self {
                Self {
                    $($typed: <$ty>::default(),)*
                    $($string: String::new(),)*
                    $($color: rgba(0),)*
                    $($pixel: px(0.0),)*
                    $($float: 0.0,)*
                }
            }

            /// Snaps every color to 8-bit channels, so derived tokens equal their persisted form.
            pub(crate) fn quantize_colors(&mut self) {
                $(self.$color = crate::theme::color::quantized(self.$color);)*
            }
        }
    };
}
