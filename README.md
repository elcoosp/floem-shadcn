# floem-shadcn

**The complete shadcn/ui component library, ported to Floem.**

<p>
  <strong>Every component, every variant, every sub-part — typed, documented, and rendered natively on the GPU.</strong><br/>
  A single-crate port that ships <strong>290 exported types across 55 modules</strong>, mapping the full shadcn/ui catalog to Floem views. No web stack, no Electron, no CSS-in-JS. Just Rust, a 60 fps layout engine, and a theming system built on OKLCH so a light/dark toggle actually looks designed on both sides.
</p>

<p>
  <img src="https://img.shields.io/badge/Rust-1.99%2B%20%7C%202024-000000?style=flat-square&logo=rust" alt="Rust"/>
  <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License MIT"/>
  <img src="https://img.shields.io/badge/Components-55-6F4E37?style=flat-square" alt="Components"/>
  <img src="https://img.shields.io/badge/Exported%20types-290-8B4513?style=flat-square" alt="Exported types"/>
  <img src="https://img.shields.io/badge/shadcn%2Fui%20coverage-100%25-00BFFF?style=flat-square" alt="shadcn/ui coverage"/>
  <img src="https://img.shields.io/badge/Renderer-Floem%20%2F%20GPU-6A0DAD?style=flat-square" alt="Renderer"/>
  <img src="https://img.shields.io/badge/Styling-floem--tailwind-4B32C3?style=flat-square" alt="Styling"/>
  <img src="https://img.shields.io/badge/Theme-OKLCH%20Light%20%2B%20Dark-FF4500?style=flat-square" alt="Theme"/>
  <img src="https://img.shields.io/badge/Tests-128%20%2B%2046%20doc-228B22?style=flat-square" alt="Tests"/>
  <img src="https://img.shields.io/badge/Clippy-pedantic%20zero%20warnings-8A2BE2?style=flat-square&logo=rust" alt="Clippy pedantic"/>
  <img src="https://img.shields.io/badge/unsafe-forbidden-CB3837?style=flat-square&logo=rust" alt="unsafe forbidden"/>
</p>
</div>

---

