//! W7: how a profile's Posts tab is shown (its view, the feelings filter, the pages shown and
//! the calendar month), and what the tab can ask the shell to do.

/// W7: how the Posts tab is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfilePosts {
    pub view: crate::domain::views::PostView,
    /// M4: only the feelings the profile posted.
    pub feelings_only: bool,
    /// Pages shown (at least one).
    pub pages: usize,
    /// The calendar month chosen (`None`: the newest post's).
    pub month: Option<crate::domain::views::calendar::YearMonth>,
}

impl Default for ProfilePosts {
    fn default() -> Self {
        Self { view: crate::domain::views::PostView::Cards, feelings_only: false, pages: 1, month: None }
    }
}

/// W7: something done in the Posts tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfilePostsEvent {
    Pick(crate::domain::views::PostView),
    ToggleFeelings,
    LoadMore,
    Month(crate::domain::views::calendar::YearMonth),
}

impl ProfilePosts {
    /// The tab after `event`: a new view or filter starts on the first page.
    pub fn after(self, event: ProfilePostsEvent) -> Self {
        match event {
            ProfilePostsEvent::Pick(view) => Self { view, pages: 1, ..self },
            ProfilePostsEvent::ToggleFeelings => Self { feelings_only: !self.feelings_only, pages: 1, ..self },
            ProfilePostsEvent::LoadMore => Self { pages: self.pages.saturating_add(1), ..self },
            ProfilePostsEvent::Month(month) => Self { month: Some(month), ..self },
        }
    }
}
