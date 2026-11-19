//! ISO 8601 durations as pages state them (`PT4M13S`, `PT1H2M`, `P0DT0H3M1S`).

const PERIOD_MARK: char = 'P';
const TIME_MARK: char = 'T';
const SECONDS_PER_UNIT: [(char, u32); 4] = [('D', 86_400), ('H', 3_600), ('M', 60), ('S', 1)];

/// Whole seconds of an ISO 8601 duration; `None` when it is not one. Fractions of a second
/// are dropped; a date-part `M` (months) is not a duration a video has, so it is rejected.
pub fn parse_iso_duration(raw: &str) -> Option<u32> {
    let body = raw.trim().strip_prefix(PERIOD_MARK)?;
    let (date_part, time_part) = body.split_once(TIME_MARK).unwrap_or((body, ""));
    let days = units(date_part, &['D'])?;
    let time = units(time_part, &['H', 'M', 'S'])?;
    let total = days.checked_add(time)?;
    (!date_part.is_empty() || !time_part.is_empty()).then_some(total)
}

/// Seconds of `part` (`4M13S`), accepting only the `allowed` unit letters.
fn units(part: &str, allowed: &[char]) -> Option<u32> {
    let mut total: u32 = 0;
    let mut number = String::new();
    for c in part.chars() {
        if c.is_ascii_digit() || c == '.' {
            number.push(c);
            continue;
        }
        let unit = c.to_ascii_uppercase();
        let per_unit = SECONDS_PER_UNIT.iter().find(|(letter, _)| *letter == unit && allowed.contains(letter))?.1;
        let whole: u32 = number.split('.').next()?.parse().ok()?;
        total = total.checked_add(whole.checked_mul(per_unit)?)?;
        number.clear();
    }
    number.is_empty().then_some(total)
}