> [!NOTE]
> floem-shadcn tracks the **shadcn/ui v4** component catalog. The API is stabilising but some components intentionally simplify where the web original leans on browser features (CSS container queries, hover-tracked overlays, SVG sub-charts). See [Project Status](#project-status) for what is fully faithful versus where the port has made a deliberate trade-off.

---

## Table of Contents

- [Why floem-shadcn](#why-floem-shadcn)
- [Features](#features)
- [Architecture](#architecture)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Components](#components)
- [Theming](#theming)
- [Development](#development)
- [Testing](#testing)
- [Project Status](#project-status)
- [License](#license)

---

## Why floem-shadcn

The shadcn/ui design language is the reference point for modern application chrome: soft surfaces, restrained borders, deliberate radii, and a two-tone palette that reads clearly at every scale. It is also, today, a React + Tailwind + Radix stack — which means every project that wants that design language pays for a web runtime, a DOM, and a JavaScript toolchain.

floem-shadcn is the opposite bet:

- **The component set is the contract, not the implementation.** 55 modules and 290 exported types mirror the shadcn/ui catalog one-to-one. If a component exists in shadcn/ui, it exists here — with its sub-parts, its variants, and its sub-variants.
- **Rendering is native.** Floem paints through a GPU-accelerated backend (vger / skia on desktop), so a `Chart`, a `Sidebar`, or a 10 000-row `Table` scrolls at frame rate without a layout thrash that a DOM would have caused.
- **Styling is typed.** `floem-tailwind` gives you `w_full()`, `gap_2()`, `text_sm()`, `rounded_md()` on `Style` — the same vocabulary as the shadcn/ui CSS, checked at compile time. There is no string class list to get wrong.
- **The theme is designed, not extracted.** Colors are stored as `AlphaColor<Oklch>`, so `light()` and `dark()` are hand-tuned pairs, and `hover_color()` / `active_color()` shift lightness by a fixed delta rather than a flat alpha. A toggle between the two does not produce the "slightly wrong" intermediate that hex-pair themes do.
- **The components are testable.** Floem ships a headless backend (`floem-test`) with a `HeadlessHarness` that dispatches real clicks, tracks layout, and asserts geometry. The 128-test suite includes interaction tests that click a `Dialog` trigger, click the backdrop, and assert the dialog actually closes — not a synthetic event bus.
- **Zero `unsafe`, zero `allow`.** `unsafe_code = "forbid"` at the workspace level, `missing_docs = "warn"`, and `clippy::pedantic = "warn"` with **no `#[allow(...)]` anywhere**. Every public item carries a doc comment because the compiler says it must.

If you are writing a Rust desktop app and want the shadcn/ui look without the web stack, floem-shadcn is the shortest path.

---

## Features

### Component coverage

- **All 55 official shadcn/ui modules** — Accordion, Alert, Alert Dialog, Aspect Ratio, Avatar, Badge, Breadcrumb, Button, Button Group, Calendar, Card, Carousel, Chart, Checkbox, Collapsible, Combobox, Command, Context Menu, Data Table, Date Picker, Dialog, Drawer, Dropdown Menu, Empty, Field (formerly Form), Hover Card, Input, Input Group, Input OTP, Item, Kbd, Label, Menubar, Navigation Menu, Pagination, Popover, Progress, Radio Group, Resizable, Scroll Area, Select, Separator, Sheet, Sidebar, Skeleton, Slider, Sonner / Toast, Spinner, Switch, Table, Tabs, Textarea, Toggle, Toggle Group, Tooltip, Typography.
- **Full sub-part decomposition** — `AlertDialog` ships as `AlertDialog + Trigger/Content/Header/Footer/Title/Description/Action/Cancel`, `Sidebar` as `Sidebar + Header/Content/Group/Menu/MenuItem/MenuButton/MenuBadge/MenuAction/MenuSkeleton/MenuSub/MenuSubItem/MenuSubButton/Rail/Inset/Trigger/Footer/Separator`. Nothing is a one-shot opaque widget.
- **Variant and size enums where shadcn/ui has them** — `ButtonVariant` (6 variants) × `ButtonSize` (4 sizes), `BadgeVariant` (4), `AlertVariant` (2), `ToggleVariant` (2) × `ToggleSize` (3), `SheetSide` (4), `DrawerSide` (4), `PopoverSide` (4) × `PopoverAlign` (3), `HoverCardSide` (4) × `HoverCardAlign` (3), `CarouselOrientation` (2), `ScrollOrientation` (3), `SeparatorOrientation` (2), `ToastVariant` (3), `SpinnerSize` (3) × `SpinnerColor` (4), `FieldOrientation` (3), `FieldLegendVariant` (2).

### Theming

- **OKLCH color system** — 20 semantic theme tokens (`background`, `foreground`, `card`, `card_foreground`, `popover`, `popover_foreground`, `primary`, `primary_foreground`, `secondary`, `secondary_foreground`, `muted`, `muted_foreground`, `accent`, `accent_foreground`, `destructive`, `destructive_foreground`, `border`, `input`, `ring`, plus the four radii) exposed as typed fields on `ShadcnTheme`.
- **Light and dark, hand-tuned** — `ShadcnTheme::light()` and `ShadcnTheme::dark()` are authored side by side. `ShadcnTheme::interpolate` drives a smooth cross-fade for theme transitions.
- **Derived shades** — `ShadcnTheme::darken`, `lighten`, `hover_color`, `active_color` return consistently shifted variants so hover and pressed states read as "the same color, more weight", not "a slightly different hex".
- **Style extension traits** — `ShadcnStyleExt` provides `bg_primary()`, `text_muted_foreground()`, `border_border()`, `rounded_radius_md()` and 40 more shadcn-token accessors directly on `Style`.
- **Live theme switching** — `set_theme(ShadcnTheme::dark())` re-renders the whole tree through the `with_shadcn_theme` reactive style closure.

### Developer experience

- **Typed builder API** — every component is a builder with `#[must_use]` on the mutating methods, so an accidental dropped return is a compiler warning, not a silently-styled default.
- **`ParentView` composition** — containers (`Card`, `Item`, `Field`, `Sidebar*`, `Table*`, `Empty*`, `CommandList`, `ComboboxContent`) accept `.child(view)` and `.children(iter)` through the standard Floem `ParentView` trait.
- **Stable `ViewId`** — every component exposes `view_id()` through `HasViewId`, and the id is attached to the *rendered* view (`Label::with_id`, `Container::with_id`), so layout queries return the real node and not a detached orphan.
- **Reactive signal accessors** — `Dialog::open_signal`, `Tabs::active_signal`, `RadioGroup::selected_signal`, `ToggleGroup::selected_signal`, `Command::search_signal`, `DateRangePicker::start_signal` / `end_signal` — so callers can drive the internal state from outside without reconstructing the view.
- **Documented sub-parts** — `missing_docs` is enforced, so every `AlertDialogCancel::dialog(signal)` carries a doc comment explaining what it does.

### Testability

- **Headless interaction tests** — `HeadlessHarness::new_with_size(TestRoot::new(), view, w, h)` builds a real view tree, runs a real layout pass, and dispatches real `Click` / `SecondaryClick` events. Every interactive component is covered this way (dialog open/close/backdrop, select click, sidebar active state, kbd group, tabs distribution, button centering, chart layout).
- **Layout assertions** — `id.get_layout()` returns the computed `Layout`, so tests can assert `size.width > 100.0`, `center_y1 ≈ center_y2`, `total_tab_width ≈ available_width`, etc.
- **Doc tests on every example** — 46 `no_run` and executable doc tests exercise the public examples in `lib.rs`, `button.rs`, `chart.rs`, `field.rs`, `sidebar.rs`, and more.

---

## Architecture

floem-shadcn is a **single crate** with a **flat module structure** under `src/components/`. Each component lives in its own file; the crate root re-exports them through `prelude`.

```
floem-shadcn/
├── src/
│   ├── lib.rs                # Crate root, prelude re-exports
│   ├── theme.rs              # ShadcnTheme, ShadcnThemeExt, light()/dark()
│   ├── styled.rs             # ShadcnStyleExt: 40+ token accessors
│   └── components/
│       ├── mod.rs            # 55 module declarations
│       ├── accordion.rs      # Accordion, AccordionItem, AccordionTrigger, AccordionContent
│       ├── alert.rs          # Alert, AlertVariant
│       ├── alert_dialog.rs   # AlertDialog + Trigger/Content/Header/Footer/Title/Description/Action/Cancel
│       ├── aspect_ratio.rs   # AspectRatio, AspectRatioSimple
│       ├── avatar.rs         # Avatar
│       ├── badge.rs          # Badge, BadgeVariant
│       ├── breadcrumb.rs     # Breadcrumb + List/Item/Link/Page/Separator/Ellipsis
│       ├── button.rs         # Button, ButtonVariant, ButtonSize
│       ├── button_group.rs   # ButtonGroup, Separator, Text
│       ├── calendar.rs       # Calendar, CalendarSimple, SimpleDate
│       ├── card.rs           # Card + Header/Title/Description/Content/Footer/Action
│       ├── carousel.rs       # Carousel, Content/Item/Previous/Next, Orientation
│       ├── chart.rs          # Chart, ChartData/Series/Point, Legend, Tooltip
│       ├── checkbox.rs       # Checkbox
│       ├── collapsible.rs    # Collapsible, Trigger, Content
│       ├── combobox.rs       # Combobox + List/Item/Group/Input/Trigger/Label/Empty/Separator/Context
│       ├── command.rs        # Command, Input/List/Group/Item/Shortcut/Separator/Empty
│       ├── context_menu.rs   # ContextMenu + Content/Item/Group/Label/Separator
│       ├── date_picker.rs    # DatePicker, DateRangePicker
│       ├── dialog.rs         # Dialog + Trigger/Content/Header/Footer/Title/Description/Close
│       ├── drawer.rs         # Drawer + Content/Header/Footer/Title/Description/Close, DrawerSide
│       ├── dropdown_menu.rs  # DropdownMenu + Content/Item/Label/Group/Shortcut/Separator
│       ├── empty.rs          # Empty + Header/Media/Title/Description/Content/Footer/Actions
│       ├── field.rs          # Field + Label/Description/Error/Group/Content/Action/Set/Legend/Title/Separator/Orientation
│       ├── hover_card.rs     # HoverCard + Trigger/Content, Side/Align
│       ├── input.rs          # Input
│       ├── input_group.rs    # InputGroup, InputGroupAddon
│       ├── input_otp.rs      # InputOTP, Slot, Group, Separator; PinInput
│       ├── item.rs           # Item + Media/Content/Actions/Header/Footer/Title/Description/Group/Separator
│       ├── kbd.rs            # Kbd, KbdGroup
│       ├── label.rs          # Label, LabelWithIcon, FormLabel, FormField
│       ├── menubar.rs        # Menubar + Menu/Trigger/Content/Item/CheckboxItem/Shortcut/Separator
│       ├── navigation_menu.rs # NavigationMenu + List/Item/Trigger/Content/Link/Indicator/Viewport
│       ├── pagination.rs     # Pagination + Content/Item/Link/Previous/Next/Ellipsis
│       ├── popover.rs        # Popover + Trigger/Content/Title/Description/Header/Close, Side/Align
│       ├── progress.rs       # Progress
│       ├── radio_group.rs    # RadioGroup, RadioGroupItem
│       ├── resizable.rs      # ResizablePanelGroup/Panel/Handle, Direction
│       ├── scroll_area.rs    # ScrollArea, ScrollAreaWithBar, ScrollOrientation
│       ├── select.rs         # Select + Trigger/Content/Item/Group/Label/Separator, SelectItemData
│       ├── separator.rs      # Separator, Orientation
│       ├── sheet.rs          # Sheet + Content/Header/Footer/Title/Description/Close
│       ├── sidebar.rs        # Sidebar + 16 sub-parts, style cache
│       ├── skeleton.rs       # Skeleton
│       ├── slider.rs         # Slider
│       ├── spinner.rs        # Spinner, SpinnerSize, SpinnerColor
│       ├── switch.rs         # Switch
│       ├── table.rs          # Table + Header/Body/Row/Head/Cell/Footer/Caption
│       ├── tabs.rs           # Tabs, TabsList, Tab, TabsContent, styled cache
│       ├── textarea.rs       # Textarea
│       ├── toast.rs          # Toast + variants, ToastAction
│       ├── toggle.rs         # Toggle, ToggleVariant, ToggleSize
│       ├── toggle_group.rs   # ToggleGroup, ToggleGroupItem(+Multiple)
│       ├── tooltip.rs        # Tooltip, TooltipExt
│       └── typography.rs     # H1–H4, P, Lead, Muted, Code, Blockquote, List
├── examples/
│   ├── showcase.rs           # Full component gallery with sidebar nav
│   └── combobox_test.rs      # Focused combobox example
├── tests/                    # 17 integration suites (button, tabs, dialog, chart, field, ...)
└── fonts/                    # DejaVu Serif (default text face)
```

### Data flow

```
User code
   │
   ▼
┌────────────────┐   ┌────────────────┐   ┌────────────────┐
│  Component     │──▶│  ParentView    │──▶│  Floem view    │
│  builder       │   │  .child(...)   │   │  tree          │
└────────────────┘   └────────────────┘   └────────────────┘
        │                                          │
        ▼                                          ▼
┌────────────────┐   ┌────────────────┐   ┌────────────────┐
│  with_shadcn_  │──▶│  Style         │──▶│  Floem layout  │
│  theme(…)      │   │  (tailwind)    │   │  + paint       │
└────────────────┘   └────────────────┘   └────────────────┘
                                                   │
                                                   ▼
                                          ┌────────────────┐
                                          │  GPU (vger /   │
                                          │  skia)         │
                                          └────────────────┘
```

Three layers:

1. **Builders** convert your typed calls into concrete Floem views.
2. **`ParentView`** stitches sub-parts into a tree with the ids you passed.
3. **`with_shadcn_theme`** evaluates the theme at render time and returns a fully styled `Style`.

Themes are read from a thread-local `RefCell<ShadcnTheme>`, so `set_theme` is a one-line swap that propagates on the next layout tick without a full tree rebuild.

See [`COMPONENTS.md`](COMPONENTS.md) for the complete per-component coverage matrix (sub-parts ported, variants, sizes).

---

## Getting Started

### Prerequisites

- **Rust 1.99 or newer** (`rustup toolchain install 1.99`). The crate uses the **2024 edition**.
- **A working Floem dependency chain** — `floem` and `floem-tailwind` are declared as `git = "https://github.com/lapce/floem"` and `path = "../floem-tailwind"` respectively, so the first build fetches them from source.
- **System libraries for Floem** — on Linux you need `libgtk-3-dev`, `libglib2.0-dev`, `libcairo2-dev`, `libpango1.0-dev`, `libxkbcommon-dev`, `libwayland-dev`, and the `libxcb-*` family (see `.github/workflows/ci.yml` for the exact list). macOS and Windows build out of the box.

### As a dependency

```toml
# Cargo.toml
[dependencies]
floem-shadcn = { path = "../floem-shadcn" }
floem        = { git = "https://github.com/lapce/floem" }
```

### First view

```rust
use floem::prelude::*;
use floem_shadcn::prelude::*;
use floem_shadcn::theme::{ShadcnTheme, ShadcnThemeExt};

fn app_view() -> impl IntoView {
    Stack::vertical((
        Button::new("Save").primary(),
        Button::new("Delete").destructive().sm(),
        Button::new("Cancel").outline(),
    ))
    .style(|s| s.flex_col().gap_3().p_6())
}

fn main() {
    floem_shadcn::theme::set_theme(ShadcnTheme::light());
    floem::launch(app_view);
}
```

### Run the showcase

The showcase example exercises every component in the library, with a `Sidebar` nav, a theme toggle, and a per-component demo section:

```bash
cargo run --example showcase
```

`cargo run --example combobox_test` is a focused, minimal example for the `Combobox`.

### Prerequisites checklist

```bash
# Verify the toolchain
rustc --version           # >= 1.99
cargo --version

# Build the library
cargo build

# Run the test suite
cargo test

# Run the showcase
cargo run --example showcase
```

---

## Usage

### Composing a Dialog

Every modal component separates its state (an `RwSignal<bool>`) from its parts. `DialogTrigger` and `DialogClose` read the signal from the reactive `Context`, so composing is just nesting:

```rust
use floem::prelude::*;
use floem_shadcn::prelude::*;

Dialog::new((
    DialogTrigger::new(Button::new("Rename")),
    DialogContent::new((
        DialogHeader::new()
            .title("Rename file")
            .description("Choose a new name for this file."),
        DialogFooter::new((
            DialogClose::new(Button::new("Cancel").outline()),
            Button::new("Save").primary(),
        )),
    )),
))
```

The trigger and close buttons dispatch into the `DialogContext` automatically. If you need to open the dialog programmatically, keep a handle:

```rust
let dialog = Dialog::new((/* … */));
let open = dialog.open_signal();
// later:
open.set(true);
```

### Building a form with `Field`

`Field` is the shadcn/ui v4 form primitive. It composes label, description, error, and control into a single accessible row:

```rust
use floem::view::ParentView;
use floem_shadcn::components::field::*;

Field::new()
    .orientation(FieldOrientation::Vertical)
    .child(FieldLabel::new("Email address"))
    .child(
        FieldContent::new()
            .child(FieldDescription::new("We will never share it."))
            .child(FieldError::new("This address is already in use.")),
    );
```

To render a variable-length list of errors, use `FieldError::from_errors(vec![…])` — it returns `None` when the list is empty, matching the shadcn/ui "no errors, no DOM" behaviour.

### Rendering a chart

`Chart` ports the shadcn/ui chart *shape* (title, description, legend, axes, bars, tooltip) without depending on `recharts`. Data is a plain `ChartData` tree; the palette is the standard six-color `--chart-1`–`--chart-6` set:

```rust
use floem_shadcn::components::chart::*;

let data = ChartData::new()
    .title("Monthly revenue")
    .description("Desktop vs mobile")
    .series(ChartSeries::new("Desktop", vec![
        ChartPoint::new("Jan", 186.0),
        ChartPoint::new("Feb", 305.0),
        ChartPoint::new("Mar", 237.0),
    ]))
    .series(ChartSeries::new("Mobile", vec![
        ChartPoint::new("Jan", 80.0),
        ChartPoint::new("Feb", 200.0),
    ]));

Chart::new(data).height(240.0).legend(true).grid(true);
```

Bars highlight on hover via Floem's `hover` style selector; the tooltip content is available as `ChartTooltip`, and `ChartLegend` renders the color swatch row. Layout math (`bar_height`, `bar_width`, `chart_colors`, `series_color`) is exposed as public functions so you can drive your own renderer if you want.

### Building a sidebar layout

```rust
use floem::reactive::RwSignal;
use floem_shadcn::components::sidebar::*;

let active = RwSignal::new("home".to_string());

Sidebar::new()
    .header(SidebarHeader::new().child(Label::derived(|| "My App")))
    .content(
        SidebarContent::new().child(
            SidebarGroup::new().child(
                SidebarGroupContent::new().child(
                    SidebarMenu::new()
                        .child(SidebarMenuItem::new().child(
                            SidebarMenuButton::with_label("Home")
                                .is_active(move || active.get() == "home"),
                        ))
                        .child(SidebarMenuItem::new().child(
                            SidebarMenuButton::with_label("Settings")
                                .is_active(move || active.get() == "settings"),
                        )),
                ),
            ),
        ),
    )
    .footer(SidebarFooter::new().child(Label::derived(|| "v0.1.0")));
```

`SidebarMenuButton::new_empty()` accepts `.child(…)` for icon + label rows; `SidebarMenuButton::with_label(…)` is the shorthand for a plain-text label.

### Theming at runtime

Themes are swapped with a single function call and propagate through `with_shadcn_theme` closures on the next render:

```rust
use floem_shadcn::theme::{set_theme, ShadcnTheme, ThemeMode};

let mode = RwSignal::new(ThemeMode::Light);
mode.get();
set_theme(match mode.get() {
    ThemeMode::Light => ShadcnTheme::light(),
    ThemeMode::Dark  => ShadcnTheme::dark(),
});
```

The showcase example wires this to a "Toggle Theme" button that flips the signal.

---

## Components

The full coverage matrix lives in [`COMPONENTS.md`](COMPONENTS.md). Quick reference:

| Family | Modules |
| --- | --- |
| **Layout & structure** | Card, Separator, Aspect Ratio, Resizable, Scroll Area, Sheet, Drawer |
| **Navigation** | Sidebar, Breadcrumb, Menubar, Navigation Menu, Pagination, Tabs |
| **Forms & inputs** | Input, Input Group, Input OTP, Textarea, Checkbox, Switch, Slider, Radio Group, Select, Combobox, Calendar, Date Picker, Field, Label |
| **Overlays & dialogs** | Dialog, Alert Dialog, Popover, Hover Card, Context Menu, Dropdown Menu, Tooltip, Command |
| **Feedback & status** | Alert, Progress, Skeleton, Spinner, Toast / Sonner |
| **Data display** | Table, Chart, Item, Empty, Avatar, Badge, Kbd |
| **Typography** | H1–H4, P, Lead, Muted, Code, Blockquote, List |
| **Actions** | Button, Button Group, Toggle, Toggle Group, Collapsible, Accordion |

Every module is available through `floem_shadcn::components::<module>` and, for the most common types, through `floem_shadcn::prelude::*`.

---

## Theming

### The `ShadcnTheme` struct

Twenty semantic tokens, four radii, one `mode` discriminant. The fields match the shadcn/ui CSS custom properties one-to-one, so a Tailwind theme you already know maps directly:

```rust
pub struct ShadcnTheme {
    pub mode:               ThemeMode,   // Light | Dark
    pub background:         Color,
    pub foreground:         Color,
    pub card:               Color,
    pub card_foreground:    Color,
    pub popover:            Color,
    pub popover_foreground: Color,
    pub primary:            Color,
    pub primary_foreground: Color,
    pub secondary:          Color,
    pub secondary_foreground: Color,
    pub muted:              Color,
    pub muted_foreground:   Color,
    pub accent:             Color,
    pub accent_foreground:  Color,
    pub destructive:        Color,
    pub destructive_foreground: Color,
    pub border:             Color,
    pub input:              Color,
    pub ring:               Color,
    pub radius:    f32,   // 6.0
    pub radius_sm: f32,   // 4.0
    pub radius_md: f32,   // 6.0
    pub radius_lg: f32,   // 8.0
}
```

### Reading the current theme

Inside a `.style(|s| …)` closure, use `with_shadcn_theme`:

```rust
Label::new("Hello").style(|s| {
    s.with_shadcn_theme(|s, t| s.color(t.foreground).background(t.background))
})
```

Outside a style closure, `current_theme()` returns a copy:

```rust
use floem_shadcn::theme::current_theme;

let t = current_theme();
let primary = t.primary;
```

### Style extension trait

`ShadcnStyleExt` puts the 40 token accessors directly on `Style`:

```rust
use floem_shadcn::styled::ShadcnStyleExt;

my_view.style(|s| {
    s.bg_primary()
        .text_primary_foreground()
        .border_border()
        .rounded_radius_md()
})
```

### Derived shades

```rust
t.hover_color(t.primary)   // Light: darken(0.05), Dark: lighten(0.05)
t.active_color(t.primary)  // Light: adj(-0.10),   Dark: adj(+0.10)
```

Both functions preserve chroma and hue, so a hover state reads as "same color, more weight" on both sides of the toggle.

---

## Development

```bash
# Build the crate
cargo build

# Build the showcase
cargo build --example showcase

# Run the full test suite (128 tests + 46 doc tests)
cargo test

# Run only a single suite
cargo test --test chart
cargo test --test field
cargo test --test dialog

# Format
cargo fmt --all

# Lint under the workspace's pedantic policy
cargo clippy --all-targets -- -D warnings

# Documentation
cargo doc --no-deps --open
```

A `justfile` is provided with shortcuts: `just build`, `just test`, `just lint`, `just check`, `just run`.

### Lint policy

The workspace enforces:

- `unsafe_code = "forbid"`
- `missing_docs = "warn"`
- `clippy::pedantic = "warn"` (all pedantic lints)
- `-D warnings` in CI

**There is no `#[allow(...)]` in the codebase.** If `cargo clippy --all-targets -- -D warnings` is green, pedantic and missing-docs are also green.

### Adding a new component

1. Create `src/components/<name>.rs` with the builder structs.
2. Register it in `src/components/mod.rs`.
3. Add the public types to `src/prelude` in `src/lib.rs`.
4. Add a headless interaction test in `tests/<name>.rs`.
5. Update `COMPONENTS.md`.
6. Run `cargo clippy --all-targets -- -D warnings` and `cargo test`.

The checklist exists because each step catches a class of regression (missing docs, missing prelude export, missing interaction coverage).

---

## Testing

The crate ships **128 unit and integration tests** across **17 suites**, plus **46 doc tests** exercised by `cargo test --doc`.

| Suite | What it covers |
| --- | --- |
| `tests/button.rs` | Button sizing, variant layout, centering in horizontal stacks, intrinsic width in vertical stacks, showcase grid behaviour. |
| `tests/button_group.rs` | Button group composition, separator alignment. |
| `tests/chart.rs` | Data construction, empty data, `bar_height` normalization and NaN/Infinity guards, `bar_width` division guard, palette assignment and wrapping, headless layout for `Chart`, `ChartLegend`, `ChartTooltip`. |
| `tests/dialog.rs` | Trigger opens dialog, close button closes, backdrop click closes, `DialogContext` propagation, multiple dialogs, dialog inside `Scroll`, `dyn_container`. |
| `tests/empty.rs` | `Empty` layout with header/media/content/footer/actions. |
| `tests/field.rs` | Orientation defaults, `FieldError::from_errors` empty/single/multiple, sub-part construction, populated `Field` layout. |
| `tests/input_group.rs` | Addon position and alignment. |
| `tests/item.rs` | Item + sub-part layout. |
| `tests/kbd.rs` | `Kbd` and `KbdGroup` composition. |
| `tests/select.rs` | Click interaction, dropdown position, trigger offset. |
| `tests/sidebar_style_cache.rs` | `SidebarMenuButton::is_active` reactivity (style cache regression). |
| `tests/slider.rs` | Slider geometry. |
| `tests/spinner.rs` | Spinner size × color. |
| `tests/subparts.rs` | Canonical sub-parts across `Card`, `ButtonGroup`, `Empty`, `Item`, `Accordion`, `Popover`, `Sidebar`. |
| `tests/tabs.rs` | `flex_grow` equal distribution across 2, 3, and 4 tabs. |
| `tests/typography.rs` | H1–H4, P, Lead, Muted, Code, Blockquote, List. |

To see the exact current total:

```bash
cargo test --workspace -- --list | wc -l
```

To run a single interaction test:

```bash
cargo test --test dialog test_dialog_backdrop_click_closes -- --nocapture
```

To see headless layout output during a failing test, add `-- --nocapture` and the `eprintln!` diagnostics (already present in `tests/button.rs` and `tests/tabs.rs`) will print.

---

## Project Status

### Fully implemented

- **All 55 components** with their sub-parts, variants, and sizes registered in `src/components/mod.rs` and re-exported through the prelude.
- **Theming** — `ShadcnTheme::light()` / `::dark()`, `with_shadcn_theme`, `ShadcnStyleExt`, and `set_theme` for runtime swap.
- **Headless interaction tests** — every interactive component has at least one click-driven test.
- **Zero `unsafe`, zero `#[allow]`, pedantic-clippy-clean** at the workspace level.
- **`missing_docs` green** — every public item documented.
- **Floem `ParentView` composition** across all container components (`Card`, `Item`, `Field`, `Sidebar*`, `Table*`, `Empty*`, `CommandList`, `ComboboxContent`, `CommandList`, `PopoverHeader`).
- **Stable `ViewId`** attached to the rendered view, so `view_id().get_layout()` works for every component.

### Deliberate simplifications vs the web original

- **`Chart`** renders bars with Floem flex views, not SVG. `ChartTooltip` is a static `series: value` box — Floem has no pointer-tracked overlay primitive for arbitrary views, so a "follow the cursor" tooltip is out of scope. Compose it with `TooltipExt` if you need one.
- **`Field` `Responsive` orientation** stacks vertically in Floem (no CSS container queries). It exists for API parity with shadcn/ui v4.
- **`DatePicker`** uses the `Calendar` component; range selection on the calendar itself is not implemented (use two `DatePicker`s, one per endpoint).
- **`InputOTP`** renders 4 or 6 slots depending on `max_length`; other lengths fall back to six. Dynamic slot counts from `length` are not supported.
- **Animations** are limited to what Floem's `Transition` API exposes — `Background` transitions on hover, and theme cross-fades. There is no keyframe system.
- **Placeholder components** — `SelectItem`, `SelectLabel`, `SelectSeparator`, `SelectGroup` construct to satisfy the shadcn/ui API surface but render `Empty::new()`. The real `Select` (with items in `.items(Vec<SelectItemData>)`) is the working path.
- **`ScrollAreaWithBar`** does not yet paint a custom scrollbar; it uses Floem's native scroll widget with an optional border.
- **`NavigationMenuItem::content`** stores content but `NavigationMenuViewport` renders it inline rather than through a positioned popover; a floating-content follow-up is planned.

### Tracked gaps

- **Theme persistence** — no built-in read/write of the selected `ThemeMode` to disk. Call `set_theme` in your `main` and persist it yourself.
- **Icon set** — components that use icons inline `svg` for a small set (chevron, check, panel-left). A configurable icon provider is a follow-up.
- **RTL layout** — Floem's layout engine is LTR-only; RTL support depends on an upstream change.
- **Accessibility metadata** — components do not yet emit ARIA-equivalent roles; Floem's accessibility tree is in progress upstream.
- **`DataTable`** is the `Table` component with sticky headers; server-side pagination / sorting / filtering are your responsibility.

> [!WARNING]
> floem-shadcn tracks shadcn/ui at the *component level*, not the *behavioural level*. Where the web original leans on a browser feature Floem does not expose (container queries, hover-tracked overlays, pointer-events passthrough, CSS animations), the port either simplifies or omits it and documents that choice above. Expect a small, honest gap between the two catalogs.

---

## License

MIT — see [`LICENSE`](LICENSE) for the full text. By contributing, you agree that your contributions will be licensed under the same terms.

---

<p align="center">
  <em>floem-shadcn is a work in progress. Contributions, bug reports, and design discussions are welcome.</em>
</p>
