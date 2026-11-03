# Component Manifest

floem-shadcn is a complete port of the shadcn/ui component library to floem.
This manifest maps every official shadcn/ui component to its implementation
here. 290 exported types (components, subparts, variants) across 55 modules.

## Coverage matrix (official shadcn/ui ↔ floem-shadcn)

| shadcn/ui | floem-shadcn module | Subparts ported |
| --- | --- | --- |
| Accordion | `accordion` | Accordion, AccordionItem, AccordionTrigger, AccordionContent |
| Alert | `alert` | Alert (default/destructive variants), AlertVariant |
| Alert Dialog | `alert_dialog` | AlertDialog + Trigger/Content/Header/Footer/Title/Description/Action/Cancel |
| Aspect Ratio | `aspect_ratio` | AspectRatio, AspectRatioSimple |
| Avatar | `avatar` | Avatar |
| Badge | `badge` | Badge (default/secondary/destructive/outline) |
| Breadcrumb | `breadcrumb` | Breadcrumb/List/Item/Link/Page/Separator/Ellipsis |
| Button | `button` | Button (9 variants × sizes), ButtonVariant, ButtonSize |
| Button Group | `button_group` | ButtonGroup, Separator, Text |
| Calendar | `calendar` | Calendar, CalendarSimple, SimpleDate |
| Card | `card` | Card + Header/Title/Description/Content/Footer/Action |
| Carousel | `carousel` | Carousel, Content/Item/Previous/Next, Orientation |
| Chart | `chart` | Chart, ChartData/Series/Point, Legend, Tooltip |
| Checkbox | `checkbox` | Checkbox |
| Collapsible | `collapsible` | Collapsible, Trigger, Content |
| Combobox | `combobox` | Combobox + List/Item/Group/Input/Trigger/Label/Empty/Separator/Context |
| Command | `command` | Command, Input/List/Group/Item/Shortcut/Separator/Empty |
| Context Menu | `context_menu` | ContextMenu + Content/Item/Group/Label/Separator |
| Data Table | `table` | Table + Header/Body/Row/Head/Cell/Footer/Caption |
| Date Picker | `date_picker` | DatePicker, DateRangePicker |
| Dialog | `dialog` | Dialog + Trigger/Content/Header/Footer/Title/Description/Close, DialogContext |
| Drawer | `drawer` | Drawer + Content/Header/Footer/Title/Description/Close, DrawerSide |
| Dropdown Menu | `dropdown_menu` | DropdownMenu + Content/Item/Label/Group/Shortcut/Separator |
| Empty | `empty` | Empty + Header/Media/Title/Description/Content/Footer/Actions |
| Field | `field` | Field + Label/Description/Error/Group/Content/Action/Set/Legend/Title/Separator/Orientation |
| Form | `field` | FormField, FormLabel (shadcn v4 renamed Form → Field) |
| Hover Card | `hover_card` | HoverCard + Trigger/Content, Side/Align |
| Input | `input` | Input |
| Input Group | `input_group` | InputGroup, InputGroupAddon (start/end) |
| Input OTP | `input_otp` | InputOTP, Slot, Group, Separator; PinInput |
| Item | `item` | Item + Media/Content/Actions/Header/Footer/Title/Description/Group/Separator |
| Kbd | `kbd` | Kbd, KbdGroup |
| Label | `label` | Label, LabelWithIcon |
| Menubar | `menubar` | Menubar + Menu/Trigger/Content/Item/CheckboxItem/Shortcut/Separator |
| Navigation Menu | `navigation_menu` | NavigationMenu + List/Item/Trigger/Content/Link/Indicator/Viewport |
| Pagination | `pagination` | Pagination + Content/Item/Link/Previous/Next/Ellipsis |
| Popover | `popover` | Popover + Trigger/Content/Title/Description/Header/Close, Side/Align |
| Progress | `progress` | Progress |
| Radio Group | `radio_group` | RadioGroup, RadioGroupItem |
| Resizable | `resizable` | ResizablePanelGroup/Panel/Handle, Direction |
| Scroll Area | `scroll_area` | ScrollArea, ScrollAreaWithBar, ScrollOrientation |
| Select | `select` | Select + Trigger/Content/Item/Group/Label/Separator, SelectItemData |
| Separator | `separator` | Separator, Orientation |
| Sheet | `sheet` | Sheet + Content/Header/Footer/Title/Description/Close |
| Sidebar | `sidebar` | Sidebar + Header/Content/Group/Menu/MenuItem/MenuButton/Badge/Action/Sub*/Rail/Inset/Trigger/Footer, style cache |
| Skeleton | `skeleton` | Skeleton |
| Slider | `slider` | Slider |
| Sonner / Toast | `toast` | Toast + variants (default/success/destructive) |
| Spinner | `spinner` | Spinner (sm/md/lg, themed colors) |
| Switch | `switch` | Switch |
| Table | `table` | (see Data Table above) |
| Tabs | `tabs` | Tabs, TabsList, Tab, styled cache |
| Textarea | `textarea` | Textarea |
| Toggle | `toggle` | Toggle |
| Toggle Group | `toggle_group` | ToggleGroup (single/multiple), variants, sizes |
| Tooltip | `tooltip` | Tooltip |
| Typography | `typography` | H1–H4, P, Lead, Muted, Code, Blockquote, List |

## Verification

- `cargo test` — 128 tests across 17 suites + 46 doc tests, all green,
  including real headless **click** interaction tests (dialog backdrop,
  trigger, close buttons).
- `cargo clippy --all-targets -- -D warnings` — zero warnings under
  `missing_docs` + `clippy::pedantic`, with **no allow attributes anywhere**.
