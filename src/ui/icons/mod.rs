//! Custom Lucide icon mapping to gpui-kit's Icon component.
//! Maps 100% Lucide icons directly into gpui-kit Icon without relying on gpui-kit's built-in icon enum.

mod catalog;

pub use catalog::LucideIcon;

use gpui_kit::assets::IconNamed;
use gpui_kit::component::Icon;
use gpui_kit::gpui::{App, IntoElement, RenderOnce, SharedString, Window};

impl LucideIcon {
    /// Converts this Lucide icon directly into gpui-kit's Icon component.
    pub fn icon(self) -> Icon {
        Icon::new(self)
    }
}

impl IconNamed for LucideIcon {
    fn path(self) -> SharedString {
        self.path()
    }
}

impl RenderOnce for LucideIcon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Icon::new(self)
    }
}

/// Lucide mapping namespace for ergonomic syntax `Lucide::icon(LucideIcon::Heart)`.
pub struct Lucide;

impl Lucide {
    pub fn icon(name: LucideIcon) -> Icon {
        Icon::new(name)
    }
}

#[cfg(test)]
mod tests;
