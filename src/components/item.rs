//! Item component – flexible list item container
//!
//! Based on shadcn/ui Item.
//!
//! # Example
//!
//! ```rust
//! use floem::view::ParentView;
//! use floem_shadcn::components::item::*;
//!
//! Item::new()
//!     .child(ItemContent::new()
//!         .child(ItemTitle::new("Settings"))
//!         .child(ItemDescription::new("Manage your preferences")));
//! ```

use floem::prelude::*;
use floem::style::FontWeight;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Main item container – renders as a horizontal row
pub struct Item {
    id: ViewId,
}

impl Item {
    /// Create a new Item
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Item {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Item {
    type V = Container;
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Row)
                .items_center()
                .gap(12.0)
                .width_full()
                .padding_top(12.0).padding_bottom(12.0)
                .padding_left(16.0).padding_right(16.0)
        })
    }
}

impl ParentView for Item {}

/// Content area inside an Item
pub struct ItemContent {
    id: ViewId,
}

impl ItemContent {
    pub fn new() -> Self { Self { id: ViewId::new() } }
}

impl Default for ItemContent {
    fn default() -> Self { Self::new() }
}

impl HasViewId for ItemContent {
    fn view_id(&self) -> ViewId { self.id }
}

impl IntoView for ItemContent {
    type V = Container;
    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| {
            s.flex_grow(1.0)
                .flex_direction(floem::style::FlexDirection::Column)
                .gap(2.0)
        })
    }
}

impl ParentView for ItemContent {}

/// Title text inside an ItemContent
pub struct ItemTitle {
    id: ViewId,
    text: String,
}

impl ItemTitle {
    pub fn new(text: impl Into<String>) -> Self {
        Self { id: ViewId::new(), text: text.into() }
    }
}

impl HasViewId for ItemTitle { fn view_id(&self) -> ViewId { self.id } }

impl IntoView for ItemTitle {
    type V = Box<dyn View>;
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(14.0).font_weight(FontWeight::MEDIUM).text_foreground()
        }))
    }
}

/// Description text inside an ItemContent
pub struct ItemDescription {
    id: ViewId,
    text: String,
}

impl ItemDescription {
    pub fn new(text: impl Into<String>) -> Self {
        Self { id: ViewId::new(), text: text.into() }
    }
}

impl HasViewId for ItemDescription { fn view_id(&self) -> ViewId { self.id } }

impl IntoView for ItemDescription {
    type V = Box<dyn View>;
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.font_size(13.0).text_muted_foreground()
        }))
    }
}
