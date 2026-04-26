use floem::{
    kurbo::Point,
    text::{LayoutLine, LayoutRun, LineEnding, TextLayout},
};
use lapce_xi_rope::{
    Cursor, Delta, DeltaElement, Interval,
    tree::{Leaf, Node, NodeInfo, TreeBuilder, UnitAffinity, UnitConverter},
};
use std::{
    cmp::Ordering,
    ops::{Add, Range, Sub},
};

const MAX_LEAF: usize = 1024;

pub struct TextLayoutLines {
    pub(crate) tree: Node<TextLayoutLineInfo>,
    default_glyph_height: f64,
    default_glyph_top: f64,
}

impl TextLayoutLines {
    pub fn builder() -> TextLayoutLineBuilder {
        TextLayoutLineBuilder::new()
    }
    pub fn utf8_len(&self) -> usize {
        self.tree.len()
    }
    pub fn num_visual_lines(&self) -> usize {
        let info = self.tree.info();
        info.line_breaks + info.line_endings + 1
    }
    pub fn default_glyph_height(&self) -> f64 {
        self.default_glyph_height
    }
    pub fn default_glyph_top(&self) -> f64 {
        self.default_glyph_top
    }
    pub fn apply_delta(&mut self, delta: Delta<TextLayoutLineInfo>) {
        let mut b = TreeBuilder::new();
        for elem in delta.els {
            match elem {
                DeltaElement::Copy(beg, end) => b.push(self.tree.subseq(Interval::new(beg, end))),
                DeltaElement::Insert(n) => b.push(n),
            }
        }
        self.tree = b.build();
    }
    pub fn visual_line(&self, a: usize) -> usize {
        self.tree.count_unit::<usize, VlineLineConverter>(a)
    }
    pub fn actual_line(&self, v: usize) -> usize {
        self.tree.measure_unit::<usize, VlineLineConverter>(v)
    }
    pub fn vline_of_height(&self, h: f64) -> usize {
        self.tree
            .convert::<GlyphPoint, PointConverter, usize, VlineConverter>(GlyphPoint {
                x: 0.0,
                glyph_top: 0.0,
                glyph_bottom: 0.0,
                line_bottom: 0.0,
                line_top: 0.0,
                total_height: h,
            })
    }
    pub fn height_of_vline(&self, v: usize) -> f64 {
        self.tree
            .convert::<usize, VlineConverter, GlyphPoint, PointConverter>(v)
            .line_bottom
    }
    pub fn offset_of_vline(&self, v: usize) -> usize {
        self.tree
            .convert::<usize, VlineConverter, usize, BaseConverter>(v)
    }
    pub fn vline_of_offset(&self, o: usize) -> usize {
        self.tree
            .convert::<usize, BaseConverter, usize, VlineConverter>(o)
    }
    pub fn point_of_offset(&self, o: usize) -> GlyphPoint {
        self.tree
            .convert::<usize, BaseConverter, GlyphPoint, PointConverter>(o)
    }
    pub fn offset_of_point(&self, pt: Point) -> usize {
        self.tree
            .convert::<GlyphPoint, PointConverter, usize, BaseConverter>(GlyphPoint {
                x: pt.x,
                glyph_top: 0.0,
                glyph_bottom: 0.0,
                line_bottom: 0.0,
                line_top: 0.0,
                total_height: pt.y,
            })
    }
    pub fn visual_lines(&self, r: Range<usize>) -> VisualLineIter<'_> {
        let h = self.point_of_offset(r.start).line_top as f32;
        let vl = self.vline_of_offset(r.start);
        VisualLineIter {
            cursor: Cursor::new(&self.tree, r.start),
            end: r.end,
            leaf_visual_index: None,
            vline: vl,
            height: h,
        }
    }
}

#[derive(Clone)]
pub struct TextLayoutLineInfo {
    utf8_len: usize,
    line_endings: usize,
    line_breaks: usize,
    num_vlines: usize,
    max_width: f64,
    total_height: f64,
    last_glyph: GlyphPoint,
}

