//! `std::gui` - a native desktop GUI module built on `eframe`/`egui`. Handle
//! module: windows and widgets are stored behind integer handles in the
//! runtime's handle table, accessed via the `GuiStore` trait (implemented by
//! each runtime).
//!
//! Ported once from the former per-runtime `stdlib/gui/*.rs` copies (the VM is
//! canonical). The error strings and behaviour are reproduced verbatim.
//!
//! Unlike the other handle modules (net / audio / http), this one manipulates
//! the *whole* handle map at once - closing a window cascades to its child
//! widgets, and `gui_run` iterates the map every frame to find secondary
//! windows - so `GuiStore` exposes the raw `HashMap` directly rather than
//! per-id accessors.
//!
//! ## Generic-over-value handle enum
//!
//! `GuiHandle`/`*State` embed the runtime's callback values (an `on_click`
//! closure, an `on_change`, ...). Those are the runtime's own value type, which
//! `rl-std` cannot name concretely. So the whole enum is made generic over the
//! value type `V`: `GuiHandle<V>` / `WindowState<V>` / ... , instantiated by
//! each runtime as `GuiHandle<VmValue>` / `GuiHandle<Value>` and stored in a
//! `HashMap<u64, GuiHandle<R::Value>>`.

#[cfg(feature = "impls")]
use eframe::egui;
#[cfg(feature = "impls")]
use rl_ast::statements::HandleKind;
#[cfg(feature = "impls")]
use rl_ast::statements::TypeAnnotation;
#[cfg(feature = "impls")]
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use rl_utils::errors::Error;
#[cfg(feature = "impls")]
use std::collections::HashMap;

// ============================================================================
// Handle enum + state structs (moved verbatim from `gui/mod.rs`, generalized
// over the runtime value type `V` for the embedded callbacks).
// ============================================================================

#[cfg(feature = "impls")]
/// A single native GUI resource.
pub enum GuiHandle<V> {
    Window(WindowState<V>),
    Button(ButtonState<V>),
    Hyperlink(HyperlinkState),
    Spinner(SpinnerState),
    Selectable(SelectableState<V>),
    Label(LabelState),
    Checkbox(CheckboxState<V>),
    Textbox(TextboxState<V>),
    Dropdown(SelectState<V>),
    RadioGroup(SelectState<V>),
    Slider(SliderState<V>),
    ProgressBar(ProgressState),
    Separator(SeparatorState),
    Image(ImageState),
    Container(ContainerState),
    Canvas(CanvasState),
    Grid(GridState),
    Scroll(ScrollState),
}

/// One vector draw command on a canvas, issued fresh every frame from RL
/// (the list clears after each render  -  draw in `on_frame` for animation).
#[cfg(feature = "impls")]
#[derive(Clone)]
pub enum DrawCmd {
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: (u8, u8, u8),
        thickness: f32,
    },
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: (u8, u8, u8),
        filled: bool,
    },
    Circle {
        x: f32,
        y: f32,
        radius: f32,
        color: (u8, u8, u8),
        filled: bool,
    },
    Text {
        text: String,
        x: f32,
        y: f32,
        size: f32,
        color: (u8, u8, u8),
    },
}

/// A grid: children flow left-to-right into a fixed column count; each
/// row is as tall as its tallest child, each column as wide as its
/// widest. Cells are top-left aligned; use nested boxes for fancier
/// alignment. Like containers, positions recompute every frame.
#[cfg(feature = "impls")]
pub struct GridState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub columns: usize,
    pub spacing: f32,
    pub padding: f32,
    pub visible: bool,
    pub children: Vec<u64>,
}

/// A scroll viewport: children stack vertically like a box, but only the
/// slice intersecting `[scroll_y, scroll_y + height]` renders  -  the rest
/// go invisible for the frame (virtualized paging, no pixel scrolling).
/// Scroll with `gui_scroll_to` (typically from `on_scroll`).
#[cfg(feature = "impls")]
pub struct ScrollState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub spacing: f32,
    pub scroll_y: f32,
    pub content_bottom: f32,
    pub visible: bool,
    pub children: Vec<u64>,
}
#[cfg(feature = "impls")]
/// A drawable region: RL issues `gui_draw_*` calls into its command list
/// every frame, and the renderer replays them with the painter. Coordinates
/// are local to the canvas origin.
pub struct CanvasState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    pub z: i32,
    pub commands: Vec<DrawCmd>,
}

/// Stacking direction plus cross-axis alignment for a container. `Start`
/// means left in a vertical box, top in a horizontal one.
#[cfg(feature = "impls")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContainerAlign {
    Start,
    Center,
    End,
}

/// A layout container: stacks child widgets vertically or horizontally
/// with spacing, starting at its origin plus padding. Children keep
/// absolute coordinates recomputed every frame, so manual `gui_set_pos`
/// on a contained widget loses to the next layout pass. Containers nest
/// freely; cycles are refused at `gui_add` time.
#[cfg(feature = "impls")]
pub struct ContainerState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub horizontal: bool,
    pub spacing: f32,
    pub padding: f32,
    pub align: ContainerAlign,
    pub visible: bool,
    pub children: Vec<u64>,
}

#[cfg(feature = "impls")]
pub struct WindowState<V> {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    pub children: Vec<u64>,
    /// RGB background fill for the window's central panel.
    pub background: (u8, u8, u8),
    /// A pending resize request from `gui_window_set_size`, applied once and
    /// then cleared so it doesn't fight the user manually resizing the
    /// window afterward.
    pub pending_size: Option<(f32, f32)>,
    /// A pending move request from `gui_window_set_pos`, applied once and
    /// then cleared so it doesn't fight the user manually dragging the
    /// window afterward.
    pub pending_position: Option<(f32, f32)>,
    /// Last known window position set via `gui_window` or `gui_window_set_pos`.
    /// Updated from the viewport rect during rendering for size accuracy.
    pub position: (f32, f32),
    /// Whether the native title bar and window borders are shown.
    pub decorated: bool,
    /// Window icon as `(width, height, rgba bytes)`. `None` uses the OS default.
    pub icon: Option<(u32, u32, Vec<u8>)>,
    /// One-shot requests from `gui_window_fullscreen/maximize/minimize`,
    /// applied on the next frame then cleared (same pattern as
    /// `pending_size`/`pending_position`).
    pub pending_fullscreen: Option<bool>,
    pub pending_maximized: Option<bool>,
    pub pending_minimized: Option<bool>,
    /// Called with no arguments when this window closes, whether via
    /// `gui_close` or the native close button.
    pub on_close: Option<V>,
    /// Called with the key's name (e.g. `"Enter"`, `"Escape"`) and whether
    /// it was pressed (`true`) or released (`false`) for every non-repeat
    /// key event while this window has focus.
    pub on_key: Option<V>,
    /// Called with `(x, y)` logical pixels on every mouse move while this
    /// window has focus.
    pub on_mouse_move: Option<V>,
    /// Called with dropped file paths (`arr[string]`) when files are
    /// dropped onto this window.
    pub on_file_drop: Option<V>,
    /// Called with `(dx, dy)` scroll delta when the wheel moves while this
    /// window has focus.
    pub on_scroll: Option<V>,
    /// Called with no arguments every frame while set. Enables polling
    /// patterns (worker threads, animations) and implies continuous
    /// repaint. Pass `null` to unset.
    pub on_frame: Option<V>,
}

#[cfg(feature = "impls")]
pub struct ButtonState<V> {
    pub window: u64,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub on_click: Option<V>,
    /// Draw order among this window's widgets: higher draws on top of
    /// lower when positions overlap. Widgets with equal z draw in creation
    /// order (later created = on top).
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

/// A hyperlink: text plus URL. Clicks open the URL in the system browser
/// (egui built-in); no RL callback fires.
#[cfg(feature = "impls")]
pub struct HyperlinkState {
    pub window: u64,
    pub text: String,
    pub url: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

/// A loading spinner: pure animation, no text, no interaction. Shows
/// while workers run; remove it when done.
#[cfg(feature = "impls")]
pub struct SpinnerState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub visible: bool,
    pub z: i32,
}

/// One selectable row: highlighted when selected, fires `on_click` like a
/// button. Selection state is RL-owned (read with `gui_is_selected`,
/// written with `gui_set_selected`) so lists stay in charge.
#[cfg(feature = "impls")]
pub struct SelectableState<V> {
    pub window: u64,
    pub text: String,
    pub selected: bool,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub on_click: Option<V>,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

#[cfg(feature = "impls")]
pub struct LabelState {
    pub window: u64,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

#[cfg(feature = "impls")]
pub struct CheckboxState<V> {
    pub window: u64,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub checked: bool,
    pub on_change: Option<V>,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

#[cfg(feature = "impls")]
pub struct TextboxState<V> {
    pub window: u64,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub on_change: Option<V>,
    /// Called with the current text when Enter is pressed while this
    /// textbox has focus. Never fires when `multiline` is true, since Enter
    /// inserts a newline there instead of submitting.
    pub on_submit: Option<V>,
    /// Whether this is a multiline textarea (`gui_textarea`) rather than a
    /// single-line textbox (`gui_textbox`). Both share this same state and
    /// every get/set/visibility/position/remove/on_change function.
    pub multiline: bool,
    /// Row height in points. Only meaningful when `multiline` is true -
    /// single-line textboxes use a fixed row height instead.
    pub height: f32,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

#[cfg(feature = "impls")]
pub struct SelectState<V> {
    pub window: u64,
    pub options: Vec<String>,
    pub selected: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub on_change: Option<V>,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

#[cfg(feature = "impls")]
pub struct SliderState<V> {
    pub window: u64,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub on_change: Option<V>,
    /// When true, renders as a compact draggable/typeable number field
    /// (`gui_number_input`) instead of a slider bar (`gui_slider`). Both
    /// share this same state and every get/set/visibility/position/remove/
    /// on_change function.
    pub drag_only: bool,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

#[cfg(feature = "impls")]
pub struct ProgressState {
    pub window: u64,
    pub value: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

#[cfg(feature = "impls")]
pub struct SeparatorState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

#[cfg(feature = "impls")]
pub struct ImageState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    /// Raw RGBA8 pixel data as `(width, height, bytes)`, row-major
    /// top-to-bottom - same representation as `gui_window_set_icon`, since
    /// `std::gui` has no image-decoding dependency. `Arc`-wrapped since this
    /// gets cloned every frame while rendering.
    pub rgba: (u32, u32, std::sync::Arc<Vec<u8>>),
    /// Bumped by `gui_update_image`; the renderer skips the GPU upload
    /// while the loaded version matches, so static images upload once.
    pub version: u64,
    pub z: i32,
    pub font_size: Option<f32>,
    pub color: Option<(u8, u8, u8)>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub tooltip: Option<String>,
}

// ============================================================================
// Store trait: exposes the raw handle map, next-id counter and quit flag.
// ============================================================================

#[cfg(feature = "impls")]
/// Per-runtime access to the `gui` handle table and its bookkeeping. Implemented
/// by `VmRuntime` / `EvalRuntime` in the runtime crates. The runtime field is
/// `HashMap<u64, GuiHandle<Self::Value>>`.
pub trait GuiStore: Runtime {
    /// The whole handle map, mutably (this module cascades on close and
    /// iterates on render, so it needs the raw map, not per-id methods).
    fn gui_handles(cx: &mut Self::Cx) -> &mut HashMap<u64, GuiHandle<Self::Value>>;
    /// The whole handle map, shared.
    fn gui_handles_ref(cx: &Self::Cx) -> &HashMap<u64, GuiHandle<Self::Value>>;
    /// The next handle id to hand out (starts at 1, incremented per insert).
    fn gui_next_handle(cx: &mut Self::Cx) -> &mut u64;
    /// Set by `gui_quit` to ask the running event loop to close on its next
    /// frame; cleared by the loop once acted upon.
    fn gui_quit_requested(cx: &mut Self::Cx) -> &mut bool;
    /// Uploaded texture versions per image widget id: `(loaded_version,
    /// managed texture id)`. The renderer skips the GPU upload while the
    /// snapshot version matches, so static images upload once ever.
    /// Plain ids (not `TextureId`) because the store side must stay
    /// egui-free; every id here comes from `load_texture` (`Managed`).
    fn texture_cache(cx: &mut Self::Cx) -> &mut HashMap<u64, (u64, u64)>;
    /// Last rendered size per widget id: `(width, height)` in logical
    /// pixels. Recorded every frame by the renderer, read by the layout
    /// pass to position container children.
    fn widget_sizes(cx: &mut Self::Cx) -> &mut HashMap<u64, (f32, f32)>;
}

// ============================================================================
// common.rs helpers (ported as generic free fns over `R: GuiStore`).
// ============================================================================

#[cfg(feature = "impls")]
pub fn insert_handle<R: GuiStore>(cx: &mut R::Cx, handle: GuiHandle<R::Value>) -> R::Value {
    let id = *R::gui_next_handle(cx);
    *R::gui_next_handle(cx) += 1;
    R::gui_handles(cx).insert(id, handle);
    R::make_handle(HandleKind::Gui, id)
}

#[cfg(feature = "impls")]
pub fn attach_child<R: GuiStore>(cx: &mut R::Cx, window_id: u64, child_id: u64) {
    if let Some(GuiHandle::Window(w)) = R::gui_handles(cx).get_mut(&window_id) {
        w.children.push(child_id);
    }
}

#[cfg(feature = "impls")]
pub fn require_window<R: GuiStore>(
    cx: &R::Cx,
    window_id: u64,
    fn_name: &str,
) -> Result<(), String> {
    match R::gui_handles_ref(cx).get(&window_id) {
        Some(GuiHandle::Window(_)) => Ok(()),
        Some(_) => Err(format!("{}: handle {} is not a window", fn_name, window_id)),
        None => Err(format!("{}: unknown handle {}", fn_name, window_id)),
    }
}

#[cfg(feature = "impls")]
/// Reports a callback error the same way top-level rl-lang errors are
/// reported, instead of silently discarding it.
pub fn report_callback_err(result: Result<impl Sized, Error>) {
    if let Err(e) = result {
        e.report_to_stderr();
    }
}

#[cfg(feature = "impls")]
/// Removes a window handle and all of its child widget handles, firing the
/// window's `on_close` callback (if any) first. Used both by `gui_close` and
/// by `gui_run` when a window's native close button is clicked. No-op if
/// `id` isn't a window (or doesn't exist).
pub fn close_window<R: GuiStore>(cx: &mut R::Cx, id: u64, span: R::Span) {
    let Some(GuiHandle::Window(w)) = R::gui_handles(cx).remove(&id) else {
        return;
    };
    for child in &w.children {
        R::gui_handles(cx).remove(child);
    }
    if let Some(cb) = w.on_close {
        report_callback_err(R::call_value(cx, &cb, &[], span));
    }
}

// ---- shared argument extraction (reproducing the old `extract_*` helpers) --

#[cfg(feature = "impls")]
/// Reproduces the old `stdlib::common::extract_handle`: unwraps a `Gui` handle
/// into its id, with the same wrong-kind / not-a-handle messages.
fn extract_handle<R: GuiStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    match R::as_handle(v, HandleKind::Gui) {
        Some(id) => Ok(id),
        None => match R::as_handle(v, HandleKind::C)
            .map(|_| HandleKind::C)
            .or_else(|| R::as_handle(v, HandleKind::Http).map(|_| HandleKind::Http))
            .or_else(|| R::as_handle(v, HandleKind::Net).map(|_| HandleKind::Net))
            .or_else(|| R::as_handle(v, HandleKind::Audio).map(|_| HandleKind::Audio))
            .or_else(|| R::as_handle(v, HandleKind::File).map(|_| HandleKind::File))
        {
            Some(kind) => Err(format!(
                "{}: expected a {:?} handle, got a {:?} handle",
                name,
                HandleKind::Gui,
                kind
            )),
            None => Err(format!(
                "{}: expected a handle, got {}",
                name,
                R::type_name(v)
            )),
        },
    }
}

/// Build an `egui::RichText` applying optional font size and text colour.
#[cfg(feature = "impls")]
fn styled_text(text: &str, font_size: Option<f32>, color: Option<(u8, u8, u8)>) -> egui::RichText {
    let mut rt = egui::RichText::new(text);
    if let Some(size) = font_size {
        rt = rt.size(size);
    }
    if let Some((r, g, b)) = color {
        rt = rt.color(egui::Color32::from_rgb(r, g, b));
    }
    rt
}

/// Paint a background rect at the given position, if `bg` is set.
#[cfg(feature = "impls")]
fn paint_bg(
    painter: &egui::Painter,
    bg: &Option<(u8, u8, u8)>,
    rect: egui::Rect,
) {
    if let Some((r, g, b)) = bg {
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(*r, *g, *b));
    }
}

#[cfg(feature = "impls")]
/// Extracts `array[string]` into a `Vec<String>`, for `gui_dropdown`/`gui_radio_group`.
/// Reproduces the old `extract_string_array`, whose per-element error string is
/// `stdlib::common::extract_string`'s `"<name>: expected string type, got <ty>"`.
fn extract_string_array<R: GuiStore>(value: &R::Value, name: &str) -> Result<Vec<String>, String> {
    match R::as_array(value) {
        Some((items, _)) => items
            .iter()
            .map(|v| match R::as_str(v) {
                Some(s) => Ok(s.to_owned()),
                None => Err(format!(
                    "{}: expected string type, got {}",
                    name,
                    R::type_name(v)
                )),
            })
            .collect(),
        None => Err(format!(
            "{}: expected array[string], got {}",
            name,
            R::type_name(value)
        )),
    }
}

#[cfg(feature = "impls")]
/// Extracts `array[int]` into a `Vec<u8>`, validating every element is a byte
/// (0-255). Used by `gui_window_set_icon`/`gui_image` for raw RGBA pixel data.
/// Reproduces the old `extract_byte_array`, whose per-element error string is
/// `stdlib::common::extract_int`'s `"<name>: expected int type, got <ty>"`.
fn extract_byte_array<R: GuiStore>(value: &R::Value, name: &str) -> Result<Vec<u8>, String> {
    match R::as_array(value) {
        Some((items, _)) => items
            .iter()
            .map(|v| {
                let n = match R::as_i64(v) {
                    Some(n) => n,
                    None => {
                        return Err(format!(
                            "{}: expected int type, got {}",
                            name,
                            R::type_name(v)
                        ));
                    }
                };
                if !(0..=255).contains(&n) {
                    return Err(format!(
                        "{}: byte value {} is out of range (must be 0-255)",
                        name, n
                    ));
                }
                Ok(n as u8)
            })
            .collect(),
        None => Err(format!(
            "{}: expected array[int], got {}",
            name,
            R::type_name(value)
        )),
    }
}

// ============================================================================
// Widget-creation functions.
// ============================================================================

#[native_fn(module = "gui", bound = "GuiStore", sig(string, int, int -> result[handle(Gui)]))]
pub fn gui_window<R: GuiStore>(cx: &mut R::Cx, title: String, width: i64, height: i64) -> R::Value {
    let width = width.max(1) as f32;
    let height = height.max(1) as f32;
    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Window(WindowState {
            title,
            width,
            height,
            visible: true,
            children: Vec::new(),
            background: (27, 27, 27),
            pending_size: Some((width, height)),
            pending_position: None,
            position: (0.0, 0.0),
            decorated: true,
            icon: None,
            pending_fullscreen: None,
            pending_maximized: None,
            pending_minimized: None,
            on_close: None,
            on_key: None,
            on_mouse_move: None,
            on_file_drop: None,
            on_scroll: None,
            on_frame: None,
        }),
    );
    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string, int, int -> result[handle(Gui)]))]
pub fn gui_button<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    label: String,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_button") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_button") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Button(ButtonState {
            window: window_id,
            label,
            x: x as f32,
            y: y as f32,
            visible: true,
            on_click: None,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );

    let button_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, button_id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string, string, int, int -> result[handle(Gui)]))]
