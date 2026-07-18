//! Static name lists for every `std::*` stdlib module.

pub mod array {
    pub const KEYWORDS: &[&str] = &[
        "arr_push",
        "arr_pop",
        "arr_insert",
        "arr_remove",
        "arr_reverse",
        "arr_concat",
        "arr_first",
        "arr_last",
        "arr_max",
        "arr_min",
        "arr_sum",
        "arr_product",
        "arr_unique",
        "arr_is_empty",
        "arr_count",
        "arr_contains",
        "arr_index_of",
        "arr_sort",
        "arr_slice",
        "arr_flatten",
        "arr_range",
        "arr_fill",
        "arr_map",
        "len",
        "arr_filter",
        "arr_all",
        "arr_any",
        "arr_find",
        "arr_find_index",
        "arr_reduce",
        "arr_sort_by",
        "arr_flat_map",
        "arr_for_each",
        "arr_zip",
    ];
}

pub mod bitwise {
    pub const KEYWORDS: &[&str] = &[
        "bit_and",
        "bit_or",
        "bit_xor",
        "bit_not",
        "bit_shift_left",
        "bit_shift_right",
        "count_bits",
        "leading_zeros",
        "trailing_zeros",
    ];
}

pub mod debug {
    pub const KEYWORDS: &[&str] = &[
        "assert",
        "assert_eq",
        "assert_ne",
        "assert_lt",
        "assert_le",
        "assert_gt",
        "assert_ge",
        "assert_approx_eq",
        "panic",
        "unreachable",
        "todo",
        "dbg",
        "type_of",
        "bench",
    ];
}

pub mod fs {
    pub const KEYWORDS: &[&str] = &[
        "mkdir",
        "mkdir_all",
        "rmdir",
        "rmdir_all",
        "list_dir",
        "copy_file",
        "move_file",
        "file_size",
        "file_modified",
        "temp_dir",
        "rename_file",
    ];
}

pub mod http {
    pub const KEYWORDS: &[&str] = &[
        "http_server_start",
        "http_server_recv",
        "http_server_try_recv",
        "http_request_method",
        "http_request_url",
        "http_request_header",
        "http_request_body",
        "http_respond",
        "http_server_stop",
        "http_get",
        "http_post",
        "http_request",
    ];
}

pub mod io {
    pub const KEYWORDS: &[&str] = &[
        "read",
        "read_int",
        "read_float",
        "read_file",
        "read_lines",
        "delete_file",
        "write_file",
        "append_file",
        "print",
        "println",
        "eprint",
        "read_bytes",
    ];
}

pub mod net {
    pub const KEYWORDS: &[&str] = &[
        "tcp_listen",
        "tcp_accept",
        "tcp_connect",
        "tcp_read",
        "tcp_write",
        "tcp_peer_addr",
        "tcp_local_addr",
        "tcp_set_timeout",
        "tcp_set_nonblocking",
        "tcp_shutdown",
        "tcp_close",
        "udp_bind",
        "udp_connect",
        "udp_send",
        "udp_send_to",
        "udp_recv",
        "udp_recv_from",
        "udp_close",
        "resolve",
    ];
}

pub mod path {
    pub const KEYWORDS: &[&str] = &[
        "path_exists",
        "path_extension",
        "path_filename",
        "path_is_dir",
        "path_is_file",
        "path_join",
        "path_parent",
        "path_pop",
        "path_push",
        "path_set_extension",
        "path_stem",
    ];
}

pub mod process {
    pub const KEYWORDS: &[&str] = &[
        "args",
        "exit",
        "env",
        "cwd",
        "set_cwd",
        "pid",
        "sleep",
        "exec",
        "exec_code",
        "exec_lines",
    ];
}

pub mod random {
    pub const KEYWORDS: &[&str] = &[
        "rand_int",
        "rand_int_range",
        "rand_float",
        "rand_float_range",
        "rand_bool",
        "rand_bool_weighted",
        "rand_dice",
        "rand_dices",
        "rand_range",
        "rand_range_step",
        "rand_choice",
        "rand_choices",
        "rand_sample",
        "rand_shuffle",
        "rand_byte",
        "rand_bytes",
        "rand_char",
        "rand_string",
    ];
}

