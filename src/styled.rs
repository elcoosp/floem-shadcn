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

macro_rules! bg_impl {
    ($name:ident, $field:ident) => {
        fn $name(self) -> Self {
            self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| {
                s.background(t.$field)
            })))
        }
    };
}

macro_rules! text_impl {
    ($name:ident, $field:ident) => {
        fn $name(self) -> Self {
            self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| {
                s.color(t.$field)
            })))
        }
    };
}

macro_rules! border_impl {
    ($name:ident, $field:ident) => {
        fn $name(self) -> Self {
            self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| {
                s.border_color(t.$field)
            })))
        }
    };
}

macro_rules! outline_impl {
    ($name:ident, $field:ident) => {
        fn $name(self) -> Self {
            self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| {
                s.outline_color(t.$field)
            })))
        }
    };
}

macro_rules! radius_impl {
    ($name:ident, $field:ident) => {
        fn $name(self) -> Self {
            self.set_context(ShadcnThemeProp, ContextValue::Computed(Box::new(|s: Style, t: &crate::theme::ShadcnTheme| {
                s.border_radius(t.$field)
            })))
        }
    };
}

impl ShadcnStyleExt for Style {
    bg_impl!(bg_background, background);
    bg_impl!(bg_foreground, foreground);
    bg_impl!(bg_card, card);
    bg_impl!(bg_card_foreground, card_foreground);
    bg_impl!(bg_popover, popover);
    bg_impl!(bg_popover_foreground, popover_foreground);
    bg_impl!(bg_primary, primary);
    bg_impl!(bg_primary_foreground, primary_foreground);
    bg_impl!(bg_secondary, secondary);
    bg_impl!(bg_secondary_foreground, secondary_foreground);
    bg_impl!(bg_muted, muted);
    bg_impl!(bg_muted_foreground, muted_foreground);
    bg_impl!(bg_accent, accent);
    bg_impl!(bg_accent_foreground, accent_foreground);
    bg_impl!(bg_destructive, destructive);
    bg_impl!(bg_destructive_foreground, destructive_foreground);

    text_impl!(text_background, background);
    text_impl!(text_foreground, foreground);
    text_impl!(text_card, card);
    text_impl!(text_card_foreground, card_foreground);
    text_impl!(text_popover, popover);
    text_impl!(text_popover_foreground, popover_foreground);
    text_impl!(text_primary, primary);
    text_impl!(text_primary_foreground, primary_foreground);
    text_impl!(text_secondary, secondary);
    text_impl!(text_secondary_foreground, secondary_foreground);
    text_impl!(text_muted, muted);
    text_impl!(text_muted_foreground, muted_foreground);
    text_impl!(text_accent, accent);
    text_impl!(text_accent_foreground, accent_foreground);
    text_impl!(text_destructive, destructive);
    text_impl!(text_destructive_foreground, destructive_foreground);

    border_impl!(border_border, border);
    border_impl!(border_input, input);
    border_impl!(border_ring, ring);
    border_impl!(border_primary, primary);
    border_impl!(border_secondary, secondary);
    border_impl!(border_destructive, destructive);
    border_impl!(border_muted, muted);
    border_impl!(border_accent, accent);

    outline_impl!(outline_ring, ring);
    outline_impl!(outline_primary, primary);
    outline_impl!(outline_destructive, destructive);

    radius_impl!(rounded_radius, radius);
    radius_impl!(rounded_radius_sm, radius_sm);
    radius_impl!(rounded_radius_md, radius_md);
    radius_impl!(rounded_radius_lg, radius_lg);
}