pub fn gui_hyperlink<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    text: String,
    url: String,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_hyperlink") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_hyperlink") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Hyperlink(HyperlinkState {
            window: window_id,
            text,
            url,
            x: x as f32,
            y: y as f32,
            visible: true,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );

    let link_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, link_id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int -> result[handle(Gui)]))]
pub fn gui_spinner<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_spinner") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_spinner") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Spinner(SpinnerState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            size: 28.0,
            visible: true,
            z: 0,
        }),
    );

    let spinner_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, spinner_id);

    R::ok(handle)
}

/// Creates a drawable region at (x, y) of the given size. Issue
/// `gui_draw_*` calls every frame (typically from `on_frame`); the
/// command list clears after each render, so static scenes redraw too.
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int, int -> result[handle(Gui)]))]
pub fn gui_canvas<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
    width: i64,
    height: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_canvas") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_canvas") {
        return R::err(R::from_string(e));
    }

    if width <= 0 || height <= 0 {
        return R::err(R::from_string(format!(
            "gui_canvas: width ({}) and height ({}) must be positive",
            width, height
        )));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Canvas(CanvasState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
            visible: true,
            z: 0,
            commands: Vec::new(),
        }),
    );

    let canvas_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, canvas_id);

    R::ok(handle)
}

/// Pushes one draw command onto a canvas. All coordinates are local to
/// the canvas origin; colors are 0-255.
#[cfg(feature = "impls")]
fn push_draw<R: GuiStore>(
    cx: &mut R::Cx,
    handle: &R::Value,
    name: &str,
    cmd: DrawCmd,
) -> R::Value {
    let id = match extract_handle::<R>(handle, name) {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Canvas(c)) => {
            c.commands.push(cmd);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "{name}: handle {id} is not a canvas",
        ))),
        None => R::err(R::from_string(format!("{name}: unknown handle {id}"))),
    }
}

#[allow(clippy::too_many_arguments)]
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int, int, int, int, int, int -> result[null]))]
pub fn gui_draw_line<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    r: i64,
    g: i64,
    b: i64,
    thickness: i64,
) -> R::Value {
    let cmd = DrawCmd::Line {
        x1: x1 as f32,
        y1: y1 as f32,
        x2: x2 as f32,
        y2: y2 as f32,
        color: (r.clamp(0, 255) as u8, g.clamp(0, 255) as u8, b.clamp(0, 255) as u8),
        thickness: (thickness.max(1)) as f32,
    };
    push_draw::<R>(cx, &handle, "gui_draw_line", cmd)
}

#[allow(clippy::too_many_arguments)]
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int, int, int, int, int, bool -> result[null]))]
pub fn gui_draw_rect<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
    r: i64,
    g: i64,
    b: i64,
    filled: bool,
) -> R::Value {
    let cmd = DrawCmd::Rect {
        x: x as f32,
        y: y as f32,
        w: w as f32,
        h: h as f32,
        color: (r.clamp(0, 255) as u8, g.clamp(0, 255) as u8, b.clamp(0, 255) as u8),
        filled,
    };
    push_draw::<R>(cx, &handle, "gui_draw_rect", cmd)
}

#[allow(clippy::too_many_arguments)]
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int, int, int, int, bool -> result[null]))]
pub fn gui_draw_circle<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    x: i64,
    y: i64,
    radius: i64,
    r: i64,
    g: i64,
    b: i64,
    filled: bool,
) -> R::Value {
    let cmd = DrawCmd::Circle {
        x: x as f32,
        y: y as f32,
        radius: (radius.max(1)) as f32,
        color: (r.clamp(0, 255) as u8, g.clamp(0, 255) as u8, b.clamp(0, 255) as u8),
        filled,
    };
    push_draw::<R>(cx, &handle, "gui_draw_circle", cmd)
}

#[allow(clippy::too_many_arguments)]
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int -> result[null]))]
pub fn gui_set_canvas_size<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    width: i64,
    height: i64,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_canvas_size") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if width <= 0 || height <= 0 {
        return R::err(R::from_string(format!(
            "gui_set_canvas_size: width ({}) and height ({}) must be positive",
            width, height
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Canvas(c)) => {
            c.width = width as f32;
            c.height = height as f32;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_canvas_size: handle {} is not a canvas",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_canvas_size: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[array[float]]))]
pub fn gui_get_canvas_size<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
) -> Result<Vec<f64>, String> {
    let id = extract_handle::<R>(&handle, "gui_get_canvas_size")?;
    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Canvas(c)) => Ok(vec![c.width as f64, c.height as f64]),
        Some(_) => Err(format!(
            "gui_get_canvas_size: handle {} is not a canvas",
            id
        )),
        None => Err(format!(
            "gui_get_canvas_size: unknown handle {}",
            id
        )),
    }
}

#[allow(clippy::too_many_arguments)]
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string, int, int, int, int, int, int -> result[null]))]
pub fn gui_draw_text<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    text: String,
    x: i64,
    y: i64,
    size: i64,
    r: i64,
    g: i64,
    b: i64,
) -> R::Value {
    let cmd = DrawCmd::Text {
        text,
        x: x as f32,
        y: y as f32,
        size: (size.max(1)) as f32,
        color: (r.clamp(0, 255) as u8, g.clamp(0, 255) as u8, b.clamp(0, 255) as u8),
    };
    push_draw::<R>(cx, &handle, "gui_draw_text", cmd)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string, bool, int, int -> result[handle(Gui)]))]
pub fn gui_selectable<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    text: String,
    selected: bool,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_selectable") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_selectable") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Selectable(SelectableState {
            window: window_id,
            text,
            selected,
            x: x as f32,
            y: y as f32,
            visible: true,
            on_click: None,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );

    let selectable_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, selectable_id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), bool -> result[null]))]
pub fn gui_set_selected<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    selected: bool,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_selected") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Selectable(s)) => {
            s.selected = selected;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_selected: handle {} is not selectable",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_selected: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[bool]))]
pub fn gui_is_selected<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_is_selected") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Selectable(s)) => R::ok(R::from_bool(s.selected)),
        Some(_) => R::err(R::from_string(format!(
            "gui_is_selected: handle {} is not selectable",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_is_selected: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string, int, int -> result[handle(Gui)]))]
pub fn gui_label<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    text: String,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_label") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get(&window_id) {
        Some(GuiHandle::Window(_)) => {}
        Some(_) => {
            return R::err(R::from_string(format!(
                "gui_label: handle {} is not a window",
                window_id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "gui_label: unknown handle {}",
                window_id
            )));
        }
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Label(LabelState {
            window: window_id,
            text,
            x: x as f32,
            y: y as f32,
            visible: true,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );

    let label_id = R::as_handle(&handle, HandleKind::Gui).unwrap();

    if let Some(GuiHandle::Window(w)) = R::gui_handles(cx).get_mut(&window_id) {
        w.children.push(label_id);
    }

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string, int, int -> result[handle(Gui)]))]
pub fn gui_checkbox<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    label: String,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_checkbox") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_checkbox") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Checkbox(CheckboxState {
            window: window_id,
            label,
            x: x as f32,
            y: y as f32,
            visible: true,
            checked: false,
            on_change: None,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );

    let checkbox_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, checkbox_id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string, int, int, int -> result[handle(Gui)]))]
pub fn gui_textbox<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    text: String,
    x: i64,
    y: i64,
    width: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_textbox") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_textbox") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Textbox(TextboxState {
            window: window_id,
            text,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            on_change: None,
            on_submit: None,
            multiline: false,
            height: 20.0,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );

    let textbox_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, textbox_id);

    R::ok(handle)
}

/// A multiline textarea. Shares `TextboxState`/`GuiHandle::Textbox` with
/// `gui_textbox` (distinguished only by `multiline: true`), so it works
/// automatically with every existing textbox function - `gui_get_text`,
/// `gui_set_text`, `gui_set_visible`, `gui_set_pos`, `gui_remove`, and
/// `gui_on_change`. The one exception is `gui_on_submit`: Enter inserts a
/// newline here instead of submitting, so it never fires for a textarea.
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string, int, int, int, int -> result[handle(Gui)]))]
pub fn gui_textarea<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    text: String,
    x: i64,
    y: i64,
    width: i64,
    height: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_textarea") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_textarea") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Textbox(TextboxState {
            window: window_id,
            text,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            on_change: None,
            on_submit: None,
            multiline: true,
            height: height.max(1) as f32,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );

    let textarea_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, textarea_id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), array[string], int, int, int -> result[handle(Gui)]))]
pub fn gui_dropdown<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    options: R::Value,
    x: i64,
    y: i64,
    width: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_dropdown") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    if let Err(e) = require_window::<R>(cx, window_id, "gui_dropdown") {
        return R::err(R::from_string(e));
    }
    let options = match extract_string_array::<R>(&options, "gui_dropdown") {
        Ok(o) => o,
        Err(e) => return R::err(R::from_string(e)),
    };
    if options.is_empty() {
        return R::err(R::from_string(
            "gui_dropdown: options array must not be empty".to_string(),
        ));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Dropdown(SelectState {
            window: window_id,
            options,
            selected: 0,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            on_change: None,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );
    let id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), array[string], int, int -> result[handle(Gui)]))]
pub fn gui_radio_group<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    options: R::Value,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_radio_group") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    if let Err(e) = require_window::<R>(cx, window_id, "gui_radio_group") {
        return R::err(R::from_string(e));
    }
    let options = match extract_string_array::<R>(&options, "gui_radio_group") {
        Ok(o) => o,
        Err(e) => return R::err(R::from_string(e)),
    };
    if options.is_empty() {
        return R::err(R::from_string(
            "gui_radio_group: options array must not be empty".to_string(),
        ));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::RadioGroup(SelectState {
            window: window_id,
            options,
            selected: 0,
            x: x as f32,
            y: y as f32,
            width: 0.0,
            visible: true,
            on_change: None,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );
    let id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), float, float, int, int, int -> result[handle(Gui)]))]
pub fn gui_slider<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    min: f64,
    max: f64,
    x: i64,
    y: i64,
    width: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_slider") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    if let Err(e) = require_window::<R>(cx, window_id, "gui_slider") {
        return R::err(R::from_string(e));
    }

    if min >= max {
        return R::err(R::from_string(format!(
            "gui_slider: min ({}) must be less than max ({})",
            min, max
        )));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Slider(SliderState {
            window: window_id,
            value: min,
            min,
            max,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            on_change: None,
            drag_only: false,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );
    let id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, id);

    R::ok(handle)
}

/// A compact draggable/typeable number field. Shares `SliderState`/
/// `GuiHandle::Slider` with `gui_slider` (distinguished only by
/// `drag_only: true`), so it works automatically with every existing
/// slider function - `gui_get_value`, `gui_set_value`, `gui_set_visible`,
/// `gui_set_pos`, `gui_remove`, and `gui_on_change`.
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), float, float, float, int, int -> result[handle(Gui)]))]
pub fn gui_number_input<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    value: f64,
    min: f64,
    max: f64,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_number_input") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    if let Err(e) = require_window::<R>(cx, window_id, "gui_number_input") {
        return R::err(R::from_string(e));
    }

    if min >= max {
        return R::err(R::from_string(format!(
            "gui_number_input: min ({}) must be less than max ({})",
            min, max
        )));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Slider(SliderState {
            window: window_id,
            value: value.clamp(min, max),
            min,
            max,
            x: x as f32,
            y: y as f32,
            width: 0.0, // unused for drag_only widgets - DragValue auto-sizes
            visible: true,
            on_change: None,
            drag_only: true,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );
    let id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int -> result[handle(Gui)]))]
pub fn gui_progress_bar<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
    width: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_progress_bar") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    if let Err(e) = require_window::<R>(cx, window_id, "gui_progress_bar") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::ProgressBar(ProgressState {
            window: window_id,
            value: 0.0,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );
    let id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int -> result[handle(Gui)]))]
pub fn gui_separator<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
    width: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_separator") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    if let Err(e) = require_window::<R>(cx, window_id, "gui_separator") {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Separator(SeparatorState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );
    let id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, id);

    R::ok(handle)
}

/// callers must supply already-decoded pixels.
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, array[int], int, int -> result[handle(Gui)]))]
pub fn gui_image<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    width: i64,
    height: i64,
    rgba: R::Value,
    x: i64,
    y: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_image") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    if let Err(e) = require_window::<R>(cx, window_id, "gui_image") {
        return R::err(R::from_string(e));
    }

    if width <= 0 || height <= 0 {
        return R::err(R::from_string(format!(
            "gui_image: width ({}) and height ({}) must be positive",
            width, height
        )));
    }

    let rgba_bytes = match extract_byte_array::<R>(&rgba, "gui_image") {
        Ok(bytes) => bytes,
        Err(e) => return R::err(R::from_string(e)),
    };

    let expected_len = width as usize * height as usize * 4;
    if rgba_bytes.len() != expected_len {
        return R::err(R::from_string(format!(
            "gui_image: expected {} rgba bytes for a {}x{} image, got {}",
            expected_len,
            width,
            height,
            rgba_bytes.len()
        )));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Image(ImageState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
            visible: true,
            rgba: (width as u32, height as u32, std::sync::Arc::new(rgba_bytes)),
            version: 0,
            z: 0,
            font_size: None,
            color: None,
            bg_color: None,
            tooltip: None,
        }),
    );
    let id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, id);

    R::ok(handle)
}

