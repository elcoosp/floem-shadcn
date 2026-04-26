use floem::{peniko::Color, style::BuiltinStyle, text::{FamilyOwned, LineHeightValue, FontWeight}, unit::PxPct};
use super::{CURSOR_BLINK_INTERVAL_MS, Document};

pub type Padding = (f64, f64, f64, f64);

pub fn extract_padding(style: &BuiltinStyle<'_>, layout_width: f64) -> Padding {
    let padding_left = match style.padding_left() { PxPct::Px(p) => p, PxPct::Pct(p) => (p/100.)*layout_width };
    let padding_right = match style.padding_right() { PxPct::Px(p) => p, PxPct::Pct(p) => (p/100.)*layout_width };
    let padding_top = match style.padding_top() { PxPct::Px(p) => p, PxPct::Pct(p) => (p/100.)*layout_width };
    let padding_bottom = match style.padding_bottom() { PxPct::Px(p) => p, PxPct::Pct(p) => (p/100.)*layout_width };
    (padding_top, padding_right, padding_bottom, padding_left)
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyles {
    pub text_color: Color, pub font_size: f32, pub line_height: LineHeightValue,
    pub font_weight: FontWeight, pub font_family: Vec<FamilyOwned>,
}
impl Default for TextStyles {
    fn default() -> Self { Self { text_color: Color::BLACK, font_size: 14.0, line_height: LineHeightValue::Normal(1.5), font_weight: FontWeight::NORMAL, font_family: Vec::new() } }
}
pub fn extract_text_styles(style: &BuiltinStyle<'_>) -> TextStyles {
    TextStyles {
        text_color: style.color().unwrap_or(Color::BLACK), font_size: style.font_size().unwrap_or(14.0),
        line_height: style.line_height().unwrap_or(LineHeightValue::Normal(1.5)),
        font_weight: style.font_weight().unwrap_or(FontWeight::NORMAL),
        font_family: style.font_family().map(|f| vec![FamilyOwned::Name(f)]).unwrap_or_default(),
    }
}
pub fn apply_styles_to_document(doc: &Document, styles: &TextStyles) {
    doc.set_text_color(styles.text_color); doc.set_font_size(styles.font_size);
    doc.set_line_height(styles.line_height); doc.set_font_weight(styles.font_weight);
    doc.set_font_family(styles.font_family.clone());
}
pub fn is_cursor_visible(elapsed_ms: u128) -> bool { (elapsed_ms / CURSOR_BLINK_INTERVAL_MS as u128).is_multiple_of(2) }
pub fn get_glyph_dimensions(glyph_top: f64, glyph_bottom: f64, default_top: f64, default_height: f64) -> (f64, f64) {
    if glyph_bottom > glyph_top { (glyph_top, glyph_bottom - glyph_top) } else { (default_top, default_height) }
}
