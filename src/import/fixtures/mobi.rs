//! Hand-written PalmDB/MOBI books: record 0 with PalmDOC, MOBI and EXTH headers, then
//! PalmDOC-compressed text records with optional trailing entries.

const RECORD_SIZE: usize = 4096;
const MOBI_HEADER_LENGTH: u32 = 0xE8;
const PALMDOC_HEADER: usize = 16;
const MAX_DISTANCE: usize = 2047;
const MAX_MATCH: usize = 10;
const MIN_MATCH: usize = 3;

/// What to put into a synthetic book.
#[derive(Debug, Clone)]
pub struct MobiOptions {
    pub title: &'static str,
    pub author: &'static str,
    pub language: &'static str,
    /// PalmDOC header compression code (1 none, 2 PalmDOC, 17480 HUFF/CDIC).
    pub compression: u16,
    /// Appends a multibyte byte and one trailing entry to every text record.
    pub trailing_entries: bool,
}

impl Default for MobiOptions {
    fn default() -> Self {
        Self { title: "Synthetic Mobi", author: "Mary Shelley", language: "en", compression: 2, trailing_entries: true }
    }
}

fn put_u16(bytes: &mut [u8], at: usize, value: u16) {
    bytes[at..at + 2].copy_from_slice(&value.to_be_bytes());
}

fn put_u32(bytes: &mut [u8], at: usize, value: u32) {
    bytes[at..at + 4].copy_from_slice(&value.to_be_bytes());
}

/// A greedy PalmDOC encoder exercising every code: literals, literal runs, space pairs and
/// back references.
pub fn palmdoc_compress(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < input.len() {
        let window = at.saturating_sub(MAX_DISTANCE)..at;
        let best = window
            .filter_map(|start| {
                let length = (0..MAX_MATCH).take_while(|&k| at + k < input.len() && input[start + k] == input[at + k]).count();
                (length >= MIN_MATCH).then_some((at - start, length))
            })
            .max_by_key(|&(_, length)| length);
        if let Some((distance, length)) = best {
            let pair = 0x8000 | ((distance as u16) << 3) | (length - MIN_MATCH) as u16;
            out.extend_from_slice(&pair.to_be_bytes());
            at += length;
        } else if input[at] == b' ' && input.get(at + 1).is_some_and(|&n| (0x40..=0x7F).contains(&n)) {
            out.push(input[at + 1] ^ 0x80);
            at += 2;
        } else if matches!(input[at], 0x00 | 0x09..=0x7F) {
            out.push(input[at]);
            at += 1;
        } else {
            let run = (at..input.len()).take_while(|&i| input[i] >= 0x80).take(8).count().max(1);
            out.push(run as u8);
            out.extend_from_slice(&input[at..at + run]);
            at += run;
        }
    }
    out
}

fn exth(options: &MobiOptions) -> Vec<u8> {
    let entries = [(100u32, options.author), (524, options.language), (503, options.title)];
    let mut body = Vec::new();
    for (kind, text) in entries {
        body.extend_from_slice(&kind.to_be_bytes());
        body.extend_from_slice(&((text.len() + 8) as u32).to_be_bytes());
        body.extend_from_slice(text.as_bytes());
    }
    let mut block = b"EXTH".to_vec();
    block.extend_from_slice(&((body.len() + 12) as u32).to_be_bytes());
    block.extend_from_slice(&(entries.len() as u32).to_be_bytes());
    block.extend(body);
    block
}

fn record0(text_length: usize, records: usize, options: &MobiOptions) -> Vec<u8> {
    let mut header = vec![0u8; PALMDOC_HEADER + MOBI_HEADER_LENGTH as usize];
    put_u16(&mut header, 0, options.compression);
    put_u32(&mut header, 4, text_length as u32);
    put_u16(&mut header, 8, records as u16);
    put_u16(&mut header, 10, RECORD_SIZE as u16);
    header[16..20].copy_from_slice(b"MOBI");
    put_u32(&mut header, 20, MOBI_HEADER_LENGTH);
    put_u32(&mut header, 28, 65001);
    put_u32(&mut header, 36, 6);
    put_u32(&mut header, 128, 0x40);
    put_u16(&mut header, 242, if options.trailing_entries { 0b11 } else { 0 });
    header.extend(exth(options));
    let name_offset = header.len() as u32;
    put_u32(&mut header, 84, name_offset);
    put_u32(&mut header, 88, options.title.len() as u32);
    header.extend_from_slice(options.title.as_bytes());
    header
}

/// A PalmDB book whose text is `html`.
pub fn mobi_book(html: &str, options: &MobiOptions) -> Vec<u8> {
    let chunks: Vec<&[u8]> = html.as_bytes().chunks(RECORD_SIZE).collect();
    let mut records = vec![record0(html.len(), chunks.len(), options)];
    for chunk in &chunks {
        let mut record = if options.compression == 2 { palmdoc_compress(chunk) } else { chunk.to_vec() };
        if options.trailing_entries {
            record.extend_from_slice(&[0xEE, 0x01, b'T', b'B', 0x83]);
        }
        records.push(record);
    }
    palm_database(options.title, b"BOOKMOBI", &records)
}

fn palm_database(name: &str, kind: &[u8; 8], records: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = vec![0u8; 78 + records.len() * 8 + 2];
    bytes[..name.len()].copy_from_slice(name.as_bytes());
    bytes[60..68].copy_from_slice(kind);
    put_u16(&mut bytes, 76, records.len() as u16);
    let mut offset = bytes.len();
    for (index, record) in records.iter().enumerate() {
        put_u32(&mut bytes, 78 + index * 8, offset as u32);
        offset += record.len();
    }
    records.iter().for_each(|record| bytes.extend_from_slice(record));
    bytes
}
