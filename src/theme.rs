//! Theme system for floem-shadcn

use floem::peniko::Color;
use floem::peniko::color::{AlphaColor, HueDirection, Oklch, Srgb};
use floem::style::{Style, StylePropValue};
use num_traits::ToPrimitive;

#[inline]
fn oklch(l: f32, c: f32, h: f32) -> Color {
    AlphaColor::<Oklch>::new([l, c, h, 1.0]).convert::<Srgb>()
}
#[inline]
fn oklcha(l: f32, c: f32, h: f32, a: f32) -> Color {
    AlphaColor::<Oklch>::new([l, c, h, a]).convert::<Srgb>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// The `ThemeMode`.
pub enum ThemeMode {
    #[default]
    /// The item.
    Light,
    /// The item.
    Dark,
}
#[derive(Debug, Clone, Copy, PartialEq)]
/// The `ShadcnTheme`.
pub struct ShadcnTheme {
    /// The item.
    pub mode: ThemeMode,
    /// The item.
    pub background: Color,
    /// The item.
    pub foreground: Color,
    /// The item.
    pub card: Color,
    /// The item.
    pub card_foreground: Color,
    /// The item.
    pub popover: Color,
    /// The item.
    pub popover_foreground: Color,
    /// The item.
    pub primary: Color,
    /// The item.
    pub primary_foreground: Color,
    /// The item.
    pub secondary: Color,
    /// The item.
    pub secondary_foreground: Color,
    /// The item.
    pub muted: Color,
    /// The item.
    pub muted_foreground: Color,
    /// The item.
    pub accent: Color,
    /// The item.
    pub accent_foreground: Color,
    /// The item.
    pub destructive: Color,
    /// The item.
    pub destructive_foreground: Color,
    /// The item.
    pub border: Color,
    /// The item.
    pub input: Color,
    /// The item.
    pub ring: Color,
    /// The item.
    pub radius: f32,
    /// The item.
    pub radius_sm: f32,
    /// The item.
    pub radius_md: f32,
    /// The item.
    pub radius_lg: f32,
}
thread_local! { static THEME: std::cell::RefCell<ShadcnTheme> = std::cell::RefCell::new(ShadcnTheme::light()); }
#[must_use]
/// The current theme.
pub fn current_theme() -> ShadcnTheme {
    THEME.with(|t| *t.borrow())
}
/// Sets theme.
pub fn set_theme(t: ShadcnTheme) {
    THEME.with(|c| *c.borrow_mut() = t);
}
impl Default for ShadcnTheme {
    fn default() -> Self {
        Self::light()
    }
}
impl ShadcnTheme {
    #[must_use]
    /// The light.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            background: oklch(1.0, 0., 0.),
            foreground: oklch(0.145, 0., 0.),
            card: oklch(1.0, 0., 0.),
            card_foreground: oklch(0.145, 0., 0.),
            popover: oklch(1.0, 0., 0.),
            popover_foreground: oklch(0.145, 0., 0.),
            primary: oklch(0.205, 0., 0.),
            primary_foreground: oklch(0.985, 0., 0.),
            secondary: oklch(0.97, 0., 0.),
            secondary_foreground: oklch(0.205, 0., 0.),
            muted: oklch(0.97, 0., 0.),
            muted_foreground: oklch(0.556, 0., 0.),
            accent: oklch(0.97, 0., 0.),
            accent_foreground: oklch(0.205, 0., 0.),
            destructive: oklch(0.577, 0.245, 27.325),
            destructive_foreground: oklch(0.985, 0., 0.),
            border: oklch(0.922, 0., 0.),
            input: oklch(0.922, 0., 0.),
            ring: oklch(0.708, 0., 0.),
            radius: 6.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
        }
    }
    #[must_use]
    /// The dark.
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            background: oklch(0.145, 0., 0.),
            foreground: oklch(0.985, 0., 0.),
            card: oklch(0.205, 0., 0.),
            card_foreground: oklch(0.985, 0., 0.),
            popover: oklch(0.205, 0., 0.),
            popover_foreground: oklch(0.985, 0., 0.),
            primary: oklch(0.922, 0., 0.),
            primary_foreground: oklch(0.205, 0., 0.),
            secondary: oklch(0.269, 0., 0.),
            secondary_foreground: oklch(0.985, 0., 0.),
            muted: oklch(0.269, 0., 0.),
            muted_foreground: oklch(0.708, 0., 0.),
            accent: oklch(0.269, 0., 0.),
            accent_foreground: oklch(0.985, 0., 0.),
            destructive: oklch(0.704, 0.191, 22.216),
            destructive_foreground: oklch(0.985, 0., 0.),
            border: oklcha(1.0, 0., 0., 0.1),
            input: oklcha(1.0, 0., 0., 0.1),
            ring: oklch(0.556, 0., 0.),
            radius: 6.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
        }
    }
    fn adj(color: Color, delta: f32) -> Color {
        let ok: AlphaColor<Oklch> = color.convert();
        let [lightness, chroma, hue, alpha] = ok.components;
        AlphaColor::<Oklch>::new([(lightness + delta).clamp(0., 1.), chroma, hue, alpha])
            .convert::<Srgb>()
    }
    #[must_use]
    /// The darken.
    pub fn darken(&self, c: Color) -> Color {
        Self::adj(c, -0.05)
    }
    #[must_use]
    /// The lighten.
    pub fn lighten(&self, c: Color) -> Color {
        Self::adj(c, 0.05)
    }
    #[must_use]
    /// The hover color.
    pub fn hover_color(&self, c: Color) -> Color {
        match self.mode {
            ThemeMode::Light => self.darken(c),
            ThemeMode::Dark => self.lighten(c),
        }
    }
    #[must_use]
    /// The active color.
    pub fn active_color(&self, c: Color) -> Color {
        match self.mode {
            ThemeMode::Light => Self::adj(c, -0.10),
            ThemeMode::Dark => Self::adj(c, 0.10),
        }
    }
}
fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
impl StylePropValue for ShadcnTheme {
    fn interpolate(&self, o: &Self, val: f64) -> Option<Self> {
        let t = val.to_f32().unwrap_or(0.0);
        Some(Self {
            mode: if t < 0.5 { self.mode } else { o.mode },
            background: self
                .background
                .lerp(o.background, t, HueDirection::default()),
            foreground: self
                .foreground
                .lerp(o.foreground, t, HueDirection::default()),
            card: self.card.lerp(o.card, t, HueDirection::default()),
            card_foreground: self.card_foreground.lerp(
                o.card_foreground,
                t,
                HueDirection::default(),
            ),
            popover: self.popover.lerp(o.popover, t, HueDirection::default()),
            popover_foreground: self.popover_foreground.lerp(
                o.popover_foreground,
                t,
                HueDirection::default(),
            ),
            primary: self.primary.lerp(o.primary, t, HueDirection::default()),
            primary_foreground: self.primary_foreground.lerp(
                o.primary_foreground,
                t,
                HueDirection::default(),
            ),
            secondary: self.secondary.lerp(o.secondary, t, HueDirection::default()),
            secondary_foreground: self.secondary_foreground.lerp(
                o.secondary_foreground,
                t,
                HueDirection::default(),
            ),
            muted: self.muted.lerp(o.muted, t, HueDirection::default()),
            muted_foreground: self.muted_foreground.lerp(
                o.muted_foreground,
                t,
                HueDirection::default(),
            ),
            accent: self.accent.lerp(o.accent, t, HueDirection::default()),
            accent_foreground: self.accent_foreground.lerp(
                o.accent_foreground,
                t,
                HueDirection::default(),
            ),
            destructive: self
                .destructive
                .lerp(o.destructive, t, HueDirection::default()),
            destructive_foreground: self.destructive_foreground.lerp(
                o.destructive_foreground,
                t,
                HueDirection::default(),
            ),
            border: self.border.lerp(o.border, t, HueDirection::default()),
            input: self.input.lerp(o.input, t, HueDirection::default()),
            ring: self.ring.lerp(o.ring, t, HueDirection::default()),
            radius: lerp_f32(self.radius, o.radius, t),
            radius_sm: lerp_f32(self.radius_sm, o.radius_sm, t),
            radius_md: lerp_f32(self.radius_md, o.radius_md, t),
            radius_lg: lerp_f32(self.radius_lg, o.radius_lg, t),
        })
    }
}
/// The `ShadcnThemeExt`.
pub trait ShadcnThemeExt {
    #[must_use]
    /// Configures shadcn theme.
    fn with_shadcn_theme(
        self,
        f: impl Fn(floem::style::Style, &ShadcnTheme) -> floem::style::Style + 'static,
    ) -> Self
    where
        Self: Sized;
}
impl ShadcnThemeExt for Style {
    fn with_shadcn_theme(
        self,
        f: impl Fn(floem::style::Style, &ShadcnTheme) -> floem::style::Style + 'static,
    ) -> Self {
        let t = current_theme();
        f(self, &t)
    }
}
