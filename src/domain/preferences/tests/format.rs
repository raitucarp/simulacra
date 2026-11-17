use crate::domain::preferences::{group_thousands, NumberFormat};
use crate::i18n::Language;

#[test]
fn thousands_are_grouped_with_the_chosen_separator() {
    assert_eq!(group_thousands(0, ','), "0");
    assert_eq!(group_thousands(999, ','), "999");
    assert_eq!(group_thousands(1000, ','), "1,000");
    assert_eq!(group_thousands(12_345, '.'), "12.345");
    assert_eq!(group_thousands(5_000_000, ','), "5,000,000");
    assert_eq!(group_thousands(-1_234_567, ','), "-1,234,567");
}

#[test]
fn language_format_follows_the_interface_language() {
    assert_eq!(NumberFormat::Language.thousands_separator(Language::English), ',');
    assert_eq!(NumberFormat::Language.thousands_separator(Language::German), '.');
    assert_eq!(NumberFormat::Language.thousands_separator(Language::French), '\u{202f}');
    assert_eq!(NumberFormat::CommaPeriod.thousands_separator(Language::German), ',');
    assert_eq!(NumberFormat::PeriodComma.thousands_separator(Language::English), '.');
}