/// Replaces an image's pixels (and size), bumping its version so the next
/// frame re-uploads the texture exactly once. Static images never call
/// this, so they upload once ever.
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, array[int] -> result[null]))]
pub fn gui_update_image<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    width: i64,
    height: i64,
    rgba: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_update_image") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if width <= 0 || height <= 0 {
        return R::err(R::from_string(format!(
            "gui_update_image: width ({}) and height ({}) must be positive",
            width, height
        )));
    }

    let rgba_bytes = match extract_byte_array::<R>(&rgba, "gui_update_image") {
        Ok(bytes) => bytes,
        Err(e) => return R::err(R::from_string(e)),
    };

    let expected_len = width as usize * height as usize * 4;
    if rgba_bytes.len() != expected_len {
        return R::err(R::from_string(format!(
            "gui_update_image: expected {} rgba bytes for a {}x{} image, got {}",
            expected_len,
            width,
            height,
            rgba_bytes.len()
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Image(img)) => {
            img.width = width as f32;
            img.height = height as f32;
            img.rgba = (
                width as u32,
                height as u32,
                std::sync::Arc::new(rgba_bytes),
            );
            img.version += 1;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_update_image: handle {} is not an image",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_update_image: unknown handle {}",
            id
        ))),
    }
}

// ============================================================================
// Layout containers.
// ============================================================================

/// Shared constructor for `gui_vbox`/`gui_hbox`.
#[cfg(feature = "impls")]
fn gui_container<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
    spacing: i64,
    horizontal: bool,
    name: &str,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, name) {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, name) {
        return R::err(R::from_string(e));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Container(ContainerState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            horizontal,
            spacing: (spacing.max(0)) as f32,
            padding: 0.0,
            align: ContainerAlign::Start,
            visible: true,
            children: Vec::new(),
        }),
    );

    let container_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, container_id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int -> result[handle(Gui)]))]
pub fn gui_vbox<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
    spacing: i64,
) -> R::Value {
    gui_container::<R>(cx, window, x, y, spacing, false, "gui_vbox")
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int -> result[handle(Gui)]))]
pub fn gui_hbox<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
    spacing: i64,
) -> R::Value {
    gui_container::<R>(cx, window, x, y, spacing, true, "gui_hbox")
}

/// Moves a widget into a container, grid, or scroll. The widget leaves
/// the window's top-level children and is positioned by the layout pass
/// from then on.
/// Containers nest freely, but cycles are refused: a container cannot
/// hold itself or one of its own ancestors.
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), handle(Gui) -> result[null]))]
pub fn gui_add<R: GuiStore>(
    cx: &mut R::Cx,
    container: R::Value,
    widget: R::Value,
) -> R::Value {
    let container_id = match extract_handle::<R>(&container, "gui_add") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    let widget_id = match extract_handle::<R>(&widget, "gui_add") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let window_id = match R::gui_handles_ref(cx).get(&container_id) {
        Some(GuiHandle::Container(c)) => c.window,
        Some(GuiHandle::Grid(g)) => g.window,
        Some(GuiHandle::Scroll(s)) => s.window,
        Some(_) => {
            return R::err(R::from_string(format!(
                "gui_add: handle {} is not a container, grid, or scroll",
                container_id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "gui_add: unknown handle {}",
                container_id
            )));
        }
    };
    match R::gui_handles_ref(cx).get(&widget_id) {
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string("gui_add: cannot add a window".to_string()));
        }
        None => {
            return R::err(R::from_string(format!(
                "gui_add: unknown handle {}",
                widget_id
            )));
        }
        _ => {}
    }
    // Cycle check: adding W into C is a cycle iff C is reachable from W.
    if container_id == widget_id || reaches::<R>(cx, widget_id, container_id) {
        return R::err(R::from_string(
            "gui_add: parents cannot hold themselves or their ancestors".to_string(),
        ));
    }

    if let Some(GuiHandle::Window(w)) = R::gui_handles(cx).get_mut(&window_id) {
        w.children.retain(|c| *c != widget_id);
    }
    // A widget lives in exactly one place: evict from any other container.
    let mut elsewhere = Vec::new();
    for (id, h) in R::gui_handles(cx).iter_mut() {
        let holds = match h {
            GuiHandle::Container(c) => *id != container_id && c.children.contains(&widget_id),
            GuiHandle::Grid(g) => *id != container_id && g.children.contains(&widget_id),
            GuiHandle::Scroll(s) => *id != container_id && s.children.contains(&widget_id),
            _ => false,
        };
        if holds {
            elsewhere.push(*id);
        }
    }
    for id in elsewhere {
        match R::gui_handles(cx).get_mut(&id) {
            Some(GuiHandle::Container(c)) => {
                c.children.retain(|c| *c != widget_id);
            }
            Some(GuiHandle::Grid(g)) => {
                g.children.retain(|c| *c != widget_id);
            }
            Some(GuiHandle::Scroll(s)) => {
                s.children.retain(|c| *c != widget_id);
            }
            _ => {}
        }
    }
    match R::gui_handles(cx).get_mut(&container_id) {
        Some(GuiHandle::Container(c)) if !c.children.contains(&widget_id) => {
            c.children.push(widget_id);
        }
        Some(GuiHandle::Grid(g)) if !g.children.contains(&widget_id) => {
            g.children.push(widget_id);
        }
        Some(GuiHandle::Scroll(s)) if !s.children.contains(&widget_id) => {
            s.children.push(widget_id);
        }
        Some(GuiHandle::Container(_))
        | Some(GuiHandle::Grid(_))
        | Some(GuiHandle::Scroll(_)) => {}
        _ => {}
    }
    R::ok(R::null())
}

/// Whether `target` is reachable from `root` through parent children.
#[cfg(feature = "impls")]
fn reaches<R: GuiStore>(cx: &R::Cx, root: u64, target: u64) -> bool {
    let mut stack = vec![root];
    let mut seen = std::collections::HashSet::new();
    while let Some(id) = stack.pop() {
        if id == target {
            return true;
        }
        if !seen.insert(id) {
            continue;
        }
        match R::gui_handles_ref(cx).get(&id) {
            Some(GuiHandle::Container(c)) => stack.extend(c.children.iter().copied()),
            Some(GuiHandle::Grid(g)) => stack.extend(g.children.iter().copied()),
            Some(GuiHandle::Scroll(s)) => stack.extend(s.children.iter().copied()),
            _ => {}
        }
    }
    false
}

/// Moves a widget out of its container back to the window's top level, at
/// its current (last laid-out) position.
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[null]))]
pub fn gui_detach<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_detach") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let window_id = match R::gui_handles_ref(cx).get(&id) {
        Some(h) => widget_window_ref(h),
        None => {
            return R::err(R::from_string(format!(
                "gui_detach: unknown handle {}",
                id
            )));
        }
    };
    let Some(window_id) = window_id else {
        return R::err(R::from_string(format!(
            "gui_detach: handle {} is a window or a parent, not a child",
            id
        )));
    };

    let mut found = false;
    for h in R::gui_handles(cx).values_mut() {
        let kids: Option<&mut Vec<u64>> = match h {
            GuiHandle::Container(c) => Some(&mut c.children),
            GuiHandle::Grid(g) => Some(&mut g.children),
            GuiHandle::Scroll(s) => Some(&mut s.children),
            _ => None,
        };
        if let Some(kids) = kids {
            let before = kids.len();
            kids.retain(|c| *c != id);
            found = found || kids.len() != before;
        }
    }
    if !found {
        return R::err(R::from_string(format!(
            "gui_detach: handle {} is in no container, grid, or scroll",
            id
        )));
    }
    if let Some(GuiHandle::Window(w)) = R::gui_handles(cx).get_mut(&window_id)
        && !w.children.contains(&id)
    {
        w.children.push(id);
    }
    R::ok(R::null())
}

/// Reads a widget's owning window id. Windows and parents (containers,
/// grids, scrolls) have none.
#[cfg(feature = "impls")]
fn widget_window_ref<V>(handle: &GuiHandle<V>) -> Option<u64> {
    match handle {
        GuiHandle::Window(_) | GuiHandle::Container(_) | GuiHandle::Grid(_) | GuiHandle::Scroll(_) => None,
        GuiHandle::Button(w) => Some(w.window),
        GuiHandle::Hyperlink(w) => Some(w.window),
        GuiHandle::Canvas(w) => Some(w.window),
        GuiHandle::Spinner(w) => Some(w.window),
        GuiHandle::Selectable(w) => Some(w.window),
        GuiHandle::Label(w) => Some(w.window),
        GuiHandle::Checkbox(w) => Some(w.window),
        GuiHandle::Textbox(w) => Some(w.window),
        GuiHandle::Dropdown(w) => Some(w.window),
        GuiHandle::RadioGroup(w) => Some(w.window),
        GuiHandle::Slider(w) => Some(w.window),
        GuiHandle::ProgressBar(w) => Some(w.window),
        GuiHandle::Separator(w) => Some(w.window),
        GuiHandle::Image(w) => Some(w.window),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int -> result[null]))]
pub fn gui_set_spacing<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    spacing: i64,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_spacing") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Container(c)) => {
            c.spacing = spacing.max(0) as f32;
            R::ok(R::null())
        }
        Some(GuiHandle::Grid(g)) => {
            g.spacing = spacing.max(0) as f32;
            R::ok(R::null())
        }
        Some(GuiHandle::Scroll(s)) => {
            s.spacing = spacing.max(0) as f32;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_spacing: handle {} is not a container",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_spacing: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int -> result[null]))]
pub fn gui_set_padding<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    padding: i64,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_padding") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Container(c)) => {
            c.padding = padding.max(0) as f32;
            R::ok(R::null())
        }
        Some(GuiHandle::Grid(g)) => {
            g.padding = padding.max(0) as f32;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_padding: handle {} is not a container",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_padding: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string -> result[null]))]
pub fn gui_set_align<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    align: String,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_align") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let parsed = match align.as_str() {
        "start" | "left" | "top" => ContainerAlign::Start,
        "center" | "middle" => ContainerAlign::Center,
        "end" | "right" | "bottom" => ContainerAlign::End,
        _ => {
            return R::err(R::from_string(format!(
                "gui_set_align: expected start/center/end (left/top and right/bottom work too), got {}",
                align
            )));
        }
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Container(c)) => {
            c.align = parsed;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_align: handle {} is not a container",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_align: unknown handle {}",
            id
        ))),
    }
}

// ============================================================================
// Grid and scroll containers.
// ============================================================================

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int -> result[handle(Gui)]))]
pub fn gui_grid<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
    columns: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_grid") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_grid") {
        return R::err(R::from_string(e));
    }

    if columns <= 0 {
        return R::err(R::from_string(format!(
            "gui_grid: columns ({}) must be positive",
            columns
        )));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Grid(GridState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            columns: columns as usize,
            spacing: 8.0,
            padding: 0.0,
            visible: true,
            children: Vec::new(),
        }),
    );

    let grid_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, grid_id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int, int -> result[handle(Gui)]))]
pub fn gui_scroll<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
    width: i64,
    height: i64,
) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_scroll") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if let Err(e) = require_window::<R>(cx, window_id, "gui_scroll") {
        return R::err(R::from_string(e));
    }

    if width <= 0 || height <= 0 {
        return R::err(R::from_string(format!(
            "gui_scroll: width ({}) and height ({}) must be positive",
            width, height
        )));
    }

    let handle = insert_handle::<R>(
        cx,
        GuiHandle::Scroll(ScrollState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
            spacing: 4.0,
            scroll_y: 0.0,
            content_bottom: 0.0,
            visible: true,
            children: Vec::new(),
        }),
    );

    let scroll_id = R::as_handle(&handle, HandleKind::Gui).unwrap();
    attach_child::<R>(cx, window_id, scroll_id);

    R::ok(handle)
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int -> result[null]))]
pub fn gui_scroll_to<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    px: i64,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_scroll_to") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Scroll(s)) => {
            s.scroll_y = (px.max(0)) as f32;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_scroll_to: handle {} is not a scroll",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_scroll_to: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[int]))]
pub fn gui_scroll_pos<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_scroll_pos") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Scroll(s)) => R::ok(R::from_i64(s.scroll_y as i64)),
        Some(_) => R::err(R::from_string(format!(
            "gui_scroll_pos: handle {} is not a scroll",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_scroll_pos: unknown handle {}",
            id
        ))),
    }
}

// ============================================================================
// Getters / setters.
// ============================================================================

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string -> result[null]))]
pub fn gui_set_text<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, text: String) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_text") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(b)) => {
            b.label = text;
            R::ok(R::null())
        }
        Some(GuiHandle::Hyperlink(h)) => {
            h.text = text;
            R::ok(R::null())
        }
        Some(GuiHandle::Selectable(s)) => {
            s.text = text;
            R::ok(R::null())
        }
        Some(GuiHandle::Label(l)) => {
            l.text = text;
            R::ok(R::null())
        }
        Some(GuiHandle::Textbox(t)) => {
            t.text = text;
            R::ok(R::null())
        }
        Some(GuiHandle::Spinner(_)) => R::err(R::from_string("gui_set_text: spinners have no text".to_string())),
        Some(_) => R::err(R::from_string(format!(
            "gui_set_text: handle {} has no text",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_text: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[string]))]
pub fn gui_get_text<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_get_text") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Button(b)) => R::ok(R::from_string(b.label.clone())),
        Some(GuiHandle::Hyperlink(h)) => R::ok(R::from_string(h.text.clone())),
        Some(GuiHandle::Label(l)) => R::ok(R::from_string(l.text.clone())),
        Some(GuiHandle::Textbox(t)) => R::ok(R::from_string(t.text.clone())),
        Some(_) => R::err(R::from_string(format!(
            "gui_get_text: handle {} has no text",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_get_text: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), bool -> result[null]))]
