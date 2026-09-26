use crate::entry::{FnEntry, StdEntry};

mod gui_button;
mod gui_clipboard_copy;
mod gui_clipboard_paste;
mod gui_canvas;
mod gui_draw_line;
mod gui_draw_rect;
mod gui_draw_circle;
mod gui_draw_text;
mod gui_checkbox;
mod gui_close;
mod gui_dropdown;
mod gui_get_pos;
mod gui_get_progress;
mod gui_get_selected;
mod gui_get_selected_index;
mod gui_get_text;
mod gui_get_value;
mod gui_get_window_pos;
mod gui_get_window_size;
mod gui_get_z;
mod gui_hyperlink;
mod gui_image;
mod gui_update_image;
mod gui_vbox;
mod gui_hbox;
mod gui_add;
mod gui_detach;
mod gui_set_spacing;
mod gui_set_padding;
mod gui_set_align;
mod gui_is_selected;
mod gui_is_checked;
mod gui_is_visible;
mod gui_label;
mod gui_number_input;
mod gui_on_change;
mod gui_on_click;
mod gui_on_close;
mod gui_on_file_drop;
mod gui_on_frame;
mod gui_on_key;
mod gui_on_mouse_move;
mod gui_on_scroll;
mod gui_on_submit;
mod gui_progress_bar;
mod gui_quit;
mod gui_radio_group;
mod gui_remove;
mod gui_run;
mod gui_selectable;
mod gui_separator;
mod gui_set_bg_color;
mod gui_set_checked;
mod gui_set_color;
mod gui_set_font_size;
mod gui_set_pos;
mod gui_set_progress;
mod gui_set_selected;
mod gui_set_selected_index;
mod gui_set_text;
mod gui_set_tooltip;
mod gui_set_value;
mod gui_set_visible;
mod gui_set_z;
mod gui_spinner;
mod gui_slider;
mod gui_textarea;
mod gui_textbox;
mod gui_window;
mod gui_window_fullscreen;
mod gui_window_maximize;
mod gui_window_minimize;
mod gui_window_set_background;
mod gui_window_set_decorated;
mod gui_window_set_icon;
mod gui_window_set_pos;
mod gui_window_set_size;
mod gui_window_set_title;

pub static GUI: StdEntry = StdEntry {
    name: "gui",
    description: "a native desktop GUI toolkit (egui/eframe) for building one or more windows out of absolutely-positioned, z-orderable widgets - buttons, labels, checkboxes, textboxes, textareas, dropdowns, radio groups, sliders, number inputs, progress bars, separators, and images - with click/change/submit/key/close callbacks",
    functions: FUNCTIONS,
    since: Some("v0.4.0"),
    unstable: true,
};

// Widget functions: create a widget and return a handle to it.
// Control functions: get/set a widget's state.
// Event functions: attach callbacks.
// Lifecycle functions: open/close the native window and its event loop.
static FUNCTIONS: &[&FnEntry] = &[
    &gui_button::GUI_BUTTON,
        &gui_clipboard_copy::GUI_CLIPBOARD_COPY,
    &gui_clipboard_paste::GUI_CLIPBOARD_PASTE,
&gui_checkbox::GUI_CHECKBOX,
    &gui_canvas::GUI_CANVAS,
    &gui_draw_line::GUI_DRAW_LINE,
    &gui_draw_rect::GUI_DRAW_RECT,
    &gui_draw_circle::GUI_DRAW_CIRCLE,
    &gui_draw_text::GUI_DRAW_TEXT,
    &gui_close::GUI_CLOSE,
    &gui_dropdown::GUI_DROPDOWN,
    &gui_get_pos::GUI_GET_POS,
    &gui_get_progress::GUI_GET_PROGRESS,
    &gui_get_selected::GUI_GET_SELECTED,
    &gui_get_selected_index::GUI_GET_SELECTED_INDEX,
    &gui_get_text::GUI_GET_TEXT,
    &gui_get_value::GUI_GET_VALUE,
    &gui_get_window_pos::GUI_GET_WINDOW_POS,
    &gui_get_window_size::GUI_GET_WINDOW_SIZE,
    &gui_get_z::GUI_GET_Z,
        &gui_hyperlink::GUI_HYPERLINK,
&gui_image::GUI_IMAGE,
    &gui_update_image::GUI_UPDATE_IMAGE,
    &gui_set_align::GUI_SET_ALIGN,
    &gui_set_padding::GUI_SET_PADDING,
    &gui_set_spacing::GUI_SET_SPACING,
    &gui_detach::GUI_DETACH,
    &gui_add::GUI_ADD,
    &gui_hbox::GUI_HBOX,
    &gui_vbox::GUI_VBOX,
        &gui_is_selected::GUI_IS_SELECTED,
&gui_is_checked::GUI_IS_CHECKED,
    &gui_is_visible::GUI_IS_VISIBLE,
    &gui_label::GUI_LABEL,
    &gui_number_input::GUI_NUMBER_INPUT,
    &gui_on_change::GUI_ON_CHANGE,
    &gui_on_click::GUI_ON_CLICK,
    &gui_on_close::GUI_ON_CLOSE,
        &gui_on_file_drop::GUI_ON_FILE_DROP,
&gui_on_frame::GUI_ON_FRAME,
    &gui_on_key::GUI_ON_KEY,
    &gui_on_mouse_move::GUI_ON_MOUSE_MOVE,
        &gui_on_scroll::GUI_ON_SCROLL,
&gui_on_submit::GUI_ON_SUBMIT,
    &gui_progress_bar::GUI_PROGRESS_BAR,
    &gui_quit::GUI_QUIT,
    &gui_radio_group::GUI_RADIO_GROUP,
    &gui_remove::GUI_REMOVE,
    &gui_run::GUI_RUN,
        &gui_selectable::GUI_SELECTABLE,
&gui_separator::GUI_SEPARATOR,
    &gui_set_checked::GUI_SET_CHECKED,
    &gui_set_color::GUI_SET_COLOR,
    &gui_set_bg_color::GUI_SET_BG_COLOR,
    &gui_set_font_size::GUI_SET_FONT_SIZE,
    &gui_set_pos::GUI_SET_POS,
    &gui_set_progress::GUI_SET_PROGRESS,
        &gui_set_selected::GUI_SET_SELECTED,
&gui_set_selected_index::GUI_SET_SELECTED_INDEX,
    &gui_set_text::GUI_SET_TEXT,
    &gui_set_tooltip::GUI_SET_TOOLTIP,
    &gui_set_value::GUI_SET_VALUE,
    &gui_set_visible::GUI_SET_VISIBLE,
    &gui_set_z::GUI_SET_Z,
        &gui_spinner::GUI_SPINNER,
&gui_slider::GUI_SLIDER,
    &gui_textarea::GUI_TEXTAREA,
    &gui_textbox::GUI_TEXTBOX,
    &gui_window::GUI_WINDOW,
        &gui_window_fullscreen::GUI_WINDOW_FULLSCREEN,
    &gui_window_maximize::GUI_WINDOW_MAXIMIZE,
    &gui_window_minimize::GUI_WINDOW_MINIMIZE,
&gui_window_set_background::GUI_WINDOW_SET_BACKGROUND,
    &gui_window_set_decorated::GUI_WINDOW_SET_DECORATED,
    &gui_window_set_icon::GUI_WINDOW_SET_ICON,
    &gui_window_set_pos::GUI_WINDOW_SET_POS,
    &gui_window_set_size::GUI_WINDOW_SET_SIZE,
    &gui_window_set_title::GUI_WINDOW_SET_TITLE,
];
