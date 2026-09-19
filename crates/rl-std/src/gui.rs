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
    Label(LabelState),
    Checkbox(CheckboxState<V>),
    Textbox(TextboxState<V>),
    Dropdown(SelectState<V>),
    RadioGroup(SelectState<V>),
    Slider(SliderState<V>),
    ProgressBar(ProgressState),
    Separator(SeparatorState),
    Image(ImageState),
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
    /// Called with no arguments when this window closes, whether via
    /// `gui_close` or the native close button.
    pub on_close: Option<V>,
    /// Called with the pressed key's name (e.g. `"Enter"`, `"Escape"`) for
    /// every non-repeat key press while this window has focus.
    pub on_key: Option<V>,
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
            on_close: None,
            on_key: None,
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
        Some(GuiHandle::Label(l)) => {
            l.text = text;
            R::ok(R::null())
        }
        Some(GuiHandle::Textbox(t)) => {
            t.text = text;
            R::ok(R::null())
        }
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
        Some(_) => R::err(R::from_string(format!(
            "gui_on_click: handle {} is not a button",
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

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), callback(string -> null) -> result[null]))]
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
        Some(GuiHandle::Label(w)) => w.z = z,
        Some(GuiHandle::Checkbox(w)) => w.z = z,
        Some(GuiHandle::Textbox(w)) => w.z = z,
        Some(GuiHandle::Dropdown(w)) => w.z = z,
        Some(GuiHandle::RadioGroup(w)) => w.z = z,
        Some(GuiHandle::Slider(w)) => w.z = z,
        Some(GuiHandle::ProgressBar(w)) => w.z = z,
        Some(GuiHandle::Separator(w)) => w.z = z,
        Some(GuiHandle::Image(w)) => w.z = z,
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
        Some(GuiHandle::Label(w)) => w.z,
        Some(GuiHandle::Checkbox(w)) => w.z,
        Some(GuiHandle::Textbox(w)) => w.z,
        Some(GuiHandle::Dropdown(w)) => w.z,
        Some(GuiHandle::RadioGroup(w)) => w.z,
        Some(GuiHandle::Slider(w)) => w.z,
        Some(GuiHandle::ProgressBar(w)) => w.z,
        Some(GuiHandle::Separator(w)) => w.z,
        Some(GuiHandle::Image(w)) => w.z,
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
    R::gui_handles(cx).remove(&id);

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

    let mut snapshots: Vec<WidgetSnapshot> = children
        .iter()
        .filter_map(|id| match R::gui_handles_ref(cx).get(id) {
            Some(GuiHandle::Button(b)) if b.visible => Some(WidgetSnapshot::Button {
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
                if let Some(text) = tooltip {
                    resp = resp.on_hover_text(text);
                }
                if resp.clicked() {
                    clicked.push(*id);
                }
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
                        if let Some((r, g, b)) = bg_color {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(*r, *g, *b))
                                .inner_margin(4.0)
                                .show(ui, |ui| ui.label(rich));
                        } else {
                            ui.label(rich);
                        }
                    });
                if let Some(tip) = tooltip {
                    resp.response.on_hover_text(tip);
                }
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
                if let Some(text) = &tooltip_clone {
                    resp.response.on_hover_text(text);
                }
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
                if let Some(text) = &tooltip_clone {
                    resp.response.on_hover_text(text);
                }
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
                if let Some(text) = &tooltip_clone {
                    resp.response.on_hover_text(text);
                }
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
                z: _,
                bg_color,
                tooltip,
            } => {
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [*texture_width as usize, *texture_height as usize],
                    rgba,
                );
                let texture = ctx.load_texture(
                    format!("rl_gui_image_{}", id),
                    color_image,
                    egui::TextureOptions::default(),
                );
                let sized =
                    egui::load::SizedTexture::new(texture.id(), egui::vec2(*width, *height));
                let tooltip_clone = tooltip.clone();
                let resp = egui::Area::new(egui::Id::new(("rl_gui_image", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        let widget = ui.add(egui::Image::from_texture(sized));
                        paint_bg(ui.painter(), bg_color, widget.rect);
                        widget
                    });
                if let Some(text) = &tooltip_clone {
                    resp.response.on_hover_text(text);
                }
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
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(R::call_value(cx, &cb, &[], span));
        }
    }

    let key_names: Vec<String> = ctx.input(|i| {
        i.events
            .iter()
            .filter_map(|e| match e {
                egui::Event::Key {
                    key,
                    pressed: true,
                    repeat: false,
                    ..
                } => Some(format!("{:?}", key)),
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
            for key_name in key_names {
                report_callback_err(R::call_value(cx, &cb, &[R::from_string(key_name)], span));
            }
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
    };

    if (state.pending_size.is_some() || state.pending_position.is_some())
        && let Some(GuiHandle::Window(w)) = R::gui_handles(cx).get_mut(&window_id)
    {
        w.pending_size = None;
        w.pending_position = None;
    }

    Some(state)
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

            let viewport_id = egui::ViewportId::from_hash_of(("rl_gui_window", window_id));
            ctx.show_viewport_immediate(viewport_id, builder, |child_ctx, _class| {
                if child_ctx.input(|i| i.viewport().close_requested()) {
                    close_window::<R>(&mut *cx, window_id, span);
                    return;
                }
                render_window::<R>(&mut *cx, child_ctx, window_id, span);
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

// ---- style setters ----------------------------------------------------------

#[native_fn(module = "gui", bound = "GuiStore", sig(handle(Gui), float -> result[null]))]
pub fn gui_set_font_size<R: GuiStore>(cx: &mut R::Cx, handle: R::Value, size: f64) -> R::Value {
    let id = match extract_handle::<R>(&handle, "gui_set_font_size") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::gui_handles(cx).get_mut(&id) {
        Some(GuiHandle::Button(b)) => { b.font_size = Some(size as f32); }
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
        gui_window,
        gui_button,
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
        gui_set_text,
        gui_get_text,
        gui_set_visible,
        gui_is_visible,
        gui_on_click,
        gui_on_change,
        gui_on_submit,
        gui_on_key,
        gui_on_close,
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