pub fn gui_set_visible<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, visible: bool) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_visible") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => w.visible = visible,
        Some(GuiHandle::Button(b)) => b.visible = visible,
        Some(GuiHandle::Hyperlink(h)) => h.visible = visible,
        Some(GuiHandle::Grid(g)) => g.visible = visible,
        Some(GuiHandle::Scroll(s)) => s.visible = visible,
        Some(GuiHandle::Canvas(c)) => c.visible = visible,
        Some(GuiHandle::Container(c)) => c.visible = visible,
        Some(GuiHandle::Selectable(s)) => s.visible = visible,
        Some(GuiHandle::Spinner(s)) => s.visible = visible,
        Some(GuiHandle::Label(l)) => l.visible = visible,
        Some(GuiHandle::Checkbox(c)) => c.visible = visible,
        Some(GuiHandle::Textbox(t)) => t.visible = visible,
        Some(GuiHandle::Dropdown(s)) => s.visible = visible,
        Some(GuiHandle::RadioGroup(s)) => s.visible = visible,
        Some(GuiHandle::Slider(s)) => s.visible = visible,
        Some(GuiHandle::ProgressBar(p)) => p.visible = visible,
        Some(GuiHandle::Separator(s)) => s.visible = visible,
        Some(GuiHandle::Image(i)) => i.visible = visible,
        None => {
            return R::err(R::from_string(format!(
                "gui_set_visible: unknown handle {}",
                id
            )));
        }
    }

    R::ok(R::null())
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[bool]))]
pub fn gui_is_visible<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_is_visible") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Window(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::Button(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::Hyperlink(h)) => R::ok(R::from_bool(h.visible)),
        Some(GuiHandle::Grid(g)) => R::ok(R::from_bool(g.visible)),
        Some(GuiHandle::Scroll(s)) => R::ok(R::from_bool(s.visible)),
        Some(GuiHandle::Canvas(c)) => R::ok(R::from_bool(c.visible)),
        Some(GuiHandle::Container(c)) => R::ok(R::from_bool(c.visible)),
        Some(GuiHandle::Selectable(s)) => R::ok(R::from_bool(s.visible)),
        Some(GuiHandle::Spinner(s)) => R::ok(R::from_bool(s.visible)),
        Some(GuiHandle::Label(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::Checkbox(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::Textbox(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::Dropdown(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::RadioGroup(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::Slider(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::ProgressBar(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::Separator(w)) => R::ok(R::from_bool(w.visible)),
        Some(GuiHandle::Image(w)) => R::ok(R::from_bool(w.visible)),
        None => R::err(R::from_string(format!(
            "gui_is_visible: unknown handle {}",
            id
        ))),
    }
}

// ---- event-callback registration -------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), callback( -> null) -> result[null]))]
pub fn gui_on_click<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, function: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_click") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if !R::is_callable(&function) {
        return R::err(R::from_string(format!(
            "gui_on_click: expected function or lambda, found {}",
            R::type_name(&function)
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(b)) => {
            b.on_click = Some(function);
            R::ok(R::null())
        }
        Some(GuiHandle::Selectable(s)) => {
            s.on_click = Some(function);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_click: handle {} is not clickable",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_on_click: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore",
    sig(handle(Gui), callback(bool -> null) -> result[null]),
    sig(handle(Gui), callback(int -> null) -> result[null]),
    sig(handle(Gui), callback(float -> null) -> result[null]),
    sig(handle(Gui), callback(string -> null) -> result[null]))]
pub fn gui_on_change<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    function: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_change") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if !R::is_callable(&function) {
        return R::err(R::from_string(format!(
            "gui_on_change: expected function or lambda, found {}",
            R::type_name(&function)
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Checkbox(c)) => {
            c.on_change = Some(function);
            R::ok(R::null())
        }
        Some(GuiHandle::Textbox(t)) => {
            t.on_change = Some(function);
            R::ok(R::null())
        }
        Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
            s.on_change = Some(function);
            R::ok(R::null())
        }
        Some(GuiHandle::Slider(s)) => {
            s.on_change = Some(function);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_change: handle {} does not support change events",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_on_change: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), callback(string -> null) -> result[null]))]
pub fn gui_on_submit<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    function: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_submit") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if !R::is_callable(&function) {
        return R::err(R::from_string(format!(
            "gui_on_submit: expected function or lambda, found {}",
            R::type_name(&function)
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Textbox(t)) if t.multiline => R::err(R::from_string(format!(
            "gui_on_submit: handle {} is a multiline textarea - Enter inserts a newline there instead of submitting, so on_submit never fires. Use a button instead",
            id
        ))),
        Some(GuiHandle::Textbox(t)) => {
            t.on_submit = Some(function);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_submit: handle {} is not a textbox",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_on_submit: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), callback(string, bool -> null) -> result[null]))]
pub fn gui_on_key<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, function: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_key") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if !R::is_callable(&function) {
        return R::err(R::from_string(format!(
            "gui_on_key: expected function or lambda, found {}",
            R::type_name(&function)
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.on_key = Some(function);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_key: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!("gui_on_key: unknown handle {}", id))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), callback(int, int -> null) -> result[null]))]
pub fn gui_on_mouse_move<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    function: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_mouse_move") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if !R::is_callable(&function) {
        return R::err(R::from_string(format!(
            "gui_on_mouse_move: expected function or lambda, found {}",
            R::type_name(&function)
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.on_mouse_move = Some(function);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_mouse_move: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_on_mouse_move: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), callback(array[string] -> null) -> result[null]))]
pub fn gui_on_file_drop<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    function: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_file_drop") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if !R::is_callable(&function) {
        return R::err(R::from_string(format!(
            "gui_on_file_drop: expected function or lambda, found {}",
            R::type_name(&function)
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.on_file_drop = Some(function);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_file_drop: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_on_file_drop: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), callback(int, int -> null) -> result[null]))]
pub fn gui_on_scroll<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    function: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_scroll") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if !R::is_callable(&function) {
        return R::err(R::from_string(format!(
            "gui_on_scroll: expected function or lambda, found {}",
            R::type_name(&function)
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.on_scroll = Some(function);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_scroll: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_on_scroll: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), callback( -> null) -> result[null]))]
pub fn gui_on_close<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, function: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_close") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if !R::is_callable(&function) {
        return R::err(R::from_string(format!(
            "gui_on_close: expected function or lambda, found {}",
            R::type_name(&function)
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.on_close = Some(function);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_close: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_on_close: unknown handle {}",
            id
        ))),
    }
}

// ---- per-frame callback ---------------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore",
    sig(handle(Gui), callback( -> null) -> result[null]))]
pub fn gui_on_frame<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, function: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_on_frame") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    // `null` unsets a previous callback; otherwise a callable is required.
    let callback = if R::type_name(&function) == "null" {
        None
    } else if R::is_callable(&function) {
        Some(function)
    } else {
        return R::err(R::from_string(format!(
            "gui_on_frame: expected function, lambda, or null, found {}",
            R::type_name(&function)
        )));
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.on_frame = callback;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_on_frame: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_on_frame: unknown handle {}",
            id
        ))),
    }
}

// ---- checkbox ---------------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[bool]))]
pub fn gui_is_checked<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_is_checked") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Checkbox(c)) => R::ok(R::from_bool(c.checked)),
        Some(_) => R::err(R::from_string(format!(
            "gui_is_checked: handle {} is not a checkbox",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_is_checked: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), bool -> result[null]))]
pub fn gui_set_checked<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, checked: bool) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_checked") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Checkbox(c)) => {
            c.checked = checked;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_checked: handle {} is not a checkbox",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_checked: unknown handle {}",
            id
        ))),
    }
}

// ---- selection --------------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[int]))]
pub fn gui_get_selected_index<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_get_selected_index") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
            R::ok(R::from_i64(s.selected as i64))
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_get_selected_index: handle {} has no selection",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_get_selected_index: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int -> result[null]))]
pub fn gui_set_selected_index<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    index: i64,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_selected_index") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
            if index < 0 || index as usize >= s.options.len() {
                return R::err(R::from_string(format!(
                    "gui_set_selected_index: index {} out of range (0..{})",
                    index,
                    s.options.len()
                )));
            }
            s.selected = index as usize;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_selected_index: handle {} has no selection",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_selected_index: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[string]))]
pub fn gui_get_selected<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_get_selected") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
            match s.options.get(s.selected) {
                Some(opt) => R::ok(R::from_string(opt.clone())),
                None => R::err(R::from_string(format!(
                    "gui_get_selected: handle {} has no options",
                    id
                ))),
            }
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_get_selected: handle {} has no selection",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_get_selected: unknown handle {}",
            id
        ))),
    }
}

// ---- slider / number-input value -------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[float]))]
pub fn gui_get_value<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_get_value") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Slider(s)) => R::ok(R::from_f64(s.value)),
        Some(_) => R::err(R::from_string(format!(
            "gui_get_value: handle {} is not a slider",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_get_value: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), float -> result[null]))]
pub fn gui_set_value<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, value: f64) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_value") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Slider(s)) => {
            s.value = value.clamp(s.min, s.max);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_value: handle {} is not a slider",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_value: unknown handle {}",
            id
        ))),
    }
}

// ---- progress bar -----------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), float -> result[null]))]
pub fn gui_set_progress<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, value: f64) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_progress") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::ProgressBar(p)) => {
            p.value = (value as f32).clamp(0.0, 1.0);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_set_progress: handle {} is not a progress bar",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_set_progress: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[float]))]
pub fn gui_get_progress<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_get_progress") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::ProgressBar(p)) => R::ok(R::from_f64(p.value as f64)),
        Some(_) => R::err(R::from_string(format!(
            "gui_get_progress: handle {} is not a progress bar",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_get_progress: unknown handle {}",
            id
        ))),
    }
}

// ---- position ---------------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int -> result[null]))]
pub fn gui_set_pos<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, x: i64, y: i64) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_pos") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Hyperlink(h)) => {
            h.x = x as f32;
            h.y = y as f32;
        }
        Some(GuiHandle::Grid(g)) => {
            g.x = x as f32;
            g.y = y as f32;
        }
        Some(GuiHandle::Scroll(s)) => {
            s.x = x as f32;
            s.y = y as f32;
        }
        Some(GuiHandle::Canvas(c)) => {
            c.x = x as f32;
            c.y = y as f32;
        }
        Some(GuiHandle::Container(c)) => {
            c.x = x as f32;
            c.y = y as f32;
        }
        Some(GuiHandle::Selectable(s)) => {
            s.x = x as f32;
            s.y = y as f32;
        }
        Some(GuiHandle::Spinner(s)) => {
            s.x = x as f32;
            s.y = y as f32;
        }
        Some(GuiHandle::Label(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Checkbox(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Textbox(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Dropdown(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::RadioGroup(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Slider(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::ProgressBar(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Separator(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Image(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(format!(
                "gui_set_pos: handle {} is a window and cannot be repositioned",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "gui_set_pos: unknown handle {}",
                id
            )));
        }
    }

    R::ok(R::null())
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[tuple[int, int]]))]
pub fn gui_get_pos<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_get_pos") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let (x, y) = match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Button(w)) => (w.x, w.y),
        Some(GuiHandle::Hyperlink(h)) => (h.x, h.y),
        Some(GuiHandle::Grid(g)) => (g.x, g.y),
        Some(GuiHandle::Scroll(s)) => (s.x, s.y),
        Some(GuiHandle::Canvas(c)) => (c.x, c.y),
        Some(GuiHandle::Container(c)) => (c.x, c.y),
        Some(GuiHandle::Selectable(s)) => (s.x, s.y),
        Some(GuiHandle::Spinner(s)) => (s.x, s.y),
        Some(GuiHandle::Label(w)) => (w.x, w.y),
        Some(GuiHandle::Checkbox(w)) => (w.x, w.y),
        Some(GuiHandle::Textbox(w)) => (w.x, w.y),
        Some(GuiHandle::Dropdown(w)) => (w.x, w.y),
        Some(GuiHandle::RadioGroup(w)) => (w.x, w.y),
        Some(GuiHandle::Slider(w)) => (w.x, w.y),
        Some(GuiHandle::ProgressBar(w)) => (w.x, w.y),
        Some(GuiHandle::Separator(w)) => (w.x, w.y),
        Some(GuiHandle::Image(w)) => (w.x, w.y),
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(format!(
                "gui_get_pos: handle {} is a window",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "gui_get_pos: unknown handle {}",
                id
            )));
        }
    };

    R::ok(R::tuple(vec![R::from_i64(x as i64), R::from_i64(y as i64)]))
}

// ---- z-order ----------------------------------------------------------------

/// Sets `handle`'s draw order among its window's widgets. Higher `z` draws
/// on top of lower `z` when positions overlap; widgets with equal `z` draw
/// in creation order (later created = on top).
#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int -> result[null]))]
pub fn gui_set_z<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, z: i64) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_z") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    let z = z as i32;

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(w)) => w.z = z,
        Some(GuiHandle::Hyperlink(h)) => h.z = z,
        Some(GuiHandle::Canvas(_)) => {
            return R::err(R::from_string("gui_set_z: canvases have no z-order".to_string()));
        }
        Some(GuiHandle::Container(_)) => {
            return R::err(R::from_string("gui_set_z: containers have no z-order".to_string()));
        }
        Some(GuiHandle::Selectable(s)) => s.z = z,
        Some(GuiHandle::Spinner(s)) => s.z = z,
        Some(GuiHandle::Label(w)) => w.z = z,
        Some(GuiHandle::Checkbox(w)) => w.z = z,
        Some(GuiHandle::Textbox(w)) => w.z = z,
        Some(GuiHandle::Dropdown(w)) => w.z = z,
        Some(GuiHandle::RadioGroup(w)) => w.z = z,
        Some(GuiHandle::Slider(w)) => w.z = z,
        Some(GuiHandle::ProgressBar(w)) => w.z = z,
        Some(GuiHandle::Separator(w)) => w.z = z,
        Some(GuiHandle::Image(w)) => w.z = z,
        Some(GuiHandle::Grid(_)) | Some(GuiHandle::Scroll(_)) => {
            return R::err(R::from_string("gui_set_z: parents have no z-order".to_string()));
        }
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(format!(
                "gui_set_z: handle {} is a window and has no z-level - z-level controls draw order between widgets inside a window, not between windows",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!("gui_set_z: unknown handle {}", id)));
        }
    }

    R::ok(R::null())
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[int]))]
pub fn gui_get_z<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_get_z") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let z = match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Button(w)) => w.z,
        Some(GuiHandle::Hyperlink(h)) => h.z,
        Some(GuiHandle::Canvas(_)) => {
            return R::err(R::from_string("gui_get_z: canvases have no z-order".to_string()));
        }
        Some(GuiHandle::Container(_)) => {
            return R::err(R::from_string("gui_get_z: containers have no z-order".to_string()));
        }
        Some(GuiHandle::Selectable(s)) => s.z,
        Some(GuiHandle::Spinner(s)) => s.z,
        Some(GuiHandle::Label(w)) => w.z,
        Some(GuiHandle::Checkbox(w)) => w.z,
        Some(GuiHandle::Textbox(w)) => w.z,
        Some(GuiHandle::Dropdown(w)) => w.z,
        Some(GuiHandle::RadioGroup(w)) => w.z,
        Some(GuiHandle::Slider(w)) => w.z,
        Some(GuiHandle::ProgressBar(w)) => w.z,
        Some(GuiHandle::Separator(w)) => w.z,
        Some(GuiHandle::Image(w)) => w.z,
        Some(GuiHandle::Grid(_)) | Some(GuiHandle::Scroll(_)) => {
            return R::err(R::from_string("gui_get_z: parents have no z-order".to_string()));
        }
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(format!(
                "gui_get_z: handle {} is a window",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!("gui_get_z: unknown handle {}", id)));
        }
    };

    R::ok(R::from_i64(z as i64))
}

// ---- removal ----------------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[null]))]
pub fn gui_remove<R: GuiStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_remove") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let window_id = match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Button(w)) => w.window,
        Some(GuiHandle::Hyperlink(h)) => h.window,
        Some(GuiHandle::Grid(g)) => g.window,
        Some(GuiHandle::Scroll(s)) => s.window,
        Some(GuiHandle::Canvas(c)) => c.window,
        Some(GuiHandle::Container(c)) => c.window,
        Some(GuiHandle::Selectable(s)) => s.window,
        Some(GuiHandle::Spinner(s)) => s.window,
        Some(GuiHandle::Label(w)) => w.window,
        Some(GuiHandle::Checkbox(w)) => w.window,
        Some(GuiHandle::Textbox(w)) => w.window,
        Some(GuiHandle::Dropdown(w)) => w.window,
        Some(GuiHandle::RadioGroup(w)) => w.window,
        Some(GuiHandle::Slider(w)) => w.window,
        Some(GuiHandle::ProgressBar(w)) => w.window,
        Some(GuiHandle::Separator(w)) => w.window,
        Some(GuiHandle::Image(w)) => w.window,
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(format!(
                "gui_remove: handle {} is a window - use gui_close instead",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!("gui_remove: unknown handle {}", id)));
        }
    };

    if let Some(GuiHandle::Window(w)) = R::gui_handles(cx).get_mut(&window_id) {
        w.children.retain(|c| *c != id);
    }
    // Removing a parent cascades to its whole subtree (and their texture
    // and size entries), so no orphaned handles or GPU uploads linger.
    // Collect transitively: nested parents reached through children.
    let mut orphans: Vec<u64> = Vec::new();
    let mut stack: Vec<u64> = match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Container(c)) => c.children.clone(),
        Some(GuiHandle::Grid(g)) => g.children.clone(),
        Some(GuiHandle::Scroll(s)) => s.children.clone(),
        _ => Vec::new(),
    };
    let mut seen_cascade = std::collections::HashSet::new();
    while let Some(child) = stack.pop() {
        if !seen_cascade.insert(child) {
            continue;
        }
        orphans.push(child);
        match R::gui_handles_ref(cx).get(&child) {
            Some(GuiHandle::Container(c)) => stack.extend(c.children.iter().copied()),
            Some(GuiHandle::Grid(g)) => stack.extend(g.children.iter().copied()),
            Some(GuiHandle::Scroll(s)) => stack.extend(s.children.iter().copied()),
            _ => {}
        }
    }
    R::gui_handles(cx).remove(&id);
    R::texture_cache(cx).remove(&id);
    R::widget_sizes(cx).remove(&id);
    for orphan in orphans {
        R::gui_handles(cx).remove(&orphan);
        R::texture_cache(cx).remove(&orphan);
        R::widget_sizes(cx).remove(&orphan);
    }

    R::ok(R::null())
}

// ============================================================================
// Window-level setters.
// ============================================================================

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string -> result[null]))]
pub fn gui_window_set_title<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    title: String,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_set_title") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.title = title;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_set_title: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_set_title: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, int -> result[null]))]
pub fn gui_window_set_background<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    r: i64,
    g: i64,
    b: i64,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_set_background") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    for (name, channel) in [("r", r), ("g", g), ("b", b)] {
        if !(0..=255).contains(&channel) {
            return R::err(R::from_string(format!(
                "gui_window_set_background: {} ({}) must be between 0 and 255",
                name, channel
            )));
        }
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.background = (r as u8, g as u8, b as u8);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_set_background: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_set_background: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int -> result[null]))]
pub fn gui_window_set_size<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    width: i64,
    height: i64,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_set_size") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.pending_size = Some((width.max(1) as f32, height.max(1) as f32));
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_set_size: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_set_size: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int -> result[null]))]
pub fn gui_window_set_pos<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    x: i64,
    y: i64,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_set_pos") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.pending_position = Some((x as f32, y as f32));
            w.position = (x as f32, y as f32);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_set_pos: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_set_pos: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), bool -> result[null]))]
