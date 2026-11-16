//! Pure Enneagram logic behind the wheel: a type-with-wing splits into its core number and
//! wing, the wing must be one of the two neighbours of the core on the circle, and the wheel
//! places the nine points clockwise with 9 at the top.

use crate::domain::EnneagramType;
use std::f32::consts::TAU;

/// Points on the Enneagram circle.
pub const POINTS: u8 = 9;
/// Separates core and wing in a type code (`"5w4"`).
const WING_SEPARATOR: char = 'w';
/// Decimal radix of the digits in a type code.
const RADIX: u32 = 10;
/// The wheel starts at the top (a quarter turn before 3 o'clock).
const TOP: f32 = -TAU / 4.0;

/// The inner triangle of the Enneagram figure, as a closed path of points.
pub const TRIANGLE: [u8; 4] = [3, 6, 9, 3];
/// The inner hexad of the Enneagram figure (1-4-2-8-5-7-1), as a closed path of points.
pub const HEXAD: [u8; 7] = [1, 4, 2, 8, 5, 7, 1];

/// The digit before (`at = 0`) or after (`at = 1`) the wing separator of `ty`'s code.
fn code_digit(ty: EnneagramType, at: usize) -> u8 {
    ty.code()
        .split(WING_SEPARATOR)
        .nth(at)
        .and_then(|part| part.chars().next())
        .and_then(|digit| digit.to_digit(RADIX))
        .map_or(POINTS, |digit| digit as u8)
}

/// The core type number (1..=9) of a type-with-wing.
pub fn core(ty: EnneagramType) -> u8 {
    code_digit(ty, 0)
}

/// The wing number (1..=9) of a type-with-wing.
pub fn wing(ty: EnneagramType) -> u8 {
    code_digit(ty, 1)
}

/// The point after `point` going clockwise (9 → 1).
pub fn clockwise(point: u8) -> u8 {
    point % POINTS + 1
}

/// The point before `point` going clockwise (1 → 9).
pub fn counter_clockwise(point: u8) -> u8 {
    point.checked_sub(1).filter(|before| *before > 0).unwrap_or(POINTS)
}

/// Whether `wing` may be the wing of `core`: one of its two neighbours on the circle.
pub fn is_valid_wing(core: u8, wing: u8) -> bool {
    wing == clockwise(core) || wing == counter_clockwise(core)
}

/// The type-with-wing variants of `core_point` with a valid wing, in display order.
pub fn variants(core_point: u8) -> Vec<EnneagramType> {
    EnneagramType::ALL.into_iter().filter(|ty| core(*ty) == core_point && is_valid_wing(core_point, wing(*ty))).collect()
}

/// The variant of `core_point` whose wing sits on the same side of the circle as `current`'s
/// wing does (so moving 5w4 to 6 gives 6w5); the first variant when none matches.
pub fn with_core(current: EnneagramType, core_point: u8) -> EnneagramType {
    let options = variants(core_point);
    let clockwise_wing = wing(current) == clockwise(core(current));
    let side = if clockwise_wing { clockwise(core_point) } else { counter_clockwise(core_point) };
    options.iter().copied().find(|ty| wing(*ty) == side).or_else(|| options.first().copied()).unwrap_or(current)
}

/// Unit offset of `point` from the wheel centre: 9 at the top, clockwise, y pointing down.
pub fn wheel_offset(point: u8) -> (f32, f32) {
    let angle = TOP + TAU * f32::from(point % POINTS) / f32::from(POINTS);
    (angle.cos(), angle.sin())
}
