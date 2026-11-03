use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// Keyboard key / shortcut chip.
///
/// Based on shadcn/ui Kbd: a small muted chip that renders a key or
/// combination, e.g. <Kbd>⌘</Kbd><Kbd>K</Kbd>.
pub struct Kbd {
    id: ViewId,
    key: String,
}
impl Kbd {
    /// Creates a new it.
    pub fn new(k: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            key: k.into(),
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        // The stored ViewId must be attached to the rendered view, otherwise
        // queries through `view_id()` target an orphan node.
        Box::new(floem::views::Label::with_id(self.id, self.key).style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.padding_left(4.0)
                    .padding_right(4.0)
                    .padding_top(2.0)
                    .padding_bottom(2.0)
                    .font_size(11.0)
                    .background(t.muted)
                    .color(t.muted_foreground)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(t.radius_sm)
                    .line_height(1.0)
            })
        }))
    }
}

/// A group of keyboard keys rendered inline with a small gap.
///
/// Based on shadcn/ui `KbdGroup`: <KbdGroup>{[`⌘`, `K`]}</KbdGroup> renders
/// each key as a [`Kbd`] chip inside a horizontal stack with `gap-1`.
pub struct KbdGroup {
    id: ViewId,
    keys: Vec<String>,
}
impl KbdGroup {
    #[must_use]
    /// Creates a new it.
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
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let keys: Vec<Kbd> = self.keys.into_iter().map(Kbd::new).collect();
        // Build the stack first (deferred child registration), then attach the
        // pre-created ViewId via Container so `view_id()` queries resolve.
        // `Stack::with_id` eagerly converts children inside the storage borrow
        // and panics with "RefCell already borrowed" when a view is nested.
        let inner = floem::views::Stack::from_iter(keys).style(|s| s.items_center().gap_1());
        Box::new(floem::views::Container::with_id(self.id, inner))
    }
}
