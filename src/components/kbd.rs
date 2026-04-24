//! Keyboard key component
//!
//! Based on shadcn/ui Kbd.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::kbd::Kbd;
//!
//! let key = Kbd::new("⌘");
//! ```

use floem::prelude::*;
use floem::style::FontWeight;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// A single keyboard key display
pub struct Kbd {
    id: ViewId,
    key: String,
}

impl Kbd {
    /// Create a new Kbd element
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            key: key.into(),
        }
    }
}

impl HasViewId for Kbd {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Kbd {
    type V = Box<dyn View>;


    fn into_view(self) -> Self::V {
        let key = self.key;

        Box::new(
            floem::views::Label::new(key)
                .style(|s| {
                    s.with_shadcn_theme(|s, t| {
                        s.padding_left(4.0)
                            .padding_right(4.0)
                            .padding_top(2.0)
                            .padding_bottom(2.0)
                            .font_size(11.0)
                            .font_weight(FontFontFontWeight::MEDIUM)
                            .background(t.muted)
                            .color(t.muted_foreground)
                            .border(1.0)
                            .border_color(t.border)
                            .border_radius(t.radius_sm)
                            .line_height(1.0)
                    })
                }),
        )
    }
}

/// A group of keyboard keys (e.g., for shortcuts)
pub struct KbdGroup {
    id: ViewId,
    keys: Vec<String>,
}

impl KbdGroup {
    /// Create a new KbdGroup
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            id: ViewId::new(),
            keys,
        }
    }
}

impl HasViewId for KbdGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for KbdGroup {
    type V = Box<dyn View>;


    fn into_view(self) -> Self::V {
        let keys = self.keys;
        let mut children: Vec<Box<dyn View>> = Vec::new();

        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                children.push(Box::new(
                    floem::views::Label::new("+")
                        .style(|s| {
                            s.with_shadcn_theme(|s, t| {
                                s.margin_left(4.0)
                                    .margin_right(4.0)
                                    .font_size(11.0)
                                    .color(t.muted_foreground)
                            })
                        }),
                ));
            }
            children.push(Box::new(Kbd::new(key.clone())));
        }

        Box::new(
            floem::views::Stack::horizontal_from_iter(children)
                .style(|s| s.items_center()),
        )
    }
}
