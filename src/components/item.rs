//! Item component – flexible list item container
//!
//! Based on shadcn/ui Item.
//!
//! # Example
//!
//! ```
//! use floem::view::ParentView;
//! use floem_shadcn::components::item::*;
//!
//! Item::new()
//!     .child(ItemContent::new()
//!         .child(ItemTitle::new("Settings"))
//!         .child(ItemDescription::new("Manage your preferences")));
//! ```

use crate::styled::ShadcnStyleExt;
use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// The Item.
pub struct Item {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl Item {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
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
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ())
                .style(|s| s.flex_row().items_center().gap_3().w_full().py_3().px_4())
        };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for Item {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `ItemContent`.
pub struct ItemContent {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl ItemContent {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for ItemContent {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ItemContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemContent {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| s.flex_grow(1.0_f32).flex_col().gap_0p5())
        };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for ItemContent {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `ItemTitle`.
pub struct ItemTitle {
    id: ViewId,
    text: String,
}
impl ItemTitle {
    /// Creates a new it.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for ItemTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| s.text_sm().text_foreground()))
    }
}

/// The `ItemDescription`.
pub struct ItemDescription {
    id: ViewId,
    text: String,
}
impl ItemDescription {
    /// Creates a new it.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for ItemDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(floem::views::Label::new(text).style(|s| s.text_xs().text_muted_foreground()))
    }
}

/// The `ItemGroup`.
pub struct ItemGroup {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl ItemGroup {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for ItemGroup {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ItemGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemGroup {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container =
            { Container::with_id(self.id, ()).style(|s| s.flex_col().w_full().gap_4()) };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for ItemGroup {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `ItemHeader`.
pub struct ItemHeader {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl ItemHeader {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for ItemHeader {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ItemHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemHeader {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| s.flex_row().items_center().gap_2().w_full())
        };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for ItemHeader {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `ItemMedia`.
pub struct ItemMedia {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl ItemMedia {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for ItemMedia {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ItemMedia {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemMedia {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.size_8()
                        .items_center()
                        .justify_center()
                        .border_1()
                        .border_color(t.border)
                        .border_radius(t.radius_sm)
                })
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for ItemMedia {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `ItemActions`.
pub struct ItemActions {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl ItemActions {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for ItemActions {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ItemActions {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemActions {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container =
            { Container::with_id(self.id, ()).style(|s| s.flex_row().items_center().gap_2()) };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for ItemActions {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `ItemSeparator`.
pub struct ItemSeparator {
    id: ViewId,
}
impl ItemSeparator {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self { id: ViewId::new() }
    }
}
impl Default for ItemSeparator {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ItemSeparator {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemSeparator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.w_full()
                .height(1.0)
                .with_shadcn_theme(|s, t| s.background(t.border))
        }))
    }
}

/// The `ItemFooter`.
pub struct ItemFooter {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl ItemFooter {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for ItemFooter {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for ItemFooter {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for ItemFooter {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| {
                s.flex_row()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .w_full()
                    .pt_2()
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for ItemFooter {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}
