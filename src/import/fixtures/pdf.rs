//! A hand-written two-page PDF with a standard font, an `/Info` title and exact xref offsets.

fn content_stream(lines: &[&str]) -> String {
    let shown: String = lines.iter().map(|line| format!("({line}) Tj 0 -14 Td ")).collect();
    format!("BT /F1 12 Tf 72 720 Td {shown}ET")
}

/// The PDF objects (without numbers) for pages showing `pages` lines each.
fn objects(pages: &[&[&str]]) -> Vec<String> {
    let page_ids: Vec<String> = (0..pages.len()).map(|i| format!("{} 0 R", 5 + i * 2)).collect();
    let mut objects = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        format!("<< /Type /Pages /Kids [{}] /Count {} >>", page_ids.join(" "), pages.len()),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".to_string(),
        "<< /Title (A Tiny PDF) /Author (Grace Hopper) /CreationDate (D:19520101000000Z) >>".to_string(),
    ];
    for (index, lines) in pages.iter().enumerate() {
        let stream = content_stream(lines);
        objects.push(format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>",
            6 + index * 2
        ));
        objects.push(format!("<< /Length {} >>\nstream\n{stream}\nendstream", stream.len()));
    }
    objects
}

/// A PDF whose pages show the given lines.
pub fn tiny_pdf(pages: &[&[&str]]) -> Vec<u8> {
    let mut out = String::from("%PDF-1.4\n");
    let mut offsets = Vec::new();
    for (index, object) in objects(pages).iter().enumerate() {
        offsets.push(out.len());
        out.push_str(&format!("{} 0 obj\n{object}\nendobj\n", index + 1));
    }
    let xref_at = out.len();
    out.push_str(&format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1));
    offsets.iter().for_each(|offset| out.push_str(&format!("{offset:010} 00000 n \n")));
    out.push_str(&format!("trailer\n<< /Size {} /Root 1 0 R /Info 4 0 R >>\nstartxref\n{xref_at}\n%%EOF\n", offsets.len() + 1));
    out.into_bytes()
}
