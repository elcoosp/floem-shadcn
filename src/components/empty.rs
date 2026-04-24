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
use floem::style::FontWeight;
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
    }
}

impl ParentView for EmptyActions {}