#[derive(Clone)]
pub struct TextLayoutLine {
    line: LayoutLine,
    line_ending: LineEnding,
    line_break: bool,
    line_height: f64,
    utf8_len: usize,
}

#[derive(Default, Clone)]
pub struct TextLayoutLineLeaf {
    utf8_len: usize,
    line_endings: usize,
    line_breaks: usize,
    visual_lines: Vec<TextLayoutLine>,
    max_width: f64,
    total_height: f64,
    last_glyph: GlyphPoint,
    default_glyph_height: f32,
    default_centering_offset: f32,
}

impl TextLayoutLineLeaf {
    fn push_layout_line(
        &mut self,
        line: &LayoutLine,
        utf8_len: usize,
        le: LineEnding,
        lb: bool,
        lh: f64,
        dgh: f32,
        dco: f32,
    ) {
        let lh = line.line_height_opt.map(|x| x as f64).unwrap_or(lh);
        self.utf8_len += utf8_len;
        if self.max_width < line.w as f64 {
            self.max_width = line.w as f64
        }
        self.default_glyph_height = dgh;
        self.default_centering_offset = dco;
        let mut gh = line.max_ascent + line.max_descent;
        let mut co = (lh as f32 - gh) / 2.0;
        if gh == 0.0 && dgh > 0.0 {
            gh = dgh;
            co = dco;
        }
        self.last_glyph = GlyphPoint {
            x: line.w as f64,
            line_top: self.total_height,
            line_bottom: self.total_height + lh,
            glyph_top: self.total_height + co as f64,
            glyph_bottom: self.total_height + (co + gh) as f64,
            total_height: self.total_height + lh,
        };
        if le != LineEnding::None || lb {
            self.last_glyph.x = 0.0;
            self.last_glyph.line_top += lh;
            self.last_glyph.line_bottom += lh;
            self.last_glyph.glyph_top += lh;
            self.last_glyph.glyph_bottom += lh;
        }
        self.total_height += lh;
        if le != LineEnding::None {
            self.line_endings += 1;
        }
        if lb {
            self.line_breaks += 1;
        }
        self.visual_lines.push(TextLayoutLine {
            line: line.to_owned(),
            line_ending: le,
            line_break: lb,
            line_height: lh,
            utf8_len,
        });
    }
}

impl NodeInfo for TextLayoutLineInfo {
    type L = TextLayoutLineLeaf;
    fn accumulate(&mut self, other: &Self) {
        self.utf8_len += other.utf8_len;
        self.line_endings += other.line_endings;
        self.line_breaks += other.line_breaks;
        self.num_vlines += other.num_vlines;
        self.max_width = self.max_width.max(other.max_width);
        self.total_height += other.total_height;
        self.last_glyph = self.last_glyph.clone() + other.last_glyph.clone();
    }
    fn compute_info(leaf: &Self::L) -> Self {
        Self {
            utf8_len: leaf.utf8_len,
            line_endings: leaf.line_endings,
            line_breaks: leaf.line_breaks,
            max_width: leaf.max_width,
            num_vlines: leaf.visual_lines.len(),
            total_height: leaf.total_height,
            last_glyph: leaf.last_glyph.clone(),
        }
    }
}

impl Leaf for TextLayoutLineLeaf {
    fn len(&self) -> usize {
        self.utf8_len
    }
    fn is_ok_child(&self) -> bool {
        true
    }
    fn push_maybe_split(&mut self, other: &Self, _: Interval) -> Option<Self> {
        let (start, end) = (0, other.visual_lines.len());
        let mut index = start;
        for line in &other.visual_lines[start..end] {
            if self.utf8_len > MAX_LEAF {
                break;
            }
            self.push_layout_line(
                &line.line,
                line.utf8_len,
                line.line_ending,
                line.line_break,
                line.line_height,
                other.default_glyph_height,
                other.default_centering_offset,
            );
            index += 1;
        }
        if index < end {
            let mut leaf = TextLayoutLineLeaf::default();
            for line in &other.visual_lines[start..end] {
                leaf.push_layout_line(
                    &line.line,
                    line.utf8_len,
                    line.line_ending,
                    line.line_break,
                    line.line_height,
                    other.default_glyph_height,
                    other.default_centering_offset,
                );
            }
            return Some(leaf);
        }
        None
    }
}