pub fn gui_window_set_decorated<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    decorated: bool,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_set_decorated") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.decorated = decorated;
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_set_decorated: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_set_decorated: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), bool -> result[null]))]
pub fn gui_window_fullscreen<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    fullscreen: bool,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_fullscreen") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.pending_fullscreen = Some(fullscreen);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_fullscreen: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_fullscreen: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), bool -> result[null]))]
pub fn gui_window_maximize<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    maximized: bool,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_maximize") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.pending_maximized = Some(maximized);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_maximize: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_maximize: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), bool -> result[null]))]
pub fn gui_window_minimize<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    minimized: bool,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_minimize") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.pending_minimized = Some(minimized);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_minimize: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_minimize: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), int, int, array[int] -> result[null]))]
pub fn gui_window_set_icon<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
    width: i64,
    height: i64,
    rgba: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_window_set_icon") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    if width <= 0 || height <= 0 {
        return R::err(R::from_string(format!(
            "gui_window_set_icon: width ({}) and height ({}) must be positive",
            width, height
        )));
    }

    let rgba = match extract_byte_array::<R>(&rgba, "gui_window_set_icon") {
        Ok(bytes) => bytes,
        Err(e) => return R::err(R::from_string(e)),
    };

    let expected_len = width as usize * height as usize * 4;
    if rgba.len() != expected_len {
        return R::err(R::from_string(format!(
            "gui_window_set_icon: expected {} rgba bytes for a {}x{} icon, got {}",
            expected_len,
            width,
            height,
            rgba.len()
        )));
    }

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.icon = Some((width as u32, height as u32, rgba));
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_window_set_icon: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!(
            "gui_window_set_icon: unknown handle {}",
            id
        ))),
    }
}

// ============================================================================
// The eframe event loop (`gui_run`) - the bulk of the module. The egui drawing
// calls are unchanged from the original; only the value/context touchpoints are
// generalized over `R: GuiStore`. State is snapshotted out of the handle map
// before invoking callbacks so those calls don't alias `cx`.
// ============================================================================

