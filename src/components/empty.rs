//! Empty state component – placeholder for empty content
//!
//! Based on shadcn/ui Empty.
//!
//! # Example
//!
//! ```rust
//! use floem::view::ParentView;
//! use floem_shadcn::components::empty::*;
//!
//! Empty::new()
//!     .child(EmptyMedia::new().child(svg_icon))
//!     .child(EmptyContent::new()
//!         .child(EmptyTitle::new("No results"))
//!         .child(EmptyDescription::new("Try adjusting your search")));
//! ```

use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Main empty state container – centers content vertically
pub struct Empty {
    id: ViewId,
}

impl Empty {
    pub fn new() -> Self { Self { id: ViewId::new() } }
}

impl Default for Empty {
    fn default() -> Self { Self::new() }
}

impl HasViewId for Empty {
    fn view_id(&self) -> ViewId { self.id }
}

impl IntoView for Empty {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
                .items_center()
                .justify_center()
                .width_full()
                .padding_top(24.0).padding_bottom(24.0)
        })
    }
}

impl ParentView for Empty {}

/// Media slot for Empty (icons, images)
pub struct EmptyMedia {
    id: ViewId,
}

impl EmptyMedia {
    pub fn new() -> Self { Self { id: ViewId::new() } }
}

impl Default for EmptyMedia {
    fn default() -> Self { Self::new() }
}

impl HasViewId for EmptyMedia {
    fn view_id(&self) -> ViewId { self.id }
}

impl IntoView for EmptyMedia {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| {
            s.margin_bottom(16.0)
        })
    }
}

impl ParentView for EmptyMedia {}

/// Content slot for Empty (title, description, actions)
pub struct EmptyContent {
    id: ViewId,
}

impl EmptyContent {
    pub fn new() -> Self { Self { id: ViewId::new() } }
}

impl Default for EmptyContent {
    fn default() -> Self { Self::new() }
}

impl HasViewId for EmptyContent {
    fn view_id(&self) -> ViewId { self.id }
}

impl IntoView for EmptyContent {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Column)
                .items_center()
                .max_width(420.0)
                .gap(4.0)
        })
    }
}

impl ParentView for EmptyContent {}

/// Title for empty state
pub struct EmptyTitle {
    id: ViewId,
    text: String,
}

impl EmptyTitle {
    pub fn new(text: impl Into<String>) -> Self {
        Self { id: ViewId::new(), text: text.into() }
    }
}

impl HasViewId for EmptyTitle { fn view_id(&self) -> ViewId { self.id } }

impl IntoView for EmptyTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.font_size(16.0).color(t.foreground)
            })
        }))
    }
}

/// Description for empty state
pub struct EmptyDescription {
    id: ViewId,
    text: String,
}

impl EmptyDescription {
    pub fn new(text: impl Into<String>) -> Self {
        Self { id: ViewId::new(), text: text.into() }
    }
}

impl HasViewId for EmptyDescription { fn view_id(&self) -> ViewId { self.id } }

impl IntoView for EmptyDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.font_size(14.0).color(t.muted_foreground)
            })
        }))
    }
}

/// Actions slot for empty state (buttons, links)
pub struct EmptyActions {
    id: ViewId,
}

impl EmptyActions {
    pub fn new() -> Self { Self { id: ViewId::new() } }
}

impl Default for EmptyActions {
    fn default() -> Self { Self::new() }
}

impl HasViewId for EmptyActions {
    fn view_id(&self) -> ViewId { self.id }
}

impl IntoView for EmptyActions {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Row)
                .gap(8.0)
                .margin_top(8.0)
        })
    }
}

impl ParentView for EmptyActions {}