pub struct TextLayoutLineBuilder {
    builder: TreeBuilder<TextLayoutLineInfo>,
    default_glyph_height: f64,
    default_glyph_top: f64,
}

impl Default for TextLayoutLineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl TextLayoutLineBuilder {
    pub fn new() -> Self {
        Self {
            builder: TreeBuilder::new(),
            default_glyph_height: 16.0,
            default_glyph_top: 0.0,
        }
    }
    pub fn build(self) -> TextLayoutLines {
        TextLayoutLines {
            tree: self.builder.build(),
            default_glyph_height: self.default_glyph_height,
            default_glyph_top: self.default_glyph_top,
        }
    }
    pub fn set_default_from_layout(&mut self, tl: &TextLayout) {
        let metrics = tl.metrics();
        let dlh = metrics.line_height as f64;
        for bl in tl.lines() {
            let lines = bl.layout_opt().into_iter().flatten();
            for line in lines {
                let gh = (line.max_ascent + line.max_descent) as f64;
                if gh > 0.0 {
                    let lh = line.line_height_opt.map(|x| x as f64).unwrap_or(dlh);
                    let co = (lh - gh) / 2.0;
                    self.default_glyph_height = gh;
                    self.default_glyph_top = co;
                    return;
                }
            }
        }
    }
    pub fn push_text_layout(&mut self, tl: &TextLayout) {
        let metrics = tl.metrics();
        let mut cap = self.default_glyph_height > 16.0;
        for bl in tl.lines() {
            let le = bl.ending();
            let mut leaf = TextLayoutLineLeaf::default();
            let lines = bl.layout_opt().into_iter().flatten();
            let mut lines = lines.peekable();
            while let Some(line) = lines.next() {
                if !cap {
                    let gh = (line.max_ascent + line.max_descent) as f64;
                    if gh > 0.0 {
                        let lh = metrics.line_height as f64;
                        let co = (lh - gh) / 2.0;
                        self.default_glyph_height = gh;
                        self.default_glyph_top = co;
                        cap = true;
                    }
                }
                let is_last = lines.peek().is_none();
                let mut ul = if let Some(next) = lines.peek() {
                    next.glyphs
                        .first()
                        .map(|g| g.start)
                        .unwrap_or(0)
                        .saturating_sub(line.glyphs.first().map(|g| g.start).unwrap_or(0))
                } else {
                    bl.text()
                        .len()
                        .saturating_sub(line.glyphs.first().map(|g| g.start).unwrap_or(0))
                };
                if is_last {
                    ul += match le {
                        LineEnding::None => 0,
                        LineEnding::Lf => 1,
                        LineEnding::CrLf => 2,
                        LineEnding::Cr => 1,
                        _ => 1,
                    };
                }
                leaf.push_layout_line(
                    line,
                    ul,
                    if is_last { le } else { LineEnding::None },
                    !is_last,
                    metrics.line_height as f64,
                    self.default_glyph_height as f32,
                    self.default_glyph_top as f32,
                );
            }
            self.builder.push(Node::from_leaf(leaf));
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GlyphPoint {
    pub x: f64,
    pub line_top: f64,
    pub line_bottom: f64,
    pub glyph_top: f64,
    pub glyph_bottom: f64,
    total_height: f64,
}
impl Sub for GlyphPoint {
    type Output = GlyphPoint;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x,
            line_top: self.line_top - rhs.line_top,
            line_bottom: self.line_top - rhs.line_top,
            glyph_top: self.line_top - rhs.line_top,
            glyph_bottom: self.line_top - rhs.line_top,
            total_height: self.total_height - rhs.total_height,
        }
    }
}
impl Add for GlyphPoint {
    type Output = GlyphPoint;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: rhs.x,
            line_top: self.line_top + rhs.line_top,
            line_bottom: self.line_top + rhs.line_bottom,
            glyph_top: self.line_top + rhs.glyph_top,
            glyph_bottom: self.line_top + rhs.glyph_bottom,
            total_height: self.total_height + rhs.total_height,
        }
    }
}
impl PartialEq for GlyphPoint {
    fn eq(&self, o: &Self) -> bool {
        self.total_height.eq(&o.total_height) && self.x.eq(&o.x)
    }
}
impl PartialOrd for GlyphPoint {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        self.total_height.partial_cmp(&o.total_height)
    }
}

