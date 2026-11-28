//! Arabic letter joining for a renderer that draws text left to right without shaping it in
//! context: each letter becomes its presentation form (isolated, final, initial or medial)
//! from its neighbours in logical order, and lam + alef become their ligature. The forms are
//! the Unicode "Arabic Presentation Forms-B" code points, which a font draws as they are.

/// Isolated, final, initial and medial forms of a letter; 0 where the letter has no such form
/// (right-joining letters connect only to the letter before them).
const FORMS: [(char, [u32; 4]); 36] = [
    ('\u{0621}', [0xFE80, 0, 0, 0]),
    ('\u{0622}', [0xFE81, 0xFE82, 0, 0]),
    ('\u{0623}', [0xFE83, 0xFE84, 0, 0]),
    ('\u{0624}', [0xFE85, 0xFE86, 0, 0]),
    ('\u{0625}', [0xFE87, 0xFE88, 0, 0]),
    ('\u{0626}', [0xFE89, 0xFE8A, 0xFE8B, 0xFE8C]),
    ('\u{0627}', [0xFE8D, 0xFE8E, 0, 0]),
    ('\u{0628}', [0xFE8F, 0xFE90, 0xFE91, 0xFE92]),
    ('\u{0629}', [0xFE93, 0xFE94, 0, 0]),
    ('\u{062A}', [0xFE95, 0xFE96, 0xFE97, 0xFE98]),
    ('\u{062B}', [0xFE99, 0xFE9A, 0xFE9B, 0xFE9C]),
    ('\u{062C}', [0xFE9D, 0xFE9E, 0xFE9F, 0xFEA0]),
    ('\u{062D}', [0xFEA1, 0xFEA2, 0xFEA3, 0xFEA4]),
    ('\u{062E}', [0xFEA5, 0xFEA6, 0xFEA7, 0xFEA8]),
    ('\u{062F}', [0xFEA9, 0xFEAA, 0, 0]),
    ('\u{0630}', [0xFEAB, 0xFEAC, 0, 0]),
    ('\u{0631}', [0xFEAD, 0xFEAE, 0, 0]),
    ('\u{0632}', [0xFEAF, 0xFEB0, 0, 0]),
    ('\u{0633}', [0xFEB1, 0xFEB2, 0xFEB3, 0xFEB4]),
    ('\u{0634}', [0xFEB5, 0xFEB6, 0xFEB7, 0xFEB8]),
    ('\u{0635}', [0xFEB9, 0xFEBA, 0xFEBB, 0xFEBC]),
    ('\u{0636}', [0xFEBD, 0xFEBE, 0xFEBF, 0xFEC0]),
    ('\u{0637}', [0xFEC1, 0xFEC2, 0xFEC3, 0xFEC4]),
    ('\u{0638}', [0xFEC5, 0xFEC6, 0xFEC7, 0xFEC8]),
    ('\u{0639}', [0xFEC9, 0xFECA, 0xFECB, 0xFECC]),
    ('\u{063A}', [0xFECD, 0xFECE, 0xFECF, 0xFED0]),
    ('\u{0641}', [0xFED1, 0xFED2, 0xFED3, 0xFED4]),
    ('\u{0642}', [0xFED5, 0xFED6, 0xFED7, 0xFED8]),
    ('\u{0643}', [0xFED9, 0xFEDA, 0xFEDB, 0xFEDC]),
    ('\u{0644}', [0xFEDD, 0xFEDE, 0xFEDF, 0xFEE0]),
    ('\u{0645}', [0xFEE1, 0xFEE2, 0xFEE3, 0xFEE4]),
    ('\u{0646}', [0xFEE5, 0xFEE6, 0xFEE7, 0xFEE8]),
    ('\u{0647}', [0xFEE9, 0xFEEA, 0xFEEB, 0xFEEC]),
    ('\u{0648}', [0xFEED, 0xFEEE, 0, 0]),
    ('\u{0649}', [0xFEEF, 0xFEF0, 0, 0]),
    ('\u{064A}', [0xFEF1, 0xFEF2, 0xFEF3, 0xFEF4]),
];
/// Lam followed by one of these alefs is drawn as one ligature: (alef, isolated, final).
const LAM_ALEF: [(char, u32, u32); 4] = [('\u{0622}', 0xFEF5, 0xFEF6), ('\u{0623}', 0xFEF7, 0xFEF8), ('\u{0625}', 0xFEF9, 0xFEFA), ('\u{0627}', 0xFEFB, 0xFEFC)];
const LAM: char = '\u{0644}';
/// The kashida stretches a joined word; it joins on both sides and has no forms of its own.
const TATWEEL: char = '\u{0640}';

/// A harakah or other mark that sits on a letter and does not break its joining.
pub fn is_mark(c: char) -> bool {
    ('\u{064B}'..='\u{065F}').contains(&c) || c == '\u{0670}'
}

fn forms(c: char) -> Option<[u32; 4]> {
    FORMS.iter().find(|(letter, _)| *letter == c).map(|(_, forms)| *forms)
}

/// The letter connects to the letter after it (in reading order).
fn joins_forward(c: char) -> bool {
    c == TATWEEL || forms(c).is_some_and(|forms| forms[2] != 0)
}

/// The letter connects to the letter before it.
fn joins_back(c: char) -> bool {
    c == TATWEEL || forms(c).is_some_and(|forms| forms[1] != 0)
}

fn neighbour(chars: &[char], at: usize, forward: bool) -> Option<usize> {
    let mut i = at;
    loop {
        i = if forward { i.checked_add(1).filter(|&i| i < chars.len())? } else { i.checked_sub(1)? };
        if !is_mark(chars[i]) {
            return Some(i);
        }
    }
}

fn form_char(code: u32, fallback: char) -> char {
    char::from_u32(code).filter(|_| code != 0).unwrap_or(fallback)
}

/// `chars` (logical order) with every Arabic letter in its joined presentation form.
pub fn shape(chars: &[char]) -> Vec<char> {
    let mut out = Vec::with_capacity(chars.len());
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let Some(letter) = forms(c) else {
            out.push(c);
            i += 1;
            continue;
        };
        let before = neighbour(chars, i, false).is_some_and(|j| joins_forward(chars[j])) && joins_back(c);
        let next = neighbour(chars, i, true);
        if let Some((k, (_, isolated, final_form))) = next.filter(|_| c == LAM).and_then(|k| LAM_ALEF.iter().find(|(alef, ..)| *alef == chars[k]).map(|ligature| (k, *ligature))) {
            out.push(form_char(if before { final_form } else { isolated }, c));
            out.extend(&chars[i + 1..k]);
            i = k + 1;
            continue;
        }
        let after = joins_forward(c) && next.is_some_and(|k| joins_back(chars[k]));
        let form = match (before, after) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        };
        out.push(form_char(letter[form], c));
        i += 1;
    }
    out
}