pub mod result {
    pub const KEYWORDS: &[&str] = &[
        "is_ok",
        "is_err",
        "result_unwrap",
        "result_unwrap_err",
        "result_unwrap_or",
        "result_map",
        "result_map_err",
    ];
}

pub mod rl {
    pub const KEYWORDS: &[&str] = &[
        "lex",
        "eval",
        "check",
        "eval_isolated",
        "rl_version",
        "source_name",
    ];
}

pub mod string {
    pub const KEYWORDS: &[&str] = &[
        "to_lower",
        "to_upper",
        "trim",
        "trim_end",
        "trim_start",
        "repeat",
        "is_empty",
        "concat",
        "char_at",
        "bytes",
        "chars",
        "slice",
        "contains",
        "starts_with",
        "ends_with",
        "replace",
        "pad_left",
        "pad_right",
        "split",
        "join",
        "count",
        "index_of",
        "format",
    ];
}

pub mod terminal {
    pub const KEYWORDS: &[&str] = &[
        "term_enter",
        "term_leave",
        "term_clear",
        "term_clear_line",
        "term_move",
        "term_move_up",
        "term_move_down",
        "term_move_left",
        "term_move_right",
        "term_move_to_col",
        "term_move_to_row",
        "term_next_line",
        "term_prev_line",
        "term_save_cursor",
        "term_restore_cursor",
        "term_hide_cursor",
        "term_show_cursor",
        "term_get_size",
        "term_set_size",
        "term_set_title",
        "term_scroll_up",
        "term_scroll_down",
        "term_print",
        "term_flush",
        "term_set_fg",
        "term_set_bg",
        "term_reset_color",
        "term_fg",
        "term_bg",
        "term_bold",
        "term_dim",
        "term_italic",
        "term_underline",
        "term_blink",
        "term_reverse",
        "term_crossed_out",
        "term_reset_attr",
        "term_enable_wrap",
        "term_disable_wrap",
        "term_begin_sync",
        "term_end_sync",
        "term_enable_mouse",
        "term_disable_mouse",
        "term_read_key",
        "term_poll",
    ];
}

pub mod time {
    pub const KEYWORDS: &[&str] = &[
        "time_now",
        "time_now_ms",
        "format_time",
        "format_date_str",
        "format_time_str",
        "time_add",
        "time_diff",
        "time_parts",
    ];
}

pub mod types {
    pub const KEYWORDS: &[&str] = &[
        "to_bin",
        "to_bool",
        "to_char",
        "to_float",
        "to_hex",
        "to_int",
        "to_oct",
        "to_string",
        "is_bool",
        "is_null",
        "is_char",
        "is_int",
        "is_string",
        "is_float",
        "is_error",
        "error_unwrap",
        "to_byte",
        "is_byte",
    ];
}

pub mod math {
    pub const KEYWORDS: &[&str] = &[
        "sin",
        "cos",
        "tan",
        "pow",
        "mod",
        "abs",
        "ceil",
        "clamp",
        "floor",
        "round",
        "log",
        "log2",
        "log10",
        "max",
        "min",
        "sqrt",
        "atan",
        "acos",
        "asin",
        "atan2",
        "radians",
        "degrees",
        "exp",
        "factorial",
        "fibonacci",
        "gcd",
        "lcm",
        "hypot",
        "lerp",
        "map_range",
        "sign",
        "is_prime",
    ];

    pub mod constants {
        pub const KEYWORDS: &[&str] = &[
            "E",
            "PI",
            "FRAC_1_PI",
            "FRAC_1_SQRT_2",
            "FRAC_2_PI",
            "FRAC_2_SQRT_PI",
            "FRAC_PI_2",
            "FRAC_PI_3",
            "FRAC_PI_4",
            "FRAC_PI_6",
            "FRAC_PI_8",
            "INF",
            "NAN",
            "is_inf",
            "is_nan",
            "LN_10",
            "LN_2",
            "LOG10_2",
            "LOG10_E",
            "LOG2_10",
            "LOG2_E",
            "SQRT_2",
            "EULER_GAMMA",
            "PHI",
            "TAU",
        ];
    }
}

pub mod set {
    pub const KEYWORDS: &[&str] = &[
        "set_add",
        "set_contains",
        "set_is_empty",
        "set_len",
        "set_remove",
        "set_to_array"
    ];
}
