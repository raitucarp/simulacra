//! Fitting a split's panes into a container too narrow for their minimums. Pure arithmetic on
//! logical pixels, so the rule is unit-tested without a window.

/// Size bounds of one pane along the split axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaneBounds {
    pub min: f32,
    pub max: f32,
    /// Size it opens at (the flex pane: its minimum).
    pub want: f32,
    /// Takes the remaining space instead of keeping a size.
    pub flex: bool,
}

/// The bounds that fit a container `available` long. Minimums that add up to more than the
/// container scale down together; sized panes then give up the length they hold above their
/// (scaled) minimum, in proportion, so the flex pane keeps its (scaled) minimum. An unmeasured
/// container (`<= 0`) changes nothing.
pub fn fit_panes(panes: &[PaneBounds], available: f32) -> Vec<PaneBounds> {
    if !available.is_finite() || available <= 0.0 {
        return panes.to_vec();
    }
    let total_min: f32 = panes.iter().map(|pane| pane.min).sum();
    let scale = if total_min > available { available / total_min } else { 1.0 };
    let min_sum = |flex: bool| panes.iter().filter(|pane| pane.flex == flex).map(|pane| pane.min * scale).sum::<f32>();
    let extra: f32 = panes.iter().filter(|pane| !pane.flex).map(|pane| (pane.want - pane.min * scale).max(0.0)).sum();
    let room = (available - min_sum(true) - min_sum(false)).max(0.0);
    let give = if extra > room && extra > 0.0 { room / extra } else { 1.0 };
    panes.iter().map(|pane| fit_one(pane, scale, give)).collect()
}

/// One pane with its minimum scaled by `scale` and (sized panes) the length it holds above
/// that minimum by `give`.
fn fit_one(pane: &PaneBounds, scale: f32, give: f32) -> PaneBounds {
    let min = pane.min * scale;
    if pane.flex {
        return PaneBounds { min, ..*pane };
    }
    let want = min + (pane.want - min).max(0.0) * give;
    let max = if give < 1.0 { want.min(pane.max).max(min) } else { pane.max };
    PaneBounds { min, max, want, flex: false }
}
