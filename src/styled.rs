use crate::theme::ShadcnThemeProp;
use floem::style::{Style, ContextValue};
use floem::peniko::Color;

pub trait ShadcnStyleExt: Sized {
    fn bg_background(self) -> Self;
    fn bg_foreground(self) -> Self;
    fn bg_card(self) -> Self;
    fn bg_card_foreground(self) -> Self;
    fn bg_popover(self) -> Self;
    fn bg_popover_foreground(self) -> Self;
    fn bg_primary(self) -> Self;
    fn bg_primary_foreground(self) -> Self;
    fn bg_secondary(self) -> Self;
    fn bg_secondary_foreground(self) -> Self;
    fn bg_muted(self) -> Self;
    fn bg_muted_foreground(self) -> Self;
    fn bg_accent(self) -> Self;
    fn bg_accent_foreground(self) -> Self;
    fn bg_destructive(self) -> Self;
    fn bg_destructive_foreground(self) -> Self;

    fn text_background(self) -> Self;
    fn text_foreground(self) -> Self;
    fn text_card(self) -> Self;
    fn text_card_foreground(self) -> Self;
    fn text_popover(self) -> Self;
    fn text_popover_foreground(self) -> Self;
    fn text_primary(self) -> Self;
    fn text_primary_foreground(self) -> Self;
    fn text_secondary(self) -> Self;
    fn text_secondary_foreground(self) -> Self;
    fn text_muted(self) -> Self;
    fn text_muted_foreground(self) -> Self;
    fn text_accent(self) -> Self;
    fn text_accent_foreground(self) -> Self;
    fn text_destructive(self) -> Self;
    fn text_destructive_foreground(self) -> Self;

    fn border_border(self) -> Self;
    fn border_input(self) -> Self;
    fn border_ring(self) -> Self;
    fn border_primary(self) -> Self;
    fn border_secondary(self) -> Self;
    fn border_destructive(self) -> Self;
    fn border_muted(self) -> Self;
    fn border_accent(self) -> Self;

    fn outline_ring(self) -> Self;
    fn outline_primary(self) -> Self;
    fn outline_destructive(self) -> Self;

    fn rounded_radius(self) -> Self;
    fn rounded_radius_sm(self) -> Self;
    fn rounded_radius_md(self) -> Self;
    fn rounded_radius_lg(self) -> Self;
}

impl ShadcnStyleExt for Style {
    fn bg_background(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.background)))) }
    fn bg_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.foreground)))) }
    fn bg_card(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.card)))) }
    fn bg_card_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.card_foreground)))) }
    fn bg_popover(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.popover)))) }
    fn bg_popover_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.popover_foreground)))) }
    fn bg_primary(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.primary)))) }
    fn bg_primary_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.primary_foreground)))) }
    fn bg_secondary(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.secondary)))) }
    fn bg_secondary_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.secondary_foreground)))) }
    fn bg_muted(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.muted)))) }
    fn bg_muted_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.muted_foreground)))) }
    fn bg_accent(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.accent)))) }
    fn bg_accent_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.accent_foreground)))) }
    fn bg_destructive(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.destructive)))) }
    fn bg_destructive_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.background(t.destructive_foreground)))) }

    fn text_background(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.background)))) }
    fn text_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.foreground)))) }
    fn text_card(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.card)))) }
    fn text_card_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.card_foreground)))) }
    fn text_popover(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.popover)))) }
    fn text_popover_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.popover_foreground)))) }
    fn text_primary(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.primary)))) }
    fn text_primary_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.primary_foreground)))) }
    fn text_secondary(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.secondary)))) }
    fn text_secondary_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.secondary_foreground)))) }
    fn text_muted(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.muted)))) }
    fn text_muted_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.muted_foreground)))) }
    fn text_accent(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.accent)))) }
    fn text_accent_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.accent_foreground)))) }
    fn text_destructive(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.destructive)))) }
    fn text_destructive_foreground(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.color(t.destructive_foreground)))) }

    fn border_border(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_color(t.border)))) }
    fn border_input(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_color(t.input)))) }
    fn border_ring(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_color(t.ring)))) }
    fn border_primary(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_color(t.primary)))) }
    fn border_secondary(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_color(t.secondary)))) }
    fn border_destructive(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_color(t.destructive)))) }
    fn border_muted(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_color(t.muted)))) }
    fn border_accent(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_color(t.accent)))) }

    fn outline_ring(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.outline_color(t.ring)))) }
    fn outline_primary(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.outline_color(t.primary)))) }
    fn outline_destructive(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.outline_color(t.destructive)))) }

    fn rounded_radius(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_radius(t.radius)))) }
    fn rounded_radius_sm(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_radius(t.radius_sm)))) }
    fn rounded_radius_md(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_radius(t.radius_md)))) }
    fn rounded_radius_lg(self) -> Self { self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| s.border_radius(t.radius_lg)))) }
}
