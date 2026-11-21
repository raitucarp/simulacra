//! Parsing `emoji-test.txt` (UTS #51): `code points ; status # glyph E<version> name` lines
//! under `# group:` and `# subgroup:` headers.

use super::{EmojiData, EmojiRecord};
use crate::domain::emoji::{EmojiGroup, SkinTone};

const VERSION_HEADER: &str = "# Version:";
const GROUP_HEADER: &str = "# group:";
const SUBGROUP_HEADER: &str = "# subgroup:";
const FULLY_QUALIFIED: &str = "fully-qualified";

/// Why `emoji-test.txt` could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// No `# Version:` header.
    MissingVersion,
    /// A data line that does not follow the format (1-based line number).
    BadLine(usize),
    /// A group name the table does not know.
    UnknownGroup(String),
}

/// Reads every fully-qualified emoji outside the Component group, folding the single
/// skin-tone variants into their base emoji.
pub fn parse_emoji_test(text: &str) -> Result<EmojiData, ParseError> {
    let mut version = None;
    let (mut group, mut subgroup) = (None::<usize>, String::new());
    let mut emoji: Vec<EmojiRecord> = Vec::new();
    for (number, line) in text.lines().enumerate() {
        if let Some(rest) = line.strip_prefix(VERSION_HEADER) {
            version = Some(rest.trim().to_string());
        } else if let Some(name) = line.strip_prefix(GROUP_HEADER) {
            group = group_index(name.trim())?;
        } else if let Some(name) = line.strip_prefix(SUBGROUP_HEADER) {
            subgroup = name.trim().to_string();
        } else if let (false, Some(group)) = (line.starts_with('#') || line.trim().is_empty(), group) {
            let Some((glyph, since, name)) = data_line(line).ok_or(ParseError::BadLine(number + 1))? else { continue };
            match variant(&name) {
                Variant::Toned(base, tone) => attach_tone(&mut emoji, &base, tone, glyph),
                Variant::Mixed => {}
                Variant::Base => emoji.push(EmojiRecord { glyph, name, group, subgroup: subgroup.clone(), since, tones: Vec::new() }),
            }
        }
    }
    emoji.iter_mut().filter(|e| e.tones.len() != SkinTone::ALL.len()).for_each(|e| e.tones.clear());
    Ok(EmojiData { version: version.ok_or(ParseError::MissingVersion)?, emoji })
}

/// The group's index in [`EmojiGroup::ALL`]; `None` for the Component group (skipped).
fn group_index(name: &str) -> Result<Option<usize>, ParseError> {
    if name == EmojiGroup::COMPONENT {
        return Ok(None);
    }
    EmojiGroup::ALL.iter().position(|group| group.unicode_name() == name).map(Some).ok_or_else(|| ParseError::UnknownGroup(name.to_string()))
}

/// `(glyph, since, name)` of a fully-qualified line, `Some(None)` for another status, `None`
/// when the line is malformed.
fn data_line(line: &str) -> Option<Option<(String, (u8, u8), String)>> {
    let (fields, comment) = line.split_once('#')?;
    let (points, status) = fields.split_once(';')?;
    if status.trim() != FULLY_QUALIFIED {
        return Some(None);
    }
    let glyph: Option<String> = points.split_whitespace().map(|hex| u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)).collect();
    let rest = comment.trim_start().split_once(' ')?.1;
    let (version, name) = rest.strip_prefix('E')?.split_once(' ')?;
    let (major, minor) = version.split_once('.')?;
    Some(Some((glyph?, (major.parse().ok()?, minor.parse().ok()?), name.trim().to_string())))
}

/// What a fully-qualified emoji is, read from its name.
enum Variant {
    /// An emoji of its own (`waving hand`, `man: red hair`).
    Base,
    /// One skin tone of a base emoji (`waving hand: light skin tone`, `man: light skin tone,
    /// red hair`), by the base's name.
    Toned(String, SkinTone),
    /// A sequence mixing skin tones (left out).
    Mixed,
}

fn variant(name: &str) -> Variant {
    let Some((head, tail)) = name.split_once(": ") else { return Variant::Base };
    let parts: Vec<&str> = tail.split(", ").collect();
    let tones: Vec<SkinTone> = parts.iter().filter_map(|part| SkinTone::from_unicode_name(part)).collect();
    let Some(&tone) = tones.first() else { return Variant::Base };
    if tones.iter().any(|t| *t != tone) {
        return Variant::Mixed;
    }
    let rest: Vec<&str> = parts.into_iter().filter(|part| SkinTone::from_unicode_name(part).is_none()).collect();
    let base = if rest.is_empty() { head.to_string() } else { format!("{head}: {}", rest.join(", ")) };
    Variant::Toned(base, tone)
}

/// Records `glyph` as `tone`'s variant of the emoji named `base` (variants follow their base).
fn attach_tone(emoji: &mut [EmojiRecord], base: &str, tone: SkinTone, glyph: String) {
    if let Some(record) = emoji.iter_mut().rev().find(|record| record.name == base) {
        if record.tones.len() == tone.index() {
            record.tones.push(glyph);
        }
    }
}
