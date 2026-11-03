//! Button Group component – groups related buttons with connected styling
//!
//! Based on shadcn/ui `ButtonGroup`.
//!
//! # Example
//!
//! ```
//! use floem::view::ParentView;
//! use floem_shadcn::components::button::Button;
//! use floem_shadcn::components::button_group::{ButtonGroup, ButtonGroupSeparator};
//!
//! ButtonGroup::new()
//!     .child(Button::new("Left"))
//!     .child(ButtonGroupSeparator::new())
//!     .child(Button::new("Right"));
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// The `ButtonGroup`.
pub struct ButtonGroup {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl ButtonGroup {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
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
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = { Container::with_id(self.id, ()).style(|s| s.flex_row().items_center()) };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for ButtonGroup {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `ButtonGroupSeparator`.
pub struct ButtonGroupSeparator {
    id: ViewId,
}
impl ButtonGroupSeparator {
    #[must_use]
    /// Creates a new it.
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.w_px()
                .h_full()
                .with_shadcn_theme(|s, t| s.background(t.border))
        }))
    }
}

/// Non-interactive text label styled to sit between buttons in a group
pub struct ButtonGroupText {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl ButtonGroupText {
    #[must_use]
    /// Create a new button group text label
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for ButtonGroupText {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ButtonGroupText {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl ParentView for ButtonGroupText {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}
impl IntoView for ButtonGroupText {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Box::new(floem::views::Container::with_id(self.id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.flex_row()
                        .items_center()
                        .gap_2()
                        .height(32.0)
                        .padding_left(10.0)
                        .padding_right(10.0)
                        .border_1()
                        .border_color(t.border)
                        .border_radius(t.radius_md)
                        .background(t.muted)
                        .text_sm()
                        .font_medium()
                        .color(t.foreground)
                })
            }))
        };
        self.id.set_children_vec(self.children);
        container
    }
}
