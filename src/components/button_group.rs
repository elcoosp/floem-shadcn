//! Button Group component – groups related buttons with connected styling
//!
//! Based on shadcn/ui ButtonGroup.
//!
//! # Example
//!
//! ```rust
//! use floem::view::ParentView;
//! use floem_shadcn::components::button::Button;
//! use floem_shadcn::components::button_group::{ButtonGroup, ButtonGroupSeparator};
//!
//! ButtonGroup::new()
//!     .child(Button::new("Left"))
//!     .child(ButtonGroupSeparator::new())
//!     .child(Button::new("Right"));
//! ```

use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;

/// Position of a button within the group
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonPosition {
    First,
    Middle,
    Last,
    Only,
}

/// Container that groups buttons with zero gap and connected borders
///
/// Uses `ParentView` to accept child buttons.
pub struct ButtonGroup {
    id: ViewId,
}

impl ButtonGroup {
    /// Create a new button group
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for ButtonGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ButtonGroup {
    type V = Container;


    fn into_view(self) -> Self::V {
        Container::with_id(self.id, ()).style(|s| {
            s.display(floem::style::Display::Flex)
                .flex_direction(floem::style::FlexDirection::Row)
                .items_center()
        })
    }
}

impl ParentView for ButtonGroup {}

/// Vertical separator line between buttons in a group
pub struct ButtonGroupSeparator {
    id: ViewId,
}

impl ButtonGroupSeparator {
    /// Create a new separator
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}

impl Default for ButtonGroupSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for ButtonGroupSeparator {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ButtonGroupSeparator {
    type V = Box<dyn View>;


    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.width(1.0)
                .height_full()
                .with_shadcn_theme(|s, t| s.background(t.border))
        }))
    }
}
