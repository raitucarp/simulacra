//! Minimal DOCX and ODT packages.

use super::zip_of;

const DOCX_BODY: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>
<w:p><w:pPr><w:pStyle w:val="Title"/></w:pPr><w:r><w:t>Field Notes</w:t></w:r></w:p>
<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Monday</w:t></w:r></w:p>
<w:p><w:r><w:t xml:space="preserve">Rain all day. </w:t></w:r><w:r><w:rPr><w:b/></w:rPr><w:t>Wrote &amp; read.</w:t></w:r></w:p>
<w:p><w:pPr><w:numPr><w:ilvl w:val="0"/></w:numPr></w:pPr><w:r><w:t>Buy ink</w:t></w:r></w:p>
<w:p><w:pPr><w:pStyle w:val="Quote"/></w:pPr><w:r><w:t>Line one</w:t><w:br/><w:t>Line two</w:t></w:r></w:p>
<w:p/>
<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Tuesday</w:t></w:r></w:p>
<w:p><w:r><w:t>Sun.</w:t><w:tab/><w:t>Walked.</w:t></w:r></w:p>
</w:body></w:document>"#;

const DOCX_CORE: &str = r#"<cp:coreProperties xmlns:cp="x" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/"><dc:title>Field Notes</dc:title><dc:creator>Rosalind Franklin</dc:creator><dc:language>en-US</dc:language><dcterms:created>2024-05-01T10:00:00Z</dcterms:created></cp:coreProperties>"#;

/// A DOCX with a title, two headed days, a list item and a quote.
pub fn docx() -> Vec<u8> {
    zip_of(&[
        ("[Content_Types].xml", "<Types/>"),
        ("word/document.xml", DOCX_BODY),
        ("docProps/core.xml", DOCX_CORE),
    ])
}

const ODT_CONTENT: &str = r#"<office:document-content xmlns:office="o" xmlns:text="t"><office:body><office:text>
<text:h text:outline-level="1">Spring</text:h>
<text:p>Blossoms<text:s text:c="3"/>everywhere.<text:line-break/>Bees too.</text:p>
<text:list><text:list-item><text:p>Plant beans</text:p></text:list-item></text:list>
<text:h text:outline-level="1">Summer</text:h><text:p>Heat.</text:p>
</office:text></office:body></office:document-content>"#;

const ODT_META: &str = r#"<office:document-meta xmlns:office="o" xmlns:meta="m" xmlns:dc="d"><office:meta><dc:title>Seasons</dc:title><meta:initial-creator>Gregor Mendel</meta:initial-creator><dc:language>de</dc:language></office:meta></office:document-meta>"#;

/// An ODT with two headed seasons and a list.
pub fn odt() -> Vec<u8> {
    zip_of(&[
        ("mimetype", "application/vnd.oasis.opendocument.text"),
        ("content.xml", ODT_CONTENT),
        ("meta.xml", ODT_META),
    ])
}
