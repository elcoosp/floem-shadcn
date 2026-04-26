use floem::{
    peniko::Color,
    style::BuiltinStyle,
    text::{FamilyOwned, LineHeightValue, Weight},
    unit::PxPct,
};

use super::{CURSOR_BLINK_INTERVAL_MS, Document};

pub type Padding = (f64, f64, f64, f64);

pub fn extract_padding(style: &BuiltinStyle<'_>, layout_width: f64) -> Padding {
    let left = match style.padding_left() { PxPct::Px(p) => p, PxPct::Pct(p) => (p/100.)*layout_width };
    let right = match style.padding_right() { PxPct::Px(p) => p, PxPct::Pct(p) => (p/100.)*layout_width };
    let top = match style.padding_top() { PxPct::Px(p) => p, PxPct::Pct(p) => (p/100.)*layout_width };
    let bottom = match style.padding_bottom() { PxPct::Px(p) => p, PxPct::Pct(p) => (p/100.)*layout_width };
    (top, right, bottom, left)
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyles {
    pub text_color: Color,
    pub font_size: f32,
    pub line_height: LineHeightValue,
    pub font_weight: Weight,
    pub font_family: Vec<FamilyOwned>,
}

impl Default for TextStyles {
    fn default() -> Self {
        Self { text_color: Color::BLACK, font_size: 14.0, line_height: LineHeightValue::Normal(1.5), font_weight: Weight::NORMAL, font_family: Vec::new() }
    }
}

pub fn extract_text_styles(style: &BuiltinStyle<'_>) -> TextStyles {
    TextStyles {
        text_color: style.color().unwrap_or(Color::BLACK),
        font_size: style.font_size().unwrap_or(14.0),
        line_height: style.line_height().unwrap_or(LineHeightValue::Normal(1.5)),
        font_weight: style.font_weight().unwrap_or(Weight::NORMAL),
        font_family: style.font_family().map(|f| vec![FamilyOwned::Name(f)]).unwrap_or_default(),
    }
}

pub fn apply_styles_to_document(doc: &Document, styles: &TextStyles) {}
pub fn is_cursor_visible(elapsed_ms: u128) -> bool { (elapsed_ms / CURSOR_BLINK_INTERVAL_MS as u128) % 2 == 0 }
pub fn get_glyph_dimensions(g_top: f64, g_bottom: f64, d_top: f64, d_height: f64) -> (f64, f64) {
    if g_bottom > g_top { (g_top, g_bottom - g_top) } else { (d_top, d_height) }
}