#[cfg(feature = "impls")]
enum WidgetSnapshot {
    Button {
        id: u64,
        label: String,
        x: f32,
        y: f32,
        z: i32,
        font_size: Option<f32>,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Hyperlink {
        id: u64,
        text: String,
        url: String,
        x: f32,
        y: f32,
        z: i32,
        font_size: Option<f32>,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Selectable {
        id: u64,
        text: String,
        selected: bool,
        x: f32,
        y: f32,
        z: i32,
        font_size: Option<f32>,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Canvas {
        id: u64,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        z: i32,
        commands: Vec<DrawCmd>,
    },
    Spinner {
        id: u64,
        x: f32,
        y: f32,
        size: f32,
        z: i32,
    },
    Label {
        id: u64,
        text: String,
        x: f32,
        y: f32,
        z: i32,
        font_size: Option<f32>,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Checkbox {
        id: u64,
        label: String,
        x: f32,
        y: f32,
        checked: bool,
        z: i32,
        font_size: Option<f32>,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Textbox {
        id: u64,
        text: String,
        x: f32,
        y: f32,
        width: f32,
        multiline: bool,
        height: f32,
        z: i32,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Dropdown {
        id: u64,
        options: Vec<String>,
        selected: usize,
        x: f32,
        y: f32,
        width: f32,
        z: i32,
        font_size: Option<f32>,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    RadioGroup {
        id: u64,
        options: Vec<String>,
        selected: usize,
        x: f32,
        y: f32,
        z: i32,
        font_size: Option<f32>,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Slider {
        id: u64,
        value: f64,
        min: f64,
        max: f64,
        x: f32,
        y: f32,
        width: f32,
        drag_only: bool,
        z: i32,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    ProgressBar {
        id: u64,
        value: f32,
        x: f32,
        y: f32,
        width: f32,
        z: i32,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Separator {
        id: u64,
        x: f32,
        y: f32,
        width: f32,
        z: i32,
        color: Option<(u8, u8, u8)>,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
    Image {
        id: u64,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_width: u32,
        texture_height: u32,
        rgba: std::sync::Arc<Vec<u8>>,
        version: u64,
        z: i32,
        bg_color: Option<(u8, u8, u8)>,
        tooltip: Option<String>,
    },
}

#[cfg(feature = "impls")]
impl WidgetSnapshot {
    /// Draw order among a window's widgets: higher draws on top. Used to
    /// stable-sort snapshots before rendering, so widgets with equal z keep
    /// drawing in creation order (the order `sort_by_key` preserves for
    /// equal keys).
    fn z(&self) -> i32 {
        match self {
            WidgetSnapshot::Button { z, .. }
            | WidgetSnapshot::Hyperlink { z, .. }
            | WidgetSnapshot::Selectable { z, .. }
            | WidgetSnapshot::Canvas { z, .. }
            | WidgetSnapshot::Spinner { z, .. }
            | WidgetSnapshot::Label { z, .. }
            | WidgetSnapshot::Checkbox { z, .. }
            | WidgetSnapshot::Textbox { z, .. }
            | WidgetSnapshot::Dropdown { z, .. }
            | WidgetSnapshot::RadioGroup { z, .. }
            | WidgetSnapshot::Slider { z, .. }
            | WidgetSnapshot::ProgressBar { z, .. }
            | WidgetSnapshot::Separator { z, .. }
            | WidgetSnapshot::Image { z, .. } => *z,
        }
    }
}

#[cfg(feature = "impls")]
/// Renders one window's background and widgets for the current frame, and
/// dispatches any callbacks triggered by this frame's interactions.
///
/// `ctx` must be scoped to the viewport `window_id` corresponds to: the
/// root's own `Context` for the root window, or the `Context` handed to a
/// `show_viewport_immediate` closure for a secondary window. Every widget is
/// drawn as its own `egui::Area`, so this same code works unmodified for
/// either case - `Area::show` is always `Context`-based, unlike panels.
fn render_window<R: GuiStore>(cx: &mut R::Cx, ctx: &egui::Context, window_id: u64, span: R::Span) {
    let Some(GuiHandle::Window(win)) = R::gui_handles_ref(cx).get(&window_id) else {
        return;
    };
    let background = win.background;
    let children = win.children.clone();

    // Splice visible parents' children into the flat walk, recursing
    // through nesting, so contained widgets snapshot and render exactly
    // like top-level ones (at the positions the layout pass computed).
    // Hidden parents hide their whole subtree. Scroll viewports additionally
    // filter to the visible slice: children outside [scroll_y,
    // scroll_y + height] never snapshot, so long lists cost only their
    // visible rows per frame.
    let sizes_now = R::widget_sizes(cx).clone();
    let children: Vec<u64> = {
        let mut flat = Vec::with_capacity(children.len());
        let mut stack: Vec<u64> = children.into_iter().rev().collect();
        let mut seen = std::collections::HashSet::new();
        while let Some(id) = stack.pop() {
            if !seen.insert(id) {
                continue;
            }
            match R::gui_handles_ref(cx).get(&id) {
                Some(GuiHandle::Container(c)) if c.visible => {
                    stack.extend(c.children.iter().rev().copied());
                }
                Some(GuiHandle::Container(_)) => {}
                Some(GuiHandle::Grid(g)) if g.visible => {
                    stack.extend(g.children.iter().rev().copied());
                }
                Some(GuiHandle::Grid(_)) => {}
                Some(GuiHandle::Scroll(s)) if s.visible => {
                    let top = s.y + s.scroll_y;
                    let bottom = top + s.height;
                    let mut in_view: Vec<u64> = Vec::new();
                    for child in &s.children {
                        let h = sizes_now.get(child).copied().unwrap_or((0.0, 0.0)).1;
                        let y = R::gui_handles_ref(cx)
                            .get(child)
                            .and_then(child_pos)
                            .map(|p| p.1)
                            .unwrap_or(top);
                        if y + h > top && y < bottom {
                            in_view.push(*child);
                        }
                    }
                    stack.extend(in_view.into_iter().rev());
                }
                Some(GuiHandle::Scroll(_)) => {}
                _ => flat.push(id),
            }
        }
        flat
    };
    let enter_pressed = ctx.input(|i| i.key_pressed(egui::Key::Enter));

    // Read the actual viewport rect so we can sync size back to WindowState.
    let viewport_rect = ctx
        .input(|i| i.raw.screen_rect)
        .unwrap_or(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(4096.0, 4096.0),
        ));
    // Sync actual viewport size back to WindowState so getters return
    // the real dimensions (accounts for user resizing the window).
    if let Some(GuiHandle::Window(w)) = R::gui_handles(cx).get_mut(&window_id) {
        w.width = viewport_rect.width();
        w.height = viewport_rect.height();
    }

    // Background fill: a full-viewport Area at `Order::Background`, painted
    // manually, rather than a panel. This sidesteps relying on the exact
    // panel API (which has shifted across egui versions) since `Area` is the
    // one drawing primitive every widget here already depends on.
    egui::Area::new(egui::Id::new(("rl_gui_window_bg", window_id)))
        .order(egui::Order::Background)
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                viewport_rect,
                0.0,
                egui::Color32::from_rgb(background.0, background.1, background.2),
            );
        });

    // Drain canvas draw commands up front: each frame's commands replay
    // exactly once, so RL redraws every frame (typically from on_frame).
    let mut drained: HashMap<u64, Vec<DrawCmd>> = HashMap::new();
    for (id, handle) in R::gui_handles(cx).iter_mut() {
        if let GuiHandle::Canvas(c) = handle {
            drained.insert(*id, std::mem::take(&mut c.commands));
        }
    }

    let mut snapshots: Vec<WidgetSnapshot> = children
        .iter()
        .filter_map(|id| match R::gui_handles_ref(cx).get(id) {            Some(GuiHandle::Button(b)) if b.visible => Some(WidgetSnapshot::Button {
                id: *id,
                label: b.label.clone(),
                x: b.x,
                y: b.y,
                z: b.z,
                font_size: b.font_size,
                color: b.color,
                bg_color: b.bg_color,
                tooltip: b.tooltip.clone(),
            }),
            Some(GuiHandle::Hyperlink(h)) if h.visible => Some(WidgetSnapshot::Hyperlink {
                id: *id,
                text: h.text.clone(),
                url: h.url.clone(),
                x: h.x,
                y: h.y,
                z: h.z,
                font_size: h.font_size,
                color: h.color,
                bg_color: h.bg_color,
                tooltip: h.tooltip.clone(),
            }),
            Some(GuiHandle::Selectable(s)) if s.visible => Some(WidgetSnapshot::Selectable {
                id: *id,
                text: s.text.clone(),
                selected: s.selected,
                x: s.x,
                y: s.y,
                z: s.z,
                font_size: s.font_size,
                color: s.color,
                bg_color: s.bg_color,
                tooltip: s.tooltip.clone(),
            }),
            Some(GuiHandle::Canvas(c)) if c.visible => Some(WidgetSnapshot::Canvas {
                id: *id,
                x: c.x,
                y: c.y,
                width: c.width,
                height: c.height,
                z: 0,
                commands: drained.remove(id).unwrap_or_default(),
            }),
            Some(GuiHandle::Spinner(s)) if s.visible => Some(WidgetSnapshot::Spinner {
                id: *id,
                x: s.x,
                y: s.y,
                size: s.size,
                z: s.z,
            }),
            Some(GuiHandle::Label(l)) if l.visible => Some(WidgetSnapshot::Label {
                id: *id,
                text: l.text.clone(),
                x: l.x,
                y: l.y,
                z: l.z,
                font_size: l.font_size,
                color: l.color,
                bg_color: l.bg_color,
                tooltip: l.tooltip.clone(),
            }),
            Some(GuiHandle::Checkbox(c)) if c.visible => Some(WidgetSnapshot::Checkbox {
                id: *id,
                label: c.label.clone(),
                x: c.x,
                y: c.y,
                checked: c.checked,
                z: c.z,
                font_size: c.font_size,
                color: c.color,
                bg_color: c.bg_color,
                tooltip: c.tooltip.clone(),
            }),
            Some(GuiHandle::Textbox(t)) if t.visible => Some(WidgetSnapshot::Textbox {
                id: *id,
                text: t.text.clone(),
                x: t.x,
                y: t.y,
                width: t.width,
                multiline: t.multiline,
                height: t.height,
                z: t.z,
                color: t.color,
                bg_color: t.bg_color,
                tooltip: t.tooltip.clone(),
            }),
            Some(GuiHandle::Dropdown(s)) if s.visible => Some(WidgetSnapshot::Dropdown {
                id: *id,
                options: s.options.clone(),
                selected: s.selected,
                x: s.x,
                y: s.y,
                width: s.width,
                z: s.z,
                font_size: s.font_size,
                color: s.color,
                bg_color: s.bg_color,
                tooltip: s.tooltip.clone(),
            }),
            Some(GuiHandle::RadioGroup(s)) if s.visible => Some(WidgetSnapshot::RadioGroup {
                id: *id,
                options: s.options.clone(),
                selected: s.selected,
                x: s.x,
                y: s.y,
                z: s.z,
                font_size: s.font_size,
                color: s.color,
                bg_color: s.bg_color,
                tooltip: s.tooltip.clone(),
            }),
            Some(GuiHandle::Slider(s)) if s.visible => Some(WidgetSnapshot::Slider {
                id: *id,
                value: s.value,
                min: s.min,
                max: s.max,
                x: s.x,
                y: s.y,
                width: s.width,
                drag_only: s.drag_only,
                z: s.z,
                color: s.color,
                bg_color: s.bg_color,
                tooltip: s.tooltip.clone(),
            }),
            Some(GuiHandle::ProgressBar(p)) if p.visible => Some(WidgetSnapshot::ProgressBar {
                id: *id,
                value: p.value,
                x: p.x,
                y: p.y,
                width: p.width,
                z: p.z,
                color: p.color,
                bg_color: p.bg_color,
                tooltip: p.tooltip.clone(),
            }),
            Some(GuiHandle::Separator(s)) if s.visible => Some(WidgetSnapshot::Separator {
                id: *id,
                x: s.x,
                y: s.y,
                width: s.width,
                z: s.z,
                color: s.color,
                bg_color: s.bg_color,
                tooltip: s.tooltip.clone(),
            }),
            Some(GuiHandle::Image(img)) if img.visible => Some(WidgetSnapshot::Image {
                id: *id,
                x: img.x,
                y: img.y,
                width: img.width,
                height: img.height,
                texture_width: img.rgba.0,
                texture_height: img.rgba.1,
                rgba: img.rgba.2.clone(),
                version: img.version,
                z: img.z,
                bg_color: img.bg_color,
                tooltip: img.tooltip.clone(),
            }),
            _ => None,
        })
        .collect();

    // Stable sort: widgets with equal z keep the relative order they were
    // already in (children's creation order), only differing z reorders
    // them. Higher z draws later, i.e. on top, since each widget is its own
    // Area drawn in this loop's order.
    snapshots.sort_by_key(WidgetSnapshot::z);

    let mut clicked: Vec<u64> = Vec::new();
    let mut changed_checkbox: Vec<(u64, bool)> = Vec::new();
    let mut changed_text: Vec<(u64, String)> = Vec::new();
    let mut submitted: Vec<(u64, String)> = Vec::new();
    let mut changed_selection: Vec<(u64, usize)> = Vec::new();
    let mut changed_value: Vec<(u64, f64)> = Vec::new();

    for snap in &snapshots {
        match snap {
            WidgetSnapshot::Button {
                id,
                label,
                x,
                y,
                font_size,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let mut btn = egui::Button::new(styled_text(label, *font_size, *color));
                if let Some((r, g, b)) = bg_color {
                    btn = btn.fill(egui::Color32::from_rgb(*r, *g, *b));
                }
                let mut resp = egui::Area::new(egui::Id::new(("rl_gui_button", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| ui.add(btn))
                    .inner;
                R::widget_sizes(cx).insert(*id, (resp.rect.width(), resp.rect.height()));
                if let Some(text) = tooltip {
                    resp = resp.on_hover_text(text);
                }
                if resp.clicked() {
                    clicked.push(*id);
                }
            }
            WidgetSnapshot::Hyperlink {
                id,
                text,
                url,
                x,
                y,
                font_size,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                // Clicks open the URL in the system browser (egui
                // built-in); no RL callback fires.
                let link =
                    egui::Hyperlink::from_label_and_url(styled_text(text, *font_size, *color), url);
                let resp = egui::Area::new(egui::Id::new(("rl_gui_hyperlink", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        if let Some((r, g, b)) = bg_color {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(*r, *g, *b))
                                .inner_margin(4.0)
                                .show(ui, |ui| ui.add(link))
                                .inner
                        } else {
                            ui.add(link)
                        }
                    })
                    .inner;
                R::widget_sizes(cx).insert(*id, (resp.rect.width(), resp.rect.height()));
                if let Some(text) = tooltip {
                    resp.on_hover_text(text);
                }
            }
            WidgetSnapshot::Selectable {
                id,
                text,
                selected,
                x,
                y,
                font_size,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let mut resp = egui::Area::new(egui::Id::new(("rl_gui_selectable", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        if let Some((r, g, b)) = bg_color {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(*r, *g, *b))
                                .inner_margin(4.0)
                                .show(ui, |ui| {
                                    ui.selectable_label(
                                        *selected,
                                        styled_text(text, *font_size, *color),
                                    )
                                })
                                .inner
                        } else {
                            ui.selectable_label(
                                *selected,
                                styled_text(text, *font_size, *color),
                            )
                        }
                    })
                    .inner;
                R::widget_sizes(cx).insert(*id, (resp.rect.width(), resp.rect.height()));
                if let Some(text) = tooltip {
                    resp = resp.on_hover_text(text);
                }
                if resp.clicked() {
                    clicked.push(*id);
                }
            }
            WidgetSnapshot::Spinner {
                id,
                x,
                y,
                size,
                ..
            } => {
                egui::Area::new(egui::Id::new(("rl_gui_spinner", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        ui.add(egui::Spinner::new().size(*size));
                    });
                R::widget_sizes(cx).insert(*id, (*size, *size));
            }
            WidgetSnapshot::Canvas {
                id,
                x,
                y,
                width,
                height,
                commands,
                ..
            } => {
                egui::Area::new(egui::Id::new(("rl_gui_canvas", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        ui.set_min_size(egui::vec2(*width, *height));
                        let painter = ui.painter().clone();
                        let origin = ui.min_rect().min;
                        for cmd in commands {
                            match cmd {
                                DrawCmd::Line {
                                    x1, y1, x2, y2, color, thickness,
                                } => {
                                    let (r, g, b) = *color;
                                    painter.line_segment(
                                        [
                                            origin + egui::vec2(*x1, *y1),
                                            origin + egui::vec2(*x2, *y2),
                                        ],
                                        egui::Stroke::new(
                                            *thickness,
                                            egui::Color32::from_rgb(r, g, b),
                                        ),
                                    );
                                }
                                DrawCmd::Rect {
                                    x, y, w, h, color, filled,
                                } => {
                                    let (r, g, b) = *color;
                                    let rect = egui::Rect::from_min_size(
                                        origin + egui::vec2(*x, *y),
                                        egui::vec2(*w, *h),
                                    );
                                    if *filled {
                                        painter.rect_filled(
                                            rect,
                                            0.0,
                                            egui::Color32::from_rgb(r, g, b),
                                        );
                                    } else {
                                        painter.rect_stroke(
                                            rect,
                                            0.0,
                                            egui::Stroke::new(
                                                1.0,
                                                egui::Color32::from_rgb(r, g, b),
                                            ),
                                            egui::StrokeKind::Inside,
                                        );
                                    }
                                }
                                DrawCmd::Circle {
                                    x, y, radius, color, filled,
                                } => {
                                    let (r, g, b) = *color;
                                    let center = origin + egui::vec2(*x, *y);
                                    if *filled {
                                        painter.circle_filled(
                                            center,
                                            *radius,
                                            egui::Color32::from_rgb(r, g, b),
                                        );
                                    } else {
                                        painter.circle_stroke(
                                            center,
                                            *radius,
                                            egui::Stroke::new(
                                                1.0,
                                                egui::Color32::from_rgb(r, g, b),
                                            ),
                                        );
                                    }
                                }
                                DrawCmd::Text {
                                    text, x, y, size, color,
                                } => {
                                    let (r, g, b) = *color;
                                    painter.text(
                                        origin + egui::vec2(*x, *y),
                                        egui::Align2::LEFT_TOP,
                                        text.clone(),
                                        egui::FontId::proportional(*size),
                                        egui::Color32::from_rgb(r, g, b),
                                    );
                                }
                            }
                        }
                    });
                R::widget_sizes(cx).insert(*id, (*width, *height));
            }
            WidgetSnapshot::Label {
                id,
                text,
                x,
                y,
                font_size,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let rich = styled_text(text, *font_size, *color);
                let resp = egui::Area::new(egui::Id::new(("rl_gui_label", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        // Never wrap: the area resizes to content, so wrapped
                        // text would reflow every time the string changes.
                        let widget = egui::Label::new(rich).wrap_mode(egui::TextWrapMode::Extend);
                        if let Some((r, g, b)) = bg_color {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(*r, *g, *b))
                                .inner_margin(4.0)
                                .show(ui, |ui| ui.add(widget));
                        } else {
                            ui.add(widget);
                        }
                    });
                let sz = resp.response.rect.size();
                if let Some(tip) = tooltip {
                    resp.response.on_hover_text(tip);
                }
                R::widget_sizes(cx).insert(*id, (sz.x, sz.y));
            }
            WidgetSnapshot::Checkbox {
                id,
                label,
                x,
                y,
                checked,
                font_size,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let mut checked = *checked;
                let rich = styled_text(label, *font_size, *color);
                let resp = egui::Area::new(egui::Id::new(("rl_gui_checkbox", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        if let Some((r, g, b)) = bg_color {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(*r, *g, *b))
                                .inner_margin(4.0)
                                .show(ui, |ui| ui.checkbox(&mut checked, rich))
                                .inner
                        } else {
                            ui.checkbox(&mut checked, rich)
                        }
                    })
                    .inner;
                R::widget_sizes(cx).insert(*id, (resp.rect.width(), resp.rect.height()));
                let changed = resp.changed();
                resp.on_hover_text(tooltip.clone().unwrap_or_default());
                if changed {
                    changed_checkbox.push((*id, checked));
                }
            }
            WidgetSnapshot::Textbox {
                id,
                text,
                x,
                y,
                width,
                multiline,
                height,
                z: _,
                color,
                bg_color,
                tooltip,
            } => {
                let mut text = text.clone();
                let mut edit = if *multiline {
                    egui::TextEdit::multiline(&mut text)
                } else {
                    egui::TextEdit::singleline(&mut text)
                };
                if let Some((r, g, b)) = color {
                    edit = edit.text_color(egui::Color32::from_rgb(*r, *g, *b));
                }
                if let Some((r, g, b)) = bg_color {
                    edit = edit.frame(
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(*r, *g, *b))
                            .inner_margin(4.0),
                    );
                }
                let sized_edit = edit.desired_width(*width);
                let mut resp = egui::Area::new(egui::Id::new(("rl_gui_textbox", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        ui.add_sized(
                            [*width, if *multiline { *height } else { 20.0 }],
                            sized_edit,
                        )
                    })
                    .inner;
                R::widget_sizes(cx).insert(*id, (resp.rect.width(), resp.rect.height()));
                if let Some(text) = tooltip {
                    resp = resp.on_hover_text(text);
                }
                if !multiline && resp.lost_focus() && enter_pressed {
                    submitted.push((*id, text.clone()));
                }
                if resp.changed() {
                    changed_text.push((*id, text));
                }
            }
            WidgetSnapshot::Dropdown {
                id,
                options,
                selected,
                x,
                y,
                width,
                font_size,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let mut sel = *selected;
                let tooltip_clone = tooltip.clone();
                let resp = egui::Area::new(egui::Id::new(("rl_gui_dropdown", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        let mut combo = |ui: &mut egui::Ui| {
                            egui::ComboBox::from_id_salt(("rl_gui_dropdown_combo", *id))
                                .width(*width)
                                .selected_text(styled_text(
                                    &options.get(sel).cloned().unwrap_or_default(),
                                    *font_size,
                                    *color,
                                ))
                                .show_ui(ui, |ui| {
                                    for (i, opt) in options.iter().enumerate() {
                                        ui.selectable_value(&mut sel, i, opt);
                                    }
                                });
                        };
                        if let Some((r, g, b)) = bg_color {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(*r, *g, *b))
                                .inner_margin(4.0)
                                .show(ui, combo);
                        } else {
                            combo(ui);
                        }
                    });
                let sz = resp.response.rect.size();
                if let Some(text) = &tooltip_clone {
                    resp.response.on_hover_text(text);
                }
                R::widget_sizes(cx).insert(*id, (sz.x, sz.y));
                if sel != *selected {
                    changed_selection.push((*id, sel));
                }
            }
            WidgetSnapshot::RadioGroup {
                id,
                options,
                selected,
                x,
                y,
                font_size,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let mut sel = *selected;
                let tooltip_clone = tooltip.clone();
                let resp = egui::Area::new(egui::Id::new(("rl_gui_radio", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        let mut radios = |ui: &mut egui::Ui| {
                            ui.vertical(|ui| {
                                for (i, opt) in options.iter().enumerate() {
                                    ui.radio_value(
                                        &mut sel,
                                        i,
                                        styled_text(opt, *font_size, *color),
                                    );
                                }
                            });
                        };
                        if let Some((r, g, b)) = bg_color {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(*r, *g, *b))
                                .inner_margin(4.0)
                                .show(ui, radios);
                        } else {
                            radios(ui);
                        }
                    });
                let sz = resp.response.rect.size();
                if let Some(text) = &tooltip_clone {
                    resp.response.on_hover_text(text);
                }
                R::widget_sizes(cx).insert(*id, (sz.x, sz.y));
                if sel != *selected {
                    changed_selection.push((*id, sel));
                }
            }
            WidgetSnapshot::Slider {
                id,
                value,
                min,
                max,
                x,
                y,
                width,
                drag_only,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let mut v = *value;
                let tooltip_clone = tooltip.clone();
                let resp = egui::Area::new(egui::Id::new(("rl_gui_slider", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        let widget = if *drag_only {
                            ui.add(egui::DragValue::new(&mut v).range(*min..=*max))
                        } else {
                            let mut sl = egui::Slider::new(&mut v, *min..=*max).show_value(true);
                            if let Some((r, g, b)) = color {
                                sl = sl.text_color(egui::Color32::from_rgb(*r, *g, *b));
                            }
                            ui.add_sized([*width, 20.0], sl)
                        };
                        paint_bg(ui.painter(), bg_color, widget.rect);
                        widget
                    })
                    .inner;
                R::widget_sizes(cx).insert(*id, (resp.rect.width(), resp.rect.height()));
                let changed = resp.changed();
                resp.on_hover_text(tooltip_clone.unwrap_or_default());
                if changed {
                    changed_value.push((*id, v));
                }
            }
            WidgetSnapshot::ProgressBar {
                id,
                value,
                x,
                y,
                width,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let tooltip_clone = tooltip.clone();
                let mut bar = egui::ProgressBar::new(*value);
                if let Some((r, g, b)) = color {
                    bar = bar.fill(egui::Color32::from_rgb(*r, *g, *b));
                }
                let resp = egui::Area::new(egui::Id::new(("rl_gui_progress", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        let widget = ui.add_sized([*width, 20.0], bar);
                        paint_bg(ui.painter(), bg_color, widget.rect);
                        widget
                    })
                    .inner;
                R::widget_sizes(cx).insert(*id, (resp.rect.width(), resp.rect.height()));
                if let Some(text) = &tooltip_clone {
                    resp.on_hover_text(text);
                }
            }
            WidgetSnapshot::Separator {
                id,
                x,
                y,
                width,
                color,
                bg_color,
                tooltip,
                ..
            } => {
                let tooltip_clone = tooltip.clone();
                let resp = egui::Area::new(egui::Id::new(("rl_gui_separator", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        ui.allocate_ui(egui::vec2(*width, 6.0), |ui| {
                            let rect = ui.available_rect_before_wrap();
                            paint_bg(ui.painter(), bg_color, rect);
                            if let Some((r, g, b)) = color {
                                ui.painter().rect_filled(
                                    rect,
                                    0.0,
                                    egui::Color32::from_rgb(*r, *g, *b),
                                );
                            } else {
                                ui.separator();
                            }
                        });
                    });
                let sz = resp.response.rect.size();
                if let Some(text) = &tooltip_clone {
                    resp.response.on_hover_text(text);
                }
                R::widget_sizes(cx).insert(*id, (sz.x, sz.y));
            }
            WidgetSnapshot::Image {
                id,
                x,
                y,
                width,
                height,
                texture_width,
                texture_height,
                rgba,
                version,
                z: _,
                bg_color,
                tooltip,
            } => {
                // Skip the GPU upload while the pixels are unchanged:
                // static images pay for one upload ever, animated ones
                // pay per changed frame only.
                let cached: Option<(u64, u64)> =
                    R::texture_cache(cx).get(id).copied();
                let tid = match cached {
                    Some((loaded, managed)) if loaded == *version => {
                        egui::TextureId::Managed(managed)
                    }
                    _ => {
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            [*texture_width as usize, *texture_height as usize],
                            rgba,
                        );
                        let texture = ctx.load_texture(
                            format!("rl_gui_image_{}", id),
                            color_image,
                            egui::TextureOptions::default(),
                        );
                        let tid = texture.id();
                        if let egui::TextureId::Managed(managed) = tid {
                            R::texture_cache(cx).insert(*id, (*version, managed));
                        }
                        tid
                    }
                };
                let sized = egui::load::SizedTexture::new(tid, egui::vec2(*width, *height));
                let tooltip_clone = tooltip.clone();
                let resp = egui::Area::new(egui::Id::new(("rl_gui_image", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        let widget = ui.add(egui::Image::from_texture(sized));
                        paint_bg(ui.painter(), bg_color, widget.rect);
                        widget
                    });
                let sz = resp.response.rect.size();
                if let Some(text) = &tooltip_clone {
                    resp.response.on_hover_text(text);
                }
                R::widget_sizes(cx).insert(*id, (sz.x, sz.y));
            }
        }
    }

    for (id, text) in changed_text {
        let callback = match R::gui_handles(cx).get_mut(&id) {
            Some(GuiHandle::Textbox(t)) => {
                t.text = text.clone();
                t.on_change.clone()
            }
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(R::call_value(cx, &cb, &[R::from_string(text)], span));
        }
    }

    for (id, text) in submitted {
        let callback = match R::gui_handles_ref(cx).get(&id) {
            Some(GuiHandle::Textbox(t)) => t.on_submit.clone(),
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(R::call_value(cx, &cb, &[R::from_string(text)], span));
        }
    }

    for (id, checked) in changed_checkbox {
        let callback = match R::gui_handles(cx).get_mut(&id) {
            Some(GuiHandle::Checkbox(c)) => {
                c.checked = checked;
                c.on_change.clone()
            }
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(R::call_value(cx, &cb, &[R::from_bool(checked)], span));
        }
    }

    for (id, sel) in changed_selection {
        let callback = match R::gui_handles(cx).get_mut(&id) {
            Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
                s.selected = sel;
                s.on_change.clone()
            }
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(R::call_value(cx, &cb, &[R::from_i64(sel as i64)], span));
        }
    }

    for (id, v) in changed_value {
        let callback = match R::gui_handles(cx).get_mut(&id) {
            Some(GuiHandle::Slider(s)) => {
                s.value = v;
                s.on_change.clone()
            }
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(R::call_value(cx, &cb, &[R::from_f64(v)], span));
        }
    }

    for id in clicked {
        let callback = match R::gui_handles_ref(cx).get(&id) {
            Some(GuiHandle::Button(b)) => b.on_click.clone(),
            Some(GuiHandle::Selectable(s)) => s.on_click.clone(),
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(R::call_value(cx, &cb, &[], span));
        }
    }

    let key_names: Vec<(String, bool)> = ctx.input(|i| {
        i.events
            .iter()
            .filter_map(|e| match e {
                egui::Event::Key {
                    key,
                    pressed,
                    repeat: false,
                    ..
                } => Some((format!("{:?}", key), *pressed)),
                _ => None,
            })
            .collect()
    });

    if !key_names.is_empty() {
        let on_key = match R::gui_handles_ref(cx).get(&window_id) {
            Some(GuiHandle::Window(w)) => w.on_key.clone(),
            _ => None,
        };
        if let Some(cb) = on_key {
            for (key_name, pressed) in key_names {
                report_callback_err(R::call_value(
                    cx,
                    &cb,
                    &[R::from_string(key_name), R::from_bool(pressed)],
                    span,
                ));
            }
        }
    }

    let mouse_moves: Vec<(i64, i64)> = ctx.input(|i| {
        i.events
            .iter()
            .filter_map(|e| match e {
                egui::Event::PointerMoved(pos) => {
                    Some((pos.x.round() as i64, pos.y.round() as i64))
                }
                _ => None,
            })
            .collect()
    });

    if !mouse_moves.is_empty() {
        let on_mouse_move = match R::gui_handles_ref(cx).get(&window_id) {
            Some(GuiHandle::Window(w)) => w.on_mouse_move.clone(),
            _ => None,
        };
        if let Some(cb) = on_mouse_move {
            for (x, y) in mouse_moves {
                report_callback_err(R::call_value(
                    cx,
                    &cb,
                    &[R::from_i64(x), R::from_i64(y)],
                    span,
                ));
            }
        }
    }

    let dropped: Vec<String> = ctx
        .input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone().map(|p| p.to_string_lossy().into_owned()))
                .collect()
        });
    if !dropped.is_empty() {
        let on_file_drop = match R::gui_handles_ref(cx).get(&window_id) {
            Some(GuiHandle::Window(w)) => w.on_file_drop.clone(),
            _ => None,
        };
        if let Some(cb) = on_file_drop {
            let items: Vec<R::Value> =
                dropped.into_iter().map(|p| R::from_string(p)).collect();
            let paths = R::array(items, TypeAnnotation::String);
            report_callback_err(R::call_value(cx, &cb, &[paths], span));
        }
    }

    let scroll = ctx.input(|i| i.smooth_scroll_delta);
    if scroll.x != 0.0 || scroll.y != 0.0 {
        let on_scroll = match R::gui_handles_ref(cx).get(&window_id) {
            Some(GuiHandle::Window(w)) => w.on_scroll.clone(),
            _ => None,
        };
        if let Some(cb) = on_scroll {
            report_callback_err(R::call_value(
                cx,
                &cb,
                &[
                    R::from_i64(scroll.x.round() as i64),
                    R::from_i64(scroll.y.round() as i64),
                ],
                span,
            ));
        }
    }
}

#[cfg(feature = "impls")]
/// A window's viewport-level state (title/visible/decorated/icon/pending
/// size & position), snapshotted with the one-shot pending fields cleared.
/// Shared by the root window (driven via `ViewportCommand`s) and secondary
/// windows (driven via their `ViewportBuilder`, rebuilt every frame).
struct ViewportState {
    title: String,
    visible: bool,
    decorated: bool,
    icon: Option<(u32, u32, Vec<u8>)>,
    pending_size: Option<(f32, f32)>,
    pending_position: Option<(f32, f32)>,
    pending_fullscreen: Option<bool>,
    pending_maximized: Option<bool>,
    pending_minimized: Option<bool>,
}

#[cfg(feature = "impls")]
fn take_viewport_state<R: GuiStore>(cx: &mut R::Cx, window_id: u64) -> Option<ViewportState> {
    let Some(GuiHandle::Window(w)) = R::gui_handles_ref(cx).get(&window_id) else {
        return None;
    };
    let state = ViewportState {
        title: w.title.clone(),
        visible: w.visible,
        decorated: w.decorated,
        icon: w.icon.clone(),
        pending_size: w.pending_size,
        pending_position: w.pending_position,
        pending_fullscreen: w.pending_fullscreen,
        pending_maximized: w.pending_maximized,
        pending_minimized: w.pending_minimized,
    };

    if (state.pending_size.is_some()
        || state.pending_position.is_some()
        || state.pending_fullscreen.is_some()
        || state.pending_maximized.is_some()
        || state.pending_minimized.is_some())
        && let Some(GuiHandle::Window(w)) = R::gui_handles(cx).get_mut(&window_id)
    {
        w.pending_size = None;
        w.pending_position = None;
        w.pending_fullscreen = None;
        w.pending_maximized = None;
        w.pending_minimized = None;
    }

    Some(state)
}

#[cfg(feature = "impls")]
/// Reads a widget's absolute position. Returns `None` for windows (which
/// position themselves via the viewport, not the layout pass).
fn child_pos<V>(handle: &GuiHandle<V>) -> Option<(f32, f32)> {
    match handle {
        GuiHandle::Window(_) => None,
        GuiHandle::Container(w) => Some((w.x, w.y)),
        GuiHandle::Button(w) => Some((w.x, w.y)),
        GuiHandle::Hyperlink(w) => Some((w.x, w.y)),
        GuiHandle::Grid(w) => Some((w.x, w.y)),
        GuiHandle::Scroll(w) => Some((w.x, w.y)),
        GuiHandle::Canvas(w) => Some((w.x, w.y)),
        GuiHandle::Spinner(w) => Some((w.x, w.y)),
        GuiHandle::Selectable(w) => Some((w.x, w.y)),
        GuiHandle::Label(w) => Some((w.x, w.y)),
        GuiHandle::Checkbox(w) => Some((w.x, w.y)),
        GuiHandle::Textbox(w) => Some((w.x, w.y)),
        GuiHandle::Dropdown(w) => Some((w.x, w.y)),
        GuiHandle::RadioGroup(w) => Some((w.x, w.y)),
        GuiHandle::Slider(w) => Some((w.x, w.y)),
        GuiHandle::ProgressBar(w) => Some((w.x, w.y)),
        GuiHandle::Separator(w) => Some((w.x, w.y)),
        GuiHandle::Image(w) => Some((w.x, w.y)),
    }
}
#[cfg(feature = "impls")]
/// Writes a widget's absolute position. Containers position their
/// children through here every frame; manual `gui_set_pos` on a
/// contained widget loses to the next layout pass.
fn set_widget_pos<V>(handle: &mut GuiHandle<V>, x: f32, y: f32) {
    match handle {
        GuiHandle::Window(w) => {
            w.pending_position = Some((x, y));
        }
        GuiHandle::Button(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Hyperlink(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Grid(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Scroll(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Canvas(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Spinner(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Selectable(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Label(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Checkbox(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Textbox(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Dropdown(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::RadioGroup(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Slider(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::ProgressBar(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Separator(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Image(w) => {
            w.x = x;
            w.y = y;
        }
        GuiHandle::Container(w) => {
            w.x = x;
            w.y = y;
        }
    }
}

#[cfg(feature = "impls")]
/// Positions every container's children from last frame's rendered sizes
/// (unknown sizes read as zero, so the first frame stacks at the origin
/// and settles on the second). Vertical boxes flow top-down, horizontal
/// ones left-to-right; alignment offsets the cross axis against the
/// widest/tallest child. Recursion follows nesting (cycles are refused
/// at `gui_add`, so this always terminates); parents lay out before
/// children so nested origins are current.
fn layout_containers<R: GuiStore>(cx: &mut R::Cx) {
    struct Frame {
        x: f32,
        y: f32,
        horizontal: bool,
        spacing: f32,
        padding: f32,
        align: ContainerAlign,
        children: Vec<u64>,
    }
    fn read_frame<R: GuiStore>(cx: &R::Cx, id: u64) -> Option<Frame> {
        match R::gui_handles_ref(cx).get(&id) {
            Some(GuiHandle::Container(c)) if c.visible => Some(Frame {
                x: c.x,
                y: c.y,
                horizontal: c.horizontal,
                spacing: c.spacing,
                padding: c.padding,
                align: c.align,
                children: c.children.clone(),
            }),
            _ => None,
        }
    }
    fn layout_one<R: GuiStore>(cx: &mut R::Cx, id: u64) {
        let Some(frame) = read_frame::<R>(cx, id) else {
            return;
        };
        let sizes = R::widget_sizes(cx).clone();
        let mut cross_max = 0.0f32;
        for child in &frame.children {
            let (w, h) = sizes.get(child).copied().unwrap_or((0.0, 0.0));
            cross_max = cross_max.max(if frame.horizontal { h } else { w });
        }
        let mut cursor = if frame.horizontal {
            frame.x + frame.padding
        } else {
            frame.y + frame.padding
        };
        let mut plans: Vec<(u64, f32, f32)> = Vec::with_capacity(frame.children.len());
        for child in &frame.children {
            let (w, h) = sizes.get(child).copied().unwrap_or((0.0, 0.0));
            let mine = if frame.horizontal { h } else { w };
            let cross_off = match frame.align {
                ContainerAlign::Start => 0.0,
                ContainerAlign::Center => ((cross_max - mine) / 2.0).max(0.0),
                ContainerAlign::End => (cross_max - mine).max(0.0),
            };
            let (x, y) = if frame.horizontal {
                let x = cursor;
                cursor += w + frame.spacing;
                (x, frame.y + frame.padding + cross_off)
            } else {
                let y = cursor;
                cursor += h + frame.spacing;
                (frame.x + frame.padding + cross_off, y)
            };
            plans.push((*child, x, y));
        }
        for (child, x, y) in plans {
            if let Some(handle) = R::gui_handles(cx).get_mut(&child) {
                set_widget_pos(handle, x, y);
            }
        }
        // (Nested parents are reached by layout_any's own recursion below,
        // so each lays out after its parent positioned it.)
        // Container extents derive bottom-up from laid-out children, so
        // parents stacking this container read a real size next frame.
        // Without this, containers measure zero and their siblings overlap.
        let sizes_now = R::widget_sizes(cx).clone();
        let mut far_x = frame.x;
        let mut far_y = frame.y;
        for child in &frame.children {
            let (w, h) = sizes_now.get(child).copied().unwrap_or((0.0, 0.0));
            if let Some(pos) = R::gui_handles_ref(cx)
                .get(child)
                .and_then(child_pos)
            {
                far_x = far_x.max(pos.0 + w);
                far_y = far_y.max(pos.1 + h);
            }
        }
        R::widget_sizes(cx).insert(
            id,
            (
                (far_x - frame.x + frame.padding).max(0.0),
                (far_y - frame.y + frame.padding).max(0.0),
            ),
        );
    }
    // Entry points: containers sitting directly in a window. Nested ones
    // are reached by recursion, so each lays out after its parent.
    let roots: Vec<u64> = R::gui_handles_ref(cx)
        .values()
        .filter_map(|h| match h {
            GuiHandle::Window(w) => Some(w.children.clone()),
            _ => None,
        })
        .flatten()
        .collect();
    /// Dispatches one layout step to whichever parent kind `id` is.
    /// Non-parents are no-ops, so parents lay out before children and
    /// nested origins are always current.
    #[cfg(feature = "impls")]
    fn layout_any<R: GuiStore>(cx: &mut R::Cx, id: u64) {
        let is_grid = matches!(
            R::gui_handles_ref(cx).get(&id),
            Some(GuiHandle::Grid(_))
        );
        let is_scroll = matches!(
            R::gui_handles_ref(cx).get(&id),
            Some(GuiHandle::Scroll(_))
        );
        if is_grid {
            layout_grid_one::<R>(cx, id);
        } else if is_scroll {
            layout_scroll_one::<R>(cx, id);
        } else {
            layout_one::<R>(cx, id);
        }
        // Recurse into nested parents regardless of kind.
        let kids: Vec<u64> = match R::gui_handles_ref(cx).get(&id) {
            Some(GuiHandle::Container(c)) => c.children.clone(),
            Some(GuiHandle::Grid(g)) => g.children.clone(),
            Some(GuiHandle::Scroll(s)) => s.children.clone(),
            _ => Vec::new(),
        };
        for child in kids {
            layout_any::<R>(cx, child);
        }
    }

    for id in roots {
        layout_any::<R>(cx, id);
    }
}


#[cfg(feature = "impls")]
/// Grid pass: children flow left-to-right into a fixed column count.
/// Each row is as tall as its tallest child, each column as wide as its
/// widest; cells are top-left aligned. Extents derive bottom-up like
/// containers so parents stacking a grid read a real size.
fn layout_grid_one<R: GuiStore>(cx: &mut R::Cx, id: u64) {
    struct Grid {
        x: f32,
        y: f32,
        columns: usize,
        spacing: f32,
        padding: f32,
        children: Vec<u64>,
    }
    let grid = match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Grid(g)) if g.visible => Grid {
            x: g.x,
            y: g.y,
            columns: g.columns.max(1),
            spacing: g.spacing,
            padding: g.padding,
            children: g.children.clone(),
        },
        _ => return,
    };
    let sizes = R::widget_sizes(cx).clone();
    let rows = grid.children.len().div_ceil(grid.columns);
    let mut col_w = vec![0.0f32; grid.columns];
    let mut row_h = vec![0.0f32; rows.max(1)];
    for (i, child) in grid.children.iter().enumerate() {
        let (w, h) = sizes.get(child).copied().unwrap_or((0.0, 0.0));
        let (r, c) = (i / grid.columns, i % grid.columns);
        col_w[c] = col_w[c].max(w);
        row_h[r] = row_h[r].max(h);
    }
    let mut plans: Vec<(u64, f32, f32)> = Vec::with_capacity(grid.children.len());
    let mut y = grid.y + grid.padding;
    for (r, row) in grid.children.chunks(grid.columns).enumerate() {
        let mut x = grid.x + grid.padding;
        for (c, child) in row.iter().enumerate() {
            plans.push((*child, x, y));
            x += col_w[c] + grid.spacing;
        }
        y += row_h[r] + grid.spacing;
    }
    for (child, x, y) in plans {
        if let Some(handle) = R::gui_handles(cx).get_mut(&child) {
            set_widget_pos(handle, x, y);
        }
    }
    let total_w: f32 = col_w.iter().sum::<f32>()
        + grid.spacing * (grid.columns.max(1) - 1) as f32;
    let total_h: f32 = row_h.iter().sum::<f32>()
        + grid.spacing * (rows.max(1) - 1) as f32;
    R::widget_sizes(cx).insert(
        id,
        (
            (total_w + grid.padding * 2.0).max(0.0),
            (total_h + grid.padding * 2.0).max(0.0),
        ),
    );
}

#[cfg(feature = "impls")]
/// Scroll pass: children stack vertically like a box, then the viewport
/// window `[scroll_y, scroll_y + height]` decides nothing here  -  the
/// snapshot walk filters by these same positions (see below). Clamps the
/// offset into range and records the content extent for clamping.
fn layout_scroll_one<R: GuiStore>(cx: &mut R::Cx, id: u64) {
    struct View {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        spacing: f32,
        scroll_y: f32,
        children: Vec<u64>,
    }
    let view = match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Scroll(s)) if s.visible => View {
            x: s.x,
            y: s.y,
            width: s.width,
            height: s.height,
            spacing: s.spacing,
            scroll_y: s.scroll_y,
            children: s.children.clone(),
        },
        _ => return,
    };
    let sizes = R::widget_sizes(cx).clone();
    let mut cursor = view.y;
    let mut plans: Vec<(u64, f32, f32)> = Vec::with_capacity(view.children.len());
    for child in &view.children {
        plans.push((*child, view.x, cursor));
        cursor += sizes.get(child).copied().unwrap_or((0.0, 0.0)).1 + view.spacing;
    }
    for (child, x, y) in plans {
        if let Some(handle) = R::gui_handles(cx).get_mut(&child) {
            set_widget_pos(handle, x, y);
        }
    }
    let bottom = (cursor - view.spacing - view.y).max(0.0);
    let max_off = (bottom - view.height).max(0.0);
    let clamped = view.scroll_y.clamp(0.0, max_off);
    if let Some(GuiHandle::Scroll(s)) = R::gui_handles(cx).get_mut(&id) {
        s.content_bottom = bottom;
        s.scroll_y = clamped;
    }
    R::widget_sizes(cx).insert(id, (view.width, view.height));
}

#[cfg(feature = "impls")]
/// Fires a window's `on_frame` callback, if set. Called once per frame
/// from the event loop; powers polling patterns (worker threads,
/// animations) that must observe every frame.
fn fire_frame_callback<R: GuiStore>(cx: &mut R::Cx, window_id: u64, span: R::Span) {
    let on_frame = match R::gui_handles_ref(cx).get(&window_id) {
        Some(GuiHandle::Window(w)) => w.on_frame.clone(),
        _ => None,
    };
    if let Some(cb) = on_frame {
        report_callback_err(R::call_value(cx, &cb, &[], span));
    }
}

#[cfg(feature = "impls")]
/// Whether any window wants per-frame callbacks. While true the event
/// loop repaints continuously; otherwise it stays on-demand so idle
/// windows cost nothing.
fn any_frame_callback<R: GuiStore>(cx: &R::Cx) -> bool {
    R::gui_handles_ref(cx)
        .values()
        .any(|h| matches!(h, GuiHandle::Window(w) if w.on_frame.is_some()))
}

#[cfg(feature = "impls")]
struct RlGuiApp<'a, R: GuiStore> {
    cx: &'a mut R::Cx,
    window: u64,
    /// The `gui_run` call-site span, threaded to every callback fired from the
    /// event loop (on the VM this is `()`; on the interpreter it anchors any
    /// callback error at the `gui_run(...)` call site).
    span: R::Span,
}

#[cfg(feature = "impls")]
impl<R: GuiStore> eframe::App for RlGuiApp<'_, R> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let cx = &mut *self.cx;
        let span = self.span;

        // Position container children from last frame's sizes before
        // snapshotting anything.
        layout_containers::<R>(cx);

        // If the user clicked the native close button, clean up (and fire
        // `on_close`) now. eframe still closes the native window at the end
        // of this frame regardless - this just makes sure the callback runs
        // and any child widget handles are freed rather than leaked.
        if ctx.input(|i| i.viewport().close_requested()) {
            close_window::<R>(cx, self.window, span);
        }

        let Some(state) = take_viewport_state::<R>(cx, self.window) else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        };

        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(state.visible));
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(state.title));
        ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(state.decorated));
        if let Some(fullscreen) = state.pending_fullscreen {
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen));
        }
        if let Some(maximized) = state.pending_maximized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(maximized));
        }
        if let Some(minimized) = state.pending_minimized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(minimized));
        }
        if let Some(minimized) = state.pending_minimized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(minimized));
        }
        if let Some((width, height)) = state.pending_size {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(width, height)));
        }
        if let Some((x, y)) = state.pending_position {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(x, y)));
        }
        if let Some((icon_width, icon_height, rgba)) = state.icon {
            ctx.send_viewport_cmd(egui::ViewportCommand::Icon(Some(std::sync::Arc::new(
                egui::IconData {
                    rgba,
                    width: icon_width,
                    height: icon_height,
                },
            ))));
        }

        render_window::<R>(cx, &ctx, self.window, span);
        fire_frame_callback::<R>(cx, self.window, span);
        if any_frame_callback::<R>(cx) {
            // Capped continuous repaint: polling UIs animate smoothly
            // without spinning the event loop uncapped.
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }

        // Every other open window becomes its own native viewport, spawned
        // fresh each frame (immediate viewports must be re-requested every
        // frame they should stay visible - stop calling this for an id and
        // its window closes).
        let secondary_window_ids: Vec<u64> = R::gui_handles_ref(cx)
            .iter()
            .filter_map(|(id, handle)| match handle {
                GuiHandle::Window(_) if *id != self.window => Some(*id),
                _ => None,
            })
            .collect();

        for window_id in secondary_window_ids {
            let Some(state) = take_viewport_state::<R>(cx, window_id) else {
                continue;
            };

            let mut builder = egui::ViewportBuilder::default()
                .with_title(state.title)
                .with_decorations(state.decorated)
                .with_visible(state.visible);
            if let Some((width, height)) = state.pending_size {
                builder = builder.with_inner_size([width, height]);
            }
            if let Some((x, y)) = state.pending_position {
                builder = builder.with_position([x, y]);
            }
            if let Some((icon_width, icon_height, rgba)) = state.icon {
                builder = builder.with_icon(egui::IconData {
                    rgba,
                    width: icon_width,
                    height: icon_height,
                });
            }
            if let Some(fullscreen) = state.pending_fullscreen {
                builder = builder.with_fullscreen(fullscreen);
            }
            if let Some(maximized) = state.pending_maximized {
                builder = builder.with_maximized(maximized);
            }

            let viewport_id = egui::ViewportId::from_hash_of(("rl_gui_window", window_id));
            ctx.show_viewport_immediate(viewport_id, builder, |child_ctx, _class| {
                if child_ctx.input(|i| i.viewport().close_requested()) {
                    close_window::<R>(&mut *cx, window_id, span);
                    return;
                }
                render_window::<R>(&mut *cx, child_ctx, window_id, span);
                fire_frame_callback::<R>(&mut *cx, window_id, span);
                if state.pending_minimized == Some(true) {
                    child_ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                }
                if any_frame_callback::<R>(&*cx) {
                    child_ctx.request_repaint_after(std::time::Duration::from_millis(16));
                }
            });
        }

        if *R::gui_quit_requested(cx) {
            *R::gui_quit_requested(cx) = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // ctx.request_repaint();
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[null]))]
pub fn gui_run<R: GuiStore>(cx: &mut R::Cx, window: R::Value, span: R::Span) -> R::Value {
    let window_id = match extract_handle::<R>(&window, "gui_run") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let (title, width, height, position, decorated, icon) =
        match R::gui_handles_ref(cx).get(&window_id) {
            Some(GuiHandle::Window(w)) => {
                let (width, height) = w.pending_size.unwrap_or((w.width, w.height));
                (
                    w.title.clone(),
                    width,
                    height,
                    w.pending_position,
                    w.decorated,
                    w.icon.clone(),
                )
            }
            Some(_) => {
                return R::err(R::from_string(format!(
                    "gui_run: handle {} is not a window",
                    window_id
                )));
            }
            None => {
                return R::err(R::from_string(format!(
                    "gui_run: unknown handle {}",
                    window_id
                )));
            }
        };

    let mut viewport = egui::ViewportBuilder::default()
        .with_title(title.clone())
        .with_inner_size([width, height])
        .with_decorations(decorated);
    if let Some((x, y)) = position {
        viewport = viewport.with_position([x, y]);
    }
    if let Some((icon_width, icon_height, rgba)) = icon {
        viewport = viewport.with_icon(egui::IconData {
            rgba,
            width: icon_width,
            height: icon_height,
        });
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let result = eframe::run_native(
        &title,
        options,
        Box::new(move |_cc| {
            Ok(Box::new(RlGuiApp::<R> {
                cx,
                window: window_id,
                span,
            }))
        }),
    );

    match result {
        Ok(()) => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!("gui_run: {}", e))),
    }
}

// ============================================================================
// Root-window teardown / quit.
// ============================================================================

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[null]))]
pub fn gui_close<R: GuiStore>(cx: &mut R::Cx, window: R::Value, span: R::Span) -> R::Value {
    let id = match extract_handle::<R>(&window, "gui_close") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Window(_)) => {
            close_window::<R>(cx, id, span);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "gui_close: handle {} is not a window",
            id
        ))),
        None => R::err(R::from_string(format!("gui_close: unknown handle {}", id))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig( -> null))]
