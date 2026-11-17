//! Name lists as each style writes them.

use crate::domain::document::record::PersonName;

/// Joins `names` as "A, B, and C" (`conjunction` = "and" or "&"; `serial` adds the comma
/// before it when there are three or more).
pub fn join(names: &[String], conjunction: &str, serial: bool) -> String {
    match names {
        [] => String::new(),
        [only] => only.clone(),
        [first, second] => format!("{first} {conjunction} {second}"),
        [init @ .., last] => {
            let comma = if serial { "," } else { "" };
            format!("{}{comma} {conjunction} {last}", init.join(", "))
        }
    }
}

/// "Family, I." for every name ("Spinoza, B.").
pub fn inverted_initials(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .map(|raw| {
            let name = PersonName::parse(raw);
            match name.initials().as_str() {
                "" => name.family,
                initials => format!("{}, {initials}", name.family),
            }
        })
        .collect()
}

/// "I. Family" for every name ("B. Spinoza").
pub fn initials_first(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .map(|raw| {
            let name = PersonName::parse(raw);
            match name.initials().as_str() {
                "" => name.family,
                initials => format!("{initials} {}", name.family),
            }
        })
        .collect()
}

/// The first name inverted ("Spinoza, Baruch"), the rest in natural order.
pub fn first_inverted(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .enumerate()
        .map(|(at, raw)| {
            let name = PersonName::parse(raw);
            if at == 0 {
                name.inverted()
            } else {
                name.natural()
            }
        })
        .collect()
}

/// Every name in natural order ("R. H. M. Elwes").
pub fn natural(names: &[&str]) -> Vec<String> {
    names.iter().map(|raw| PersonName::parse(raw).natural()).collect()
}
