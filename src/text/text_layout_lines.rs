use floem::kurbo::Point;
pub struct TextLayoutLines {}
impl TextLayoutLines {
    pub fn default_glyph_top(&self) -> f64 { 0.0 }
    pub fn default_glyph_height(&self) -> f64 { 14.0 }
    pub fn point_of_offset(&self, _: usize) -> Point { Point::ZERO }
    pub fn utf8_len(&self) -> usize { 0 }
    pub fn visual_lines(&self, _: std::ops::Range<usize>) -> impl Iterator<Item = (usize, usize, f64, f64)> { std::iter::empty() }
}