pub fn gui_quit<R: GuiStore>(cx: &mut R::Cx) -> R::Value {
    *R::gui_quit_requested(cx) = true;
    R::null()
}

// ---- clipboard (system-wide, no window needed) --------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(string -> result[null]))]
pub fn gui_clipboard_copy<R: GuiStore>(text: String) -> R::Value {
    match arboard::Clipboard::new() {
        Ok(mut cb) => match cb.set_text(text) {
            Ok(()) => R::ok(R::null()),
            Err(e) => R::err(R::from_string(format!("gui_clipboard_copy: {e}"))),
        },
        Err(e) => R::err(R::from_string(format!("gui_clipboard_copy: {e}"))),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig( -> result[string]))]
pub fn gui_clipboard_paste<R: GuiStore>() -> R::Value {
    match arboard::Clipboard::new() {
        Ok(mut cb) => match cb.get_text() {
            Ok(text) => R::ok(R::from_string(text)),
            Err(e) => R::err(R::from_string(format!("gui_clipboard_paste: {e}"))),
        },
        Err(e) => R::err(R::from_string(format!("gui_clipboard_paste: {e}"))),
    }
}

// ---- style setters ----------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), float -> result[null]))]
pub fn gui_set_font_size<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, size: f64) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_font_size") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(b)) => { b.font_size = Some(size as f32); }
        Some(GuiHandle::Hyperlink(h)) => { h.font_size = Some(size as f32); }
        Some(GuiHandle::Grid(_)) | Some(GuiHandle::Scroll(_)) => {
            return R::err(R::from_string("gui_set_font_size: parents have no text".to_string()));
        }
        Some(GuiHandle::Canvas(_)) => {
            return R::err(R::from_string("gui_set_font_size: canvases have no text".to_string()));
        }
        Some(GuiHandle::Container(_)) => {
            return R::err(R::from_string("gui_set_font_size: containers have no text".to_string()));
        }
        Some(GuiHandle::Selectable(s)) => { s.font_size = Some(size as f32); }
        Some(GuiHandle::Label(l)) => { l.font_size = Some(size as f32); }
        Some(GuiHandle::Checkbox(c)) => { c.font_size = Some(size as f32); }
        Some(GuiHandle::Textbox(t)) => { t.font_size = Some(size as f32); }
        Some(GuiHandle::Dropdown(s)) => { s.font_size = Some(size as f32); }
        Some(GuiHandle::RadioGroup(s)) => { s.font_size = Some(size as f32); }
        Some(GuiHandle::Slider(s)) => { s.font_size = Some(size as f32); }
        Some(GuiHandle::ProgressBar(p)) => { p.font_size = Some(size as f32); }
        Some(GuiHandle::Separator(s)) => { s.font_size = Some(size as f32); }
        Some(GuiHandle::Image(i)) => { i.font_size = Some(size as f32); }
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(
                "gui_set_font_size: cannot set font size on a window".into(),
            ));
        }
        Some(GuiHandle::Spinner(_)) => {}
        None => {
            return R::err(R::from_string(format!(
                "gui_set_font_size: unknown handle {}",
                id
            )));
        }
    }

    R::ok(R::null())
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), byte, byte, byte -> result[null]))]
pub fn gui_set_color<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    r: u8,
    g: u8,
    b: u8,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_color") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(bh)) => { bh.color = Some((r, g, b)); }
        Some(GuiHandle::Hyperlink(h)) => { h.color = Some((r, g, b)); }
        Some(GuiHandle::Grid(_)) | Some(GuiHandle::Scroll(_)) => {
            return R::err(R::from_string("gui_set_color: parents have no text".to_string()));
        }
        Some(GuiHandle::Canvas(_)) => {
            return R::err(R::from_string("gui_set_color: canvases have no text".to_string()));
        }
        Some(GuiHandle::Container(_)) => {
            return R::err(R::from_string("gui_set_color: containers have no text".to_string()));
        }
        Some(GuiHandle::Selectable(s)) => { s.color = Some((r, g, b)); }
        Some(GuiHandle::Label(l)) => { l.color = Some((r, g, b)); }
        Some(GuiHandle::Checkbox(c)) => { c.color = Some((r, g, b)); }
        Some(GuiHandle::Textbox(t)) => { t.color = Some((r, g, b)); }
        Some(GuiHandle::Dropdown(s)) => { s.color = Some((r, g, b)); }
        Some(GuiHandle::RadioGroup(s)) => { s.color = Some((r, g, b)); }
        Some(GuiHandle::Slider(s)) => { s.color = Some((r, g, b)); }
        Some(GuiHandle::ProgressBar(p)) => { p.color = Some((r, g, b)); }
        Some(GuiHandle::Separator(s)) => { s.color = Some((r, g, b)); }
        Some(GuiHandle::Image(i)) => { i.color = Some((r, g, b)); }
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(
                "gui_set_color: cannot set color on a window".into(),
            ));
        }
        Some(GuiHandle::Spinner(_)) => {}
        None => {
            return R::err(R::from_string(format!(
                "gui_set_color: unknown handle {}",
                id
            )));
        }
    }

    R::ok(R::null())
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), byte, byte, byte -> result[null]))]
pub fn gui_set_bg_color<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    r: u8,
    g: u8,
    b: u8,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_bg_color") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(bh)) => { bh.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Hyperlink(h)) => { h.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Grid(_)) | Some(GuiHandle::Scroll(_)) => {
            return R::err(R::from_string("gui_set_bg_color: parents have no background".to_string()));
        }
        Some(GuiHandle::Canvas(_)) => {
            return R::err(R::from_string("gui_set_bg_color: canvases have no background".to_string()));
        }
        Some(GuiHandle::Container(_)) => {
            return R::err(R::from_string("gui_set_bg_color: containers have no background".to_string()));
        }
        Some(GuiHandle::Selectable(s)) => { s.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Label(l)) => { l.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Checkbox(c)) => { c.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Textbox(t)) => { t.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Dropdown(s)) => { s.bg_color = Some((r, g, b)); }
        Some(GuiHandle::RadioGroup(s)) => { s.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Slider(s)) => { s.bg_color = Some((r, g, b)); }
        Some(GuiHandle::ProgressBar(p)) => { p.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Separator(s)) => { s.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Image(i)) => { i.bg_color = Some((r, g, b)); }
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(
                "gui_set_bg_color: cannot set background color on a window".into(),
            ));
        }
        Some(GuiHandle::Spinner(_)) => {}
        None => {
            return R::err(R::from_string(format!(
                "gui_set_bg_color: unknown handle {}",
                id
            )));
        }
    }

    R::ok(R::null())
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), string -> result[null]))]
pub fn gui_set_tooltip<R: GuiStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    text: String,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_tooltip") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(bh)) => { bh.tooltip = Some(text); }
        Some(GuiHandle::Hyperlink(h)) => { h.tooltip = Some(text); }
        Some(GuiHandle::Grid(_)) | Some(GuiHandle::Scroll(_)) => {
            return R::err(R::from_string("gui_set_tooltip: parents show no tooltip".to_string()));
        }
        Some(GuiHandle::Canvas(_)) => {
            return R::err(R::from_string("gui_set_tooltip: canvases show no tooltip".to_string()));
        }
        Some(GuiHandle::Container(_)) => {
            return R::err(R::from_string("gui_set_tooltip: containers show no tooltip".to_string()));
        }
        Some(GuiHandle::Selectable(s)) => { s.tooltip = Some(text); }
        Some(GuiHandle::Label(l)) => { l.tooltip = Some(text); }
        Some(GuiHandle::Checkbox(c)) => { c.tooltip = Some(text); }
        Some(GuiHandle::Textbox(t)) => { t.tooltip = Some(text); }
        Some(GuiHandle::Dropdown(s)) => { s.tooltip = Some(text); }
        Some(GuiHandle::RadioGroup(s)) => { s.tooltip = Some(text); }
        Some(GuiHandle::Slider(s)) => { s.tooltip = Some(text); }
        Some(GuiHandle::ProgressBar(p)) => { p.tooltip = Some(text); }
        Some(GuiHandle::Separator(s)) => { s.tooltip = Some(text); }
        Some(GuiHandle::Image(i)) => { i.tooltip = Some(text); }
        Some(GuiHandle::Window(_)) => {
            return R::err(R::from_string(
                "gui_set_tooltip: cannot set tooltip on a window".into(),
            ));
        }
        Some(GuiHandle::Spinner(_)) => {}
        None => {
            return R::err(R::from_string(format!(
                "gui_set_tooltip: unknown handle {}",
                id
            )));
        }
    }

    R::ok(R::null())
}

