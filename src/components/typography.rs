//! Typography components – text styling primitives
//!
//! Based on shadcn/ui Typography.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::typography::*;
//!
//! let heading = TypographyH1::new("Title");
//! let text = TypographyP::new("Some paragraph.");
//! ```

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::styled::ShadcnStyleExt;

/// H1 heading
pub struct TypographyH1 { id: ViewId, text: String }
impl TypographyH1 { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyH1 { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyH1 {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(36.0).line_height(1.2).text_foreground()
        }))
    }
}

/// H2 heading
pub struct TypographyH2 { id: ViewId, text: String }
impl TypographyH2 { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyH2 { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyH2 {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(24.0).line_height(1.3).text_foreground()
        }))
    }
}

/// H3 heading
pub struct TypographyH3 { id: ViewId, text: String }
impl TypographyH3 { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyH3 { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyH3 {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(20.0).line_height(1.4).text_foreground()
        }))
    }
}

/// H4 heading
pub struct TypographyH4 { id: ViewId, text: String }
impl TypographyH4 { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyH4 { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyH4 {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(16.0).line_height(1.5).text_foreground()
        }))
    }
}

/// Paragraph
pub struct TypographyP { id: ViewId, text: String }
impl TypographyP { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyP { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyP {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(14.0).line_height(1.6).text_foreground()
        }))
    }
}

/// Lead paragraph (larger intro text)
pub struct TypographyLead { id: ViewId, text: String }
impl TypographyLead { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyLead { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyLead {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(18.0).line_height(1.5).text_muted_foreground()
        }))
    }
}

/// Muted text (small, subdued)
pub struct TypographyMuted { id: ViewId, text: String }
impl TypographyMuted { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyMuted { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyMuted {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(12.0).line_height(1.5).text_muted_foreground()
        }))
    }
}

/// Inline code
pub struct TypographyCode { id: ViewId, text: String }
impl TypographyCode { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyCode { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyCode {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.padding_left(4.0).padding_right(4.0).padding_top(2.0).padding_bottom(2.0)
                .font_size(13.0).bg_muted().text_foreground()
                .border(1.0).border_border().border_radius(4.0)
                
        }))
    }
}

/// Blockquote
pub struct TypographyBlockquote { id: ViewId, text: String }
impl TypographyBlockquote { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for TypographyBlockquote { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyBlockquote {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.margin_top(16.0).margin_bottom(16.0)
                .padding_left(16.0)
                .border_left(2.0).border_border()
                .font_size(14.0).line_height(1.6).text_muted_foreground()
        }))
    }
}

/// Unordered/ordered list
pub struct TypographyList { id: ViewId, items: Vec<String>, ordered: bool }
impl TypographyList {
    pub fn new(items: Vec<String>, ordered: bool) -> Self { Self { id: ViewId::new(), items, ordered } }
}
impl HasViewId for TypographyList { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for TypographyList {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let items = self.items;
        let ordered = self.ordered;
        let children: Vec<Box<dyn View>> = items.iter().enumerate().map(|(i, item)| {
            let bullet = if ordered { format!("{}. ", i + 1) } else { "• ".to_string() };
            let text = format!("{}{}", bullet, item);
            Box::new(floem::views::Label::new(text).style(|s| {
                s.font_size(14.0).line_height(1.6).text_foreground()
            })) as Box<dyn View>
        }).collect();
        Box::new(floem::views::Stack::vertical_from_iter(children)
            .style(|s| s.margin_top(8.0).margin_bottom(8.0).gap(4.0)))
    }
}
