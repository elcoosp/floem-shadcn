//! Semantic Tailwind-style utilities for floem-shadcn

use crate::theme::ShadcnThemeProp;
use floem::style::Style;

pub trait ShadcnStyleExt: Sized {
    fn bg_background(self) -> Self; fn bg_foreground(self) -> Self; fn bg_card(self) -> Self; fn bg_card_foreground(self) -> Self;
    fn bg_popover(self) -> Self; fn bg_popover_foreground(self) -> Self; fn bg_primary(self) -> Self; fn bg_primary_foreground(self) -> Self;
    fn bg_secondary(self) -> Self; fn bg_secondary_foreground(self) -> Self; fn bg_muted(self) -> Self; fn bg_muted_foreground(self) -> Self;
    fn bg_accent(self) -> Self; fn bg_accent_foreground(self) -> Self; fn bg_destructive(self) -> Self; fn bg_destructive_foreground(self) -> Self;
    fn text_background(self) -> Self; fn text_foreground(self) -> Self; fn text_card(self) -> Self; fn text_card_foreground(self) -> Self;
    fn text_popover(self) -> Self; fn text_popover_foreground(self) -> Self; fn text_primary(self) -> Self; fn text_primary_foreground(self) -> Self;
    fn text_secondary(self) -> Self; fn text_secondary_foreground(self) -> Self; fn text_muted(self) -> Self; fn text_muted_foreground(self) -> Self;
    fn text_accent(self) -> Self; fn text_accent_foreground(self) -> Self; fn text_destructive(self) -> Self; fn text_destructive_foreground(self) -> Self;
    fn border_border(self) -> Self; fn border_input(self) -> Self; fn border_ring(self) -> Self; fn border_primary(self) -> Self;
    fn border_secondary(self) -> Self; fn border_destructive(self) -> Self; fn border_muted(self) -> Self; fn border_accent(self) -> Self;
    fn outline_ring(self) -> Self; fn outline_primary(self) -> Self; fn outline_destructive(self) -> Self;
    fn rounded_radius(self) -> Self; fn rounded_radius_sm(self) -> Self; fn rounded_radius_md(self) -> Self; fn rounded_radius_lg(self) -> Self;
}

impl ShadcnStyleExt for Style {
    fn bg_background(self) -> Self { self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.background)) }
    fn bg_foreground(self) -> Self { self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.foreground)) }
    fn bg_card(self) -> Self { self.with_context::<ShadcnThemeProp>(|s, t| s.background(t.card)) }
    // ... (rest of the methods, same pattern)
    fn rounded_radius_lg(self) -> Self { self.with_context::<ShadcnThemeProp>(|s, t| s.border_radius(t.radius_lg)) }
}
