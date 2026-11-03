//! Empty state component – placeholder for empty content
//! (tailwind-enhanced)

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// The Empty.
pub struct Empty {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl Empty {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for Empty {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for Empty {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ())
                .style(|s| s.flex_col().items_center().justify_center().w_full().py_6())
        };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for Empty {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `EmptyMedia`.
pub struct EmptyMedia {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl EmptyMedia {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for EmptyMedia {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for EmptyMedia {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyMedia {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container =
            { Container::with_id(self.id, ()).style(floem_tailwind::TailwindExt::mb_4) };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for EmptyMedia {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `EmptyContent`.
pub struct EmptyContent {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl EmptyContent {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for EmptyContent {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for EmptyContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyContent {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ())
                .style(|s| s.flex_col().items_center().max_width(420.0).gap_1())
        };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for EmptyContent {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `EmptyTitle`.
pub struct EmptyTitle {
    id: ViewId,
    text: String,
}
impl EmptyTitle {
    /// Creates a new it.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for EmptyTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_lg().color(t.foreground))),
        )
    }
}

/// The `EmptyDescription`.
pub struct EmptyDescription {
    id: ViewId,
    text: String,
}
impl EmptyDescription {
    /// Creates a new it.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}
impl HasViewId for EmptyDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Label::new(text)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_sm().color(t.muted_foreground))),
        )
    }
}

/// The `EmptyActions`.
pub struct EmptyActions {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl EmptyActions {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for EmptyActions {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for EmptyActions {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyActions {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = { Container::with_id(self.id, ()).style(|s| s.flex_row().gap_2().mt_2()) };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for EmptyActions {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `EmptyHeader`.
pub struct EmptyHeader {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl EmptyHeader {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for EmptyHeader {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for EmptyHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyHeader {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container =
            { Container::with_id(self.id, ()).style(|s| s.flex_col().items_center().gap_1()) };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for EmptyHeader {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// The `EmptyFooter`.
pub struct EmptyFooter {
    id: ViewId,
    children: Vec<floem::AnyView>,
}
impl EmptyFooter {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}
impl Default for EmptyFooter {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for EmptyFooter {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for EmptyFooter {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| s.flex_col().items_center().gap_1().mt_2())
        };
        self.id.set_children_vec(self.children);
        container
    }
}
impl ParentView for EmptyFooter {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}
