//! Dedicated frameless window title bar organism.
//! Features 100% Lucide SVG icons and decoupled drag/interactive zones.

use crate::domain::Network;
use crate::i18n::Language;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::{div, App, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window, WindowControlArea};
use rust_i18n::t;

pub type TitleBarAction = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Atomic Window Titlebar component.
#[derive(IntoElement)]
pub struct AppTitleBar {
    active_network: Option<Network>,
    current_language: Language,
    theme: ThemeTokens,
    on_toggle_left_sidebar: TitleBarAction,
    on_toggle_right_sidebar: TitleBarAction,
    on_toggle_lang: TitleBarAction,
    on_toggle_settings: TitleBarAction,
}

impl AppTitleBar {
    /// Creates a new AppTitleBar with sidebar toggles and Lucide icon controls.
    pub fn new(
        active_network: Option<Network>,
        current_language: Language,
        theme: ThemeTokens,
        on_toggle_left_sidebar: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_right_sidebar: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_lang: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_settings: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            active_network,
            current_language,
            theme,
            on_toggle_left_sidebar: Box::new(on_toggle_left_sidebar),
            on_toggle_right_sidebar: Box::new(on_toggle_right_sidebar),
            on_toggle_lang: Box::new(on_toggle_lang),
            on_toggle_settings: Box::new(on_toggle_settings),
        }
    }
}

impl RenderOnce for AppTitleBar {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active_net_name = self
            .active_network
            .map(|n| n.name)
            .unwrap_or_else(|| t!("feed.general_timeline").to_string());

        let on_toggle_left = self.on_toggle_left_sidebar;
        let on_toggle_right = self.on_toggle_right_sidebar;
        let on_toggle_lang = self.on_toggle_lang;
        let on_toggle_settings = self.on_toggle_settings;
        let is_maximized = window.is_maximized();

        let win_hover_bg = self.theme.bg_window_control_hover;
        let win_close_hover_bg = self.theme.bg_window_control_close_hover;
        let text_white = self.theme.text_white;
        let win_control_width = self.theme.win_control_width;
        let win_control_height = self.theme.win_control_height;

        div()
            .w_full()
            .h(self.theme.titlebar_height)
            .bg(self.theme.bg_card)
            .border_b_1()
            .border_color(self.theme.border)
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            // 1. Left Section: Sidebar Toggle + Brand & Breadcrumbs
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("toggle-left-sidebar-btn")
                            .ghost()
                            .compact()
                            .icon(Icon::new(LucideIcon::PanelLeft))
                            .on_click(move |_, window, cx| {
                                on_toggle_left(window, cx);
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(self.theme.text_primary)
                                    .child(t!("app.title").to_string()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(self.theme.text_muted)
                                    .child(Icon::new(LucideIcon::ChevronRight)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_medium()
                                    .text_color(self.theme.accent_agent)
                                    .child(active_net_name),
                            ),
                    ),
            )
            // 2. Center Draggable Area
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .window_control_area(WindowControlArea::Drag),
            )
            // 3. Right Section: PanelRight Toggle + Language + Settings + Window Controls
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1p5()
                    // Toggle Right Sidebar Inspector
                    .child(
                        Button::new("toggle-right-sidebar-btn")
                            .ghost()
                            .compact()
                            .icon(Icon::new(LucideIcon::PanelRight))
                            .on_click(move |_, window, cx| {
                                on_toggle_right(window, cx);
                            }),
                    )
                    // Language Dropdown Trigger
                    .child(
                        Button::new("lang-menu-btn")
                            .ghost()
                            .compact()
                            .icon(Icon::new(LucideIcon::Globe))
                            .label(self.current_language.code().to_uppercase())
                            .on_click(move |_, window, cx| {
                                on_toggle_lang(window, cx);
                            }),
                    )
                    // Global Settings Trigger
                    .child(
                        Button::new("settings-btn")
                            .ghost()
                            .compact()
                            .icon(Icon::new(LucideIcon::Settings))
                            .on_click(move |_, window, cx| {
                                on_toggle_settings(window, cx);
                            }),
                    )
                    // Window Controls (Min, Max, Close)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .pl_2()
                            // Minimize
                            .child(
                                div()
                                    .id("win-control-min")
                                    .w(win_control_width)
                                    .h(win_control_height)
                                    .rounded_sm()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(self.theme.text_secondary)
                                    .hover(move |s| s.bg(win_hover_bg).text_color(text_white))
                                    .window_control_area(WindowControlArea::Min)
                                    .child(Icon::new(LucideIcon::Minus)),
                            )
                            // Maximize / Restore
                            .child(
                                div()
                                    .id("win-control-max")
                                    .w(win_control_width)
                                    .h(win_control_height)
                                    .rounded_sm()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(self.theme.text_secondary)
                                    .hover(move |s| s.bg(win_hover_bg).text_color(text_white))
                                    .window_control_area(WindowControlArea::Max)
                                    .child(if is_maximized {
                                        Icon::new(LucideIcon::Minimize2)
                                    } else {
                                        Icon::new(LucideIcon::Maximize2)
                                    }),
                            )
                            // Close
                            .child(
                                div()
                                    .id("win-control-close")
                                    .w(win_control_width)
                                    .h(win_control_height)
                                    .rounded_sm()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(self.theme.text_secondary)
                                    .hover(move |s| s.bg(win_close_hover_bg).text_color(text_white))
                                    .window_control_area(WindowControlArea::Close)
                                    .child(Icon::new(LucideIcon::X)),
                            ),
                    ),
            )
    }
}
