//! Input Group component – input with adornments
//!
//! Based on shadcn/ui InputGroup.
//!
//! # Example
//!
//! ```rust
//! use floem::view::ParentView;
//! use floem_shadcn::components::input::Input;
//! use floem_shadcn::components::input_group::{InputGroup, InputGroupAddon, AddonPosition};
//!
//! InputGroup::new()
//!     .child(InputGroupAddon::new(AddonPosition::Prefix, label("$")))
//!     .child(Input::new());
//! ```

use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Position of an addon within an input group
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddonPosition {
    Prefix,
    Suffix,
}

/// Container that groups inputs with adornments (addons)
pub struct InputGroup {
    id: ViewId,
    disabled: bool,
}

impl InputGroup {
    /// Create a new input group
    pub fn new() -> Self {
        Self { id: ViewId::new(), disabled: false }
    }

    /// Set the group as disabled (propagates to children)
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Default for InputGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for InputGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for InputGroup {
    type V = Container;


    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Row)
                .items_center()
        })
    }
}

impl ParentView for InputGroup {}

/// Addon element (prefix or suffix) for InputGroup
pub struct InputGroupAddon<V> {
    id: ViewId,
    position: AddonPosition,
    child: V,
}

impl<V: IntoView + 'static> InputGroupAddon<V> {
    /// Create a new addon
    pub fn new(position: AddonPosition, child: V) -> Self {
        Self { id: ViewId::new(), position, child }
    }
}

impl<V: IntoView + 'static> HasViewId for InputGroupAddon<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for InputGroupAddon<V> {
    type V = Box<dyn View>;


    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| {
                    s.with_shadcn_theme(|s, t| {
                        s.padding_left(8.0).padding_right(8.0)
                            .padding_top(6.0).padding_bottom(6.0)
                            .font_size(14.0)
                            .background(t.muted)
                            .color(t.muted_foreground)
                            .border(1.0)
                            .border_color(t.input)
                    })
                }),
        )
    }
}