pub struct BaseConverter(());
impl UnitConverter<TextLayoutLineInfo, usize> for BaseConverter {
    fn count(_: &TextLayoutLineLeaf, m: usize) -> usize {
        m
    }
    fn measure(_: &TextLayoutLineLeaf, b: usize) -> usize {
        b
    }
    fn base(l: &TextLayoutLineInfo) -> usize {
        l.utf8_len
    }
    fn node_measured(l: &TextLayoutLineInfo) -> usize {
        l.utf8_len
    }
    fn count_affinty() -> UnitAffinity {
        UnitAffinity::After
    }
    fn measure_affinty() -> UnitAffinity {
        UnitAffinity::After
    }
}

pub struct VlineConverter(());
impl UnitConverter<TextLayoutLineInfo, usize> for VlineConverter {
    fn count(l: &TextLayoutLineLeaf, m: usize) -> usize {
        l.visual_lines[..m].iter().map(|x| x.utf8_len).sum()
    }
    fn measure(l: &TextLayoutLineLeaf, b: usize) -> usize {
        let mut base = b;
        let mut n = 0;
        for x in &l.visual_lines {
            match base.cmp(&x.utf8_len) {
                Ordering::Greater => {
                    base -= x.utf8_len;
                    n += 1;
                    continue;
                }
                Ordering::Equal => {
                    return if x.line_ending != LineEnding::None || x.line_break {
                        n + 1
                    } else {
                        n
                    };
                }
                Ordering::Less => return n,
            }
        }
        n
    }
    fn base(info: &TextLayoutLineInfo) -> usize {
        info.utf8_len
    }
    fn node_measured(info: &TextLayoutLineInfo) -> usize {
        info.num_vlines
    }
    fn count_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }
    fn measure_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }
}

pub struct PointConverter(());
impl UnitConverter<TextLayoutLineInfo, GlyphPoint> for PointConverter {
    fn count(l: &TextLayoutLineLeaf, m: GlyphPoint) -> usize {
        let mut h = m.total_height;
        let x = m.x;
        let mut off = 0;
        for line in &l.visual_lines {
            if h < line.line_height {
                let ls = off;
                for g in &line.line.glyphs {
                    if x < (g.x + g.w) as f64 {
                        return off;
                    } else {
                        off += g.end - g.start;
                    }
                }
                if line.line_ending != LineEnding::None || line.line_break {
                    return if off > ls { off - 1 } else { off };
                } else {
                    return off;
                }
            } else {
                h -= line.line_height;
                off += line.utf8_len;
            }
        }
        off
    }
    fn measure(leaf: &TextLayoutLineLeaf, b: usize) -> GlyphPoint {
        let mut base = b;
        let mut pt = GlyphPoint::default();
        for line in &leaf.visual_lines {
            let mut gh = line.line.max_ascent + line.line.max_descent;
            let mut co = (line.line_height as f32 - gh) / 2.0;
            if gh == 0.0 && leaf.default_glyph_height > 0.0 {
                gh = leaf.default_glyph_height;
                co = leaf.default_centering_offset;
            }
            match base.cmp(&line.utf8_len) {
                Ordering::Greater => {
                    base -= line.utf8_len;
                    pt.x = 0.;
                    pt.line_top += line.line_height;
                    pt.glyph_top += line.line_height;
                    pt.glyph_bottom += line.line_height;
                    pt.line_bottom += line.line_height;
                    continue;
                }
                Ordering::Equal => {
                    if line.line_ending != LineEnding::None || line.line_break {
                        base -= line.utf8_len;
                        pt.x = 0.;
                        pt.line_top += line.line_height;
                        pt.glyph_top += line.line_height;
                        pt.glyph_bottom += line.line_height;
                        pt.line_bottom += line.line_height;
                    } else {
                        pt.x = line.line.w as f64;
                        pt.glyph_top += co as f64;
                        pt.glyph_bottom += (co + gh) as f64;
                        pt.line_bottom += line.line_height;
                        return pt;
                    }
                }
                Ordering::Less => {
                    pt.x = 0.;
                    pt.glyph_top += co as f64;
                    pt.glyph_bottom += (co + gh) as f64;
                    pt.line_bottom += line.line_height;
                    for g in &line.line.glyphs {
                        let len = g.end - g.start;
                        if base < len {
                            pt.x = g.x as f64;
                            return pt;
                        }
                        base -= len;
                    }
                    pt.x = line.line.w as f64;
                    return pt;
                }
            }
        }
        pt
    }
    fn base(info: &TextLayoutLineInfo) -> usize {
        info.utf8_len
    }
    fn node_measured(info: &TextLayoutLineInfo) -> GlyphPoint {
        info.last_glyph.clone()
    }
    fn count_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }
    fn measure_affinty() -> UnitAffinity {
        UnitAffinity::After
    }
}