// ---- window query -----------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[array[float]]))]
pub fn gui_get_window_size<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
) -> Result<Vec<f64>, String> {
    let id = extract_handle::<R>(&window, "gui_get_window_size")?;
    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Window(w)) => Ok(vec![w.width as f64, w.height as f64]),
        Some(_) => Err(format!(
            "gui_get_window_size: handle {} is not a window",
            id
        )),
        None => Err(format!(
            "gui_get_window_size: unknown handle {}",
            id
        )),
    }
}

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui) -> result[array[float]]))]
pub fn gui_get_window_pos<R: GuiStore>(
    cx: &mut R::Cx,
    window: R::Value,
) -> Result<Vec<f64>, String> {
    let id = extract_handle::<R>(&window, "gui_get_window_pos")?;
    match R::gui_handles_ref(cx).get(&id) {
        Some(GuiHandle::Window(w)) => Ok(vec![w.position.0 as f64, w.position.1 as f64]),
        Some(_) => Err(format!(
            "gui_get_window_pos: handle {} is not a window",
            id
        )),
        None => Err(format!(
            "gui_get_window_pos: unknown handle {}",
            id
        )),
    }
}

rl_std_core::native_module!("gui";
    bound: GuiStore;
    funcs: [
        gui_clipboard_copy,
        gui_clipboard_paste,
        gui_window,
        gui_button,
        gui_hyperlink,
        gui_spinner,
        gui_selectable,
        gui_set_selected,
        gui_is_selected,
        gui_canvas,
        gui_set_canvas_size,
        gui_get_canvas_size,
        gui_draw_line,
        gui_draw_rect,
        gui_draw_circle,
        gui_draw_text,
        gui_label,
        gui_checkbox,
        gui_textbox,
        gui_textarea,
        gui_dropdown,
        gui_radio_group,
        gui_slider,
        gui_number_input,
        gui_progress_bar,
        gui_separator,
        gui_image,
        gui_update_image,
        gui_vbox,
        gui_hbox,
        gui_add,
        gui_grid,
        gui_scroll,
        gui_scroll_to,
        gui_scroll_pos,
        gui_detach,
        gui_set_spacing,
        gui_set_padding,
        gui_set_align,
        gui_set_text,
        gui_get_text,
        gui_set_visible,
        gui_is_visible,
        gui_on_click,
        gui_on_change,
        gui_on_submit,
        gui_on_key,
        gui_on_mouse_move,
        gui_on_file_drop,
        gui_on_scroll,
        gui_on_close,
        gui_on_frame,
        gui_is_checked,
        gui_set_checked,
        gui_get_selected_index,
        gui_set_selected_index,
        gui_get_selected,
        gui_get_value,
        gui_set_value,
        gui_set_progress,
        gui_get_progress,
        gui_set_pos,
        gui_get_pos,
        gui_set_z,
        gui_get_z,
        gui_remove,
        gui_window_set_title,
        gui_window_set_background,
        gui_window_set_size,
        gui_window_set_pos,
        gui_window_set_decorated,
        gui_window_fullscreen,
        gui_window_maximize,
        gui_window_minimize,
        gui_window_set_icon,
        gui_run,
        gui_close,
        gui_quit,
        // style
        gui_set_font_size,
        gui_set_color,
        gui_set_bg_color,
        gui_set_tooltip,
        // window query
        gui_get_window_size,
        gui_get_window_pos,
    ],
);
