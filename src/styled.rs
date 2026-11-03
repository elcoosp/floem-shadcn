use crate::theme::current_theme;
use floem::style::Style;

/// The `ShadcnStyleExt`.
pub trait ShadcnStyleExt: Sized {
    #[must_use]
    /// The bg background.
    fn bg_background(self) -> Self;
    #[must_use]
    /// The bg foreground.
    fn bg_foreground(self) -> Self;
    #[must_use]
    /// The bg card.
    fn bg_card(self) -> Self;
    #[must_use]
    /// The bg card foreground.
    fn bg_card_foreground(self) -> Self;
    #[must_use]
    /// The bg popover.
    fn bg_popover(self) -> Self;
    #[must_use]
    /// The bg popover foreground.
    fn bg_popover_foreground(self) -> Self;
    #[must_use]
    /// The bg primary.
    fn bg_primary(self) -> Self;
    #[must_use]
    /// The bg primary foreground.
    fn bg_primary_foreground(self) -> Self;
    #[must_use]
    /// The bg secondary.
    fn bg_secondary(self) -> Self;
    #[must_use]
    /// The bg secondary foreground.
    fn bg_secondary_foreground(self) -> Self;
    #[must_use]
    /// The bg muted.
    fn bg_muted(self) -> Self;
    #[must_use]
    /// The bg muted foreground.
    fn bg_muted_foreground(self) -> Self;
    #[must_use]
    /// The bg accent.
    fn bg_accent(self) -> Self;
    #[must_use]
    /// The bg accent foreground.
    fn bg_accent_foreground(self) -> Self;
    #[must_use]
    /// The bg destructive.
    fn bg_destructive(self) -> Self;
    #[must_use]
    /// The bg destructive foreground.
    fn bg_destructive_foreground(self) -> Self;
    #[must_use]
    /// The text background.
    fn text_background(self) -> Self;
    #[must_use]
    /// The text foreground.
    fn text_foreground(self) -> Self;
    #[must_use]
    /// The text card.
    fn text_card(self) -> Self;
    #[must_use]
    /// The text card foreground.
    fn text_card_foreground(self) -> Self;
    #[must_use]
    /// The text popover.
    fn text_popover(self) -> Self;
    #[must_use]
    /// The text popover foreground.
    fn text_popover_foreground(self) -> Self;
    #[must_use]
    /// The text primary.
    fn text_primary(self) -> Self;
    #[must_use]
    /// The text primary foreground.
    fn text_primary_foreground(self) -> Self;
    #[must_use]
    /// The text secondary.
    fn text_secondary(self) -> Self;
    #[must_use]
    /// The text secondary foreground.
    fn text_secondary_foreground(self) -> Self;
    #[must_use]
    /// The text muted.
    fn text_muted(self) -> Self;
    #[must_use]
    /// The text muted foreground.
    fn text_muted_foreground(self) -> Self;
    #[must_use]
    /// The text accent.
    fn text_accent(self) -> Self;
    #[must_use]
    /// The text accent foreground.
    fn text_accent_foreground(self) -> Self;
    #[must_use]
    /// The text destructive.
    fn text_destructive(self) -> Self;
    #[must_use]
    /// The text destructive foreground.
    fn text_destructive_foreground(self) -> Self;
    #[must_use]
    /// The border border.
    fn border_border(self) -> Self;
    #[must_use]
    /// The border input.
    fn border_input(self) -> Self;
    #[must_use]
    /// The border ring.
    fn border_ring(self) -> Self;
    #[must_use]
    /// The border primary.
    fn border_primary(self) -> Self;
    #[must_use]
    /// The border secondary.
    fn border_secondary(self) -> Self;
    #[must_use]
    /// The border destructive.
    fn border_destructive(self) -> Self;
    #[must_use]
    /// The border muted.
    fn border_muted(self) -> Self;
    #[must_use]
    /// The border accent.
    fn border_accent(self) -> Self;
    #[must_use]
    /// The outline ring.
    fn outline_ring(self) -> Self;
    #[must_use]
    /// The outline primary.
    fn outline_primary(self) -> Self;
    #[must_use]
    /// The outline destructive.
    fn outline_destructive(self) -> Self;
    #[must_use]
    /// The rounded radius.
    fn rounded_radius(self) -> Self;
    #[must_use]
    /// The rounded radius sm.
    fn rounded_radius_sm(self) -> Self;
    #[must_use]
    /// The rounded radius md.
    fn rounded_radius_md(self) -> Self;
    #[must_use]
    /// The rounded radius lg.
    fn rounded_radius_lg(self) -> Self;
}
macro_rules! bg {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.background(t.$f)
        }
    };
}
macro_rules! txt {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.color(t.$f)
        }
    };
}
macro_rules! brd {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.border_color(t.$f)
        }
    };
}
macro_rules! out {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.outline_color(t.$f)
        }
    };
}
macro_rules! rad {
    ($n:ident,$f:ident) => {
        fn $n(self) -> Self {
            let t = current_theme();
            self.border_radius(t.$f)
        }
    };
}
impl ShadcnStyleExt for Style {
    bg!(bg_background, background);
    bg!(bg_foreground, foreground);
    bg!(bg_card, card);
    bg!(bg_card_foreground, card_foreground);
    bg!(bg_popover, popover);
    bg!(bg_popover_foreground, popover_foreground);
    bg!(bg_primary, primary);
    bg!(bg_primary_foreground, primary_foreground);
    bg!(bg_secondary, secondary);
    bg!(bg_secondary_foreground, secondary_foreground);
    bg!(bg_muted, muted);
    bg!(bg_muted_foreground, muted_foreground);
    bg!(bg_accent, accent);
    bg!(bg_accent_foreground, accent_foreground);
    bg!(bg_destructive, destructive);
    bg!(bg_destructive_foreground, destructive_foreground);
    txt!(text_background, background);
    txt!(text_foreground, foreground);
    txt!(text_card, card);
    txt!(text_card_foreground, card_foreground);
    txt!(text_popover, popover);
    txt!(text_popover_foreground, popover_foreground);
    txt!(text_primary, primary);
    txt!(text_primary_foreground, primary_foreground);
    txt!(text_secondary, secondary);
    txt!(text_secondary_foreground, secondary_foreground);
    txt!(text_muted, muted);
    txt!(text_muted_foreground, muted_foreground);
    txt!(text_accent, accent);
    txt!(text_accent_foreground, accent_foreground);
    txt!(text_destructive, destructive);
    txt!(text_destructive_foreground, destructive_foreground);
    brd!(border_border, border);
    brd!(border_input, input);
    brd!(border_ring, ring);
    brd!(border_primary, primary);
    brd!(border_secondary, secondary);
    brd!(border_destructive, destructive);
    brd!(border_muted, muted);
    brd!(border_accent, accent);
    out!(outline_ring, ring);
    out!(outline_primary, primary);
    out!(outline_destructive, destructive);
    rad!(rounded_radius, radius);
    rad!(rounded_radius_sm, radius_sm);
    rad!(rounded_radius_md, radius_md);
    rad!(rounded_radius_lg, radius_lg);
}