pub struct VlineLineConverter(());
impl UnitConverter<TextLayoutLineInfo, usize> for VlineLineConverter {
    fn count(l: &TextLayoutLineLeaf, m: usize) -> usize {
        let mut c = 0;
        let mut i = 0;
        for x in &l.visual_lines {
            if c >= m {
                return i;
            }
            if x.line_ending != LineEnding::None {
                c += 1;
            }
            i += 1;
        }
        i
    }
    fn measure(l: &TextLayoutLineLeaf, b: usize) -> usize {
        l.visual_lines[..b]
            .iter()
            .filter(|x| x.line_ending != LineEnding::None)
            .count()
    }
    fn base(info: &TextLayoutLineInfo) -> usize {
        info.line_breaks + info.line_endings
    }
    fn node_measured(info: &TextLayoutLineInfo) -> usize {
        info.line_endings
    }
    fn count_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }
    fn measure_affinty() -> UnitAffinity {
        UnitAffinity::Before
    }
}

pub struct VisualLineIter<'a> {
    cursor: Cursor<'a, TextLayoutLineInfo>,
    leaf_visual_index: Option<usize>,
    vline: usize,
    height: f32,
    end: usize,
}
impl<'a> Iterator for VisualLineIter<'a> {
    type Item = LayoutRun<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.pos() >= self.end {
            return None;
        }
        let (leaf, sp) = self.cursor.get_leaf()?;
        if sp >= leaf.len() {
            return None;
        }
        let idx = if let Some(i) = self.leaf_visual_index {
            i
        } else {
            let mut utf8 = 0;
            let mut i = 0;
            for l in &leaf.visual_lines {
                if sp <= utf8 {
                    break;
                }
                utf8 += l.utf8_len;
                i += 1;
            }
            self.leaf_visual_index = Some(i);
            i
        };
        let line = &leaf.visual_lines[idx];
        let gh = line.line.max_ascent + line.line.max_descent;
        let co = (line.line_height as f32 - gh) / 2.0;
        let ly = self.height + co + line.line.max_ascent;
        let run = LayoutRun {
            line_i: self.vline,
            text: "",
            rtl: false,
            glyphs: &line.line.glyphs,
            max_ascent: line.line.max_ascent,
            max_descent: line.line.max_descent,
            line_y: ly,
            line_top: self.height,
            line_height: line.line_height as f32,
            line_w: line.line.w,
        };
        self.height += line.line_height as f32;
        self.vline += 1;
        if sp + line.utf8_len >= leaf.len() {
            self.leaf_visual_index = Some(0);
            self.cursor.next_leaf();
        } else {
            self.leaf_visual_index = Some(idx + 1);
            self.cursor.set(self.cursor.pos() + line.utf8_len);
        }
        Some(run)
    }
}
