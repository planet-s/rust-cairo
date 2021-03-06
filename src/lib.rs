#![crate_name = "rust_cairo"]
#![crate_type = "lib"]
#![allow(non_camel_case_types)]

pub const CAIRO_TAG_DEST: &'static [u8; 11usize] = b"cairo.dest\0";
pub const CAIRO_TAG_LINK: &'static [u8; 5usize] = b"Link\0";
pub const CAIRO_MIME_TYPE_JPEG: &'static [u8; 11usize] = b"image/jpeg\0";
pub const CAIRO_MIME_TYPE_PNG: &'static [u8; 10usize] = b"image/png\0";
pub const CAIRO_MIME_TYPE_JP2: &'static [u8; 10usize] = b"image/jp2\0";
pub const CAIRO_MIME_TYPE_URI: &'static [u8; 11usize] = b"text/x-uri\0";
pub const CAIRO_MIME_TYPE_UNIQUE_ID: &'static [u8; 25usize] = b"application/x-cairo.uuid\0";
pub const CAIRO_MIME_TYPE_JBIG2: &'static [u8; 26usize] = b"application/x-cairo.jbig2\0";
pub const CAIRO_MIME_TYPE_JBIG2_GLOBAL: &'static [u8; 33usize] = b"application/x-cairo.jbig2-global\0";
pub const CAIRO_MIME_TYPE_JBIG2_GLOBAL_ID: &'static [u8; 36usize] = b"application/x-cairo.jbig2-global-id\0";
pub const CAIRO_MIME_TYPE_CCITT_FAX: &'static [u8; 12usize] = b"image/g3fax\0";
pub const CAIRO_MIME_TYPE_CCITT_FAX_PARAMS: &'static [u8; 33usize] = b"application/x-cairo.ccitt.params\0";
pub const CAIRO_MIME_TYPE_EPS: &'static [u8; 23usize] = b"application/postscript\0";
pub const CAIRO_MIME_TYPE_EPS_PARAMS: &'static [u8; 31usize] = b"application/x-cairo.eps.params\0";

pub type cairo_bool_t = ::std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo {
    _unused: [u8; 0],
}

pub type cairo_t = _cairo;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_surface {
    _unused: [u8; 0],
}

pub type cairo_surface_t = _cairo_surface;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_device {
    _unused: [u8; 0],
}

pub type cairo_device_t = _cairo_device;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_matrix {
    pub xx: f64,
    pub yx: f64,
    pub xy: f64,
    pub yy: f64,
    pub x0: f64,
    pub y0: f64,
}

pub type cairo_matrix_t = _cairo_matrix;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_pattern {
    _unused: [u8; 0],
}

pub type cairo_pattern_t = _cairo_pattern;

pub type cairo_destroy_func_t = ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_user_data_key {
    pub unused: ::std::os::raw::c_int,
}


pub type cairo_user_data_key_t = _cairo_user_data_key;

pub const CAIRO_STATUS_SUCCESS: _cairo_status = 0;
pub const CAIRO_STATUS_NO_MEMORY: _cairo_status = 1;
pub const CAIRO_STATUS_INVALID_RESTORE: _cairo_status = 2;
pub const CAIRO_STATUS_INVALID_POP_GROUP: _cairo_status = 3;
pub const CAIRO_STATUS_NO_CURRENT_POINT: _cairo_status = 4;
pub const CAIRO_STATUS_INVALID_MATRIX: _cairo_status = 5;
pub const CAIRO_STATUS_INVALID_STATUS: _cairo_status = 6;
pub const CAIRO_STATUS_NULL_POINTER: _cairo_status = 7;
pub const CAIRO_STATUS_INVALID_STRING: _cairo_status = 8;
pub const CAIRO_STATUS_INVALID_PATH_DATA: _cairo_status = 9;
pub const CAIRO_STATUS_READ_ERROR: _cairo_status = 10;
pub const CAIRO_STATUS_WRITE_ERROR: _cairo_status = 11;
pub const CAIRO_STATUS_SURFACE_FINISHED: _cairo_status = 12;
pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: _cairo_status = 13;
pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: _cairo_status = 14;
pub const CAIRO_STATUS_INVALID_CONTENT: _cairo_status = 15;
pub const CAIRO_STATUS_INVALID_FORMAT: _cairo_status = 16;
pub const CAIRO_STATUS_INVALID_VISUAL: _cairo_status = 17;
pub const CAIRO_STATUS_FILE_NOT_FOUND: _cairo_status = 18;
pub const CAIRO_STATUS_INVALID_DASH: _cairo_status = 19;
pub const CAIRO_STATUS_INVALID_DSC_COMMENT: _cairo_status = 20;
pub const CAIRO_STATUS_INVALID_INDEX: _cairo_status = 21;
pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_status = 22;
pub const CAIRO_STATUS_TEMP_FILE_ERROR: _cairo_status = 23;
pub const CAIRO_STATUS_INVALID_STRIDE: _cairo_status = 24;
pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: _cairo_status = 25;
pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: _cairo_status = 26;
pub const CAIRO_STATUS_USER_FONT_ERROR: _cairo_status = 27;
pub const CAIRO_STATUS_NEGATIVE_COUNT: _cairo_status = 28;
pub const CAIRO_STATUS_INVALID_CLUSTERS: _cairo_status = 29;
pub const CAIRO_STATUS_INVALID_SLANT: _cairo_status = 30;
pub const CAIRO_STATUS_INVALID_WEIGHT: _cairo_status = 31;
pub const CAIRO_STATUS_INVALID_SIZE: _cairo_status = 32;
pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_status = 33;
pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: _cairo_status = 34;
pub const CAIRO_STATUS_DEVICE_ERROR: _cairo_status = 35;
pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_status = 36;
pub const CAIRO_STATUS_DEVICE_FINISHED: _cairo_status = 37;
pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: _cairo_status = 38;
pub const CAIRO_STATUS_PNG_ERROR: _cairo_status = 39;
pub const CAIRO_STATUS_FREETYPE_ERROR: _cairo_status = 40;
pub const CAIRO_STATUS_WIN32_GDI_ERROR: _cairo_status = 41;
pub const CAIRO_STATUS_TAG_ERROR: _cairo_status = 42;
pub const CAIRO_STATUS_LAST_STATUS: _cairo_status = 43;

pub type _cairo_status = u32;

pub use self::_cairo_status as cairo_status_t;

pub const CAIRO_CONTENT_COLOR: _cairo_content = 4096;
pub const CAIRO_CONTENT_ALPHA: _cairo_content = 8192;
pub const CAIRO_CONTENT_COLOR_ALPHA: _cairo_content = 12288;

pub type _cairo_content = u32;

pub use self::_cairo_content as cairo_content_t;

pub const CAIRO_FORMAT_INVALID: _cairo_format = -1;
pub const CAIRO_FORMAT_ARGB32: _cairo_format = 0;
pub const CAIRO_FORMAT_RGB24: _cairo_format = 1;
pub const CAIRO_FORMAT_A8: _cairo_format = 2;
pub const CAIRO_FORMAT_A1: _cairo_format = 3;
pub const CAIRO_FORMAT_RGB16_565: _cairo_format = 4;
pub const CAIRO_FORMAT_RGB30: _cairo_format = 5;

pub type _cairo_format = i32;

pub use self::_cairo_format as cairo_format_t;

pub type cairo_write_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        closure: *mut ::std::os::raw::c_void,
        data: *const ::std::os::raw::c_uchar,
        length: ::std::os::raw::c_uint,
    ) -> cairo_status_t,
>;

pub type cairo_read_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        closure: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_uchar,
        length: ::std::os::raw::c_uint,
    ) -> cairo_status_t,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_rectangle_int {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}

pub type cairo_rectangle_int_t = _cairo_rectangle_int;

pub const CAIRO_OPERATOR_CLEAR: _cairo_operator = 0;
pub const CAIRO_OPERATOR_SOURCE: _cairo_operator = 1;
pub const CAIRO_OPERATOR_OVER: _cairo_operator = 2;
pub const CAIRO_OPERATOR_IN: _cairo_operator = 3;
pub const CAIRO_OPERATOR_OUT: _cairo_operator = 4;
pub const CAIRO_OPERATOR_ATOP: _cairo_operator = 5;
pub const CAIRO_OPERATOR_DEST: _cairo_operator = 6;
pub const CAIRO_OPERATOR_DEST_OVER: _cairo_operator = 7;
pub const CAIRO_OPERATOR_DEST_IN: _cairo_operator = 8;
pub const CAIRO_OPERATOR_DEST_OUT: _cairo_operator = 9;
pub const CAIRO_OPERATOR_DEST_ATOP: _cairo_operator = 10;
pub const CAIRO_OPERATOR_XOR: _cairo_operator = 11;
pub const CAIRO_OPERATOR_ADD: _cairo_operator = 12;
pub const CAIRO_OPERATOR_SATURATE: _cairo_operator = 13;
pub const CAIRO_OPERATOR_MULTIPLY: _cairo_operator = 14;
pub const CAIRO_OPERATOR_SCREEN: _cairo_operator = 15;
pub const CAIRO_OPERATOR_OVERLAY: _cairo_operator = 16;
pub const CAIRO_OPERATOR_DARKEN: _cairo_operator = 17;
pub const CAIRO_OPERATOR_LIGHTEN: _cairo_operator = 18;
pub const CAIRO_OPERATOR_COLOR_DODGE: _cairo_operator = 19;
pub const CAIRO_OPERATOR_COLOR_BURN: _cairo_operator = 20;
pub const CAIRO_OPERATOR_HARD_LIGHT: _cairo_operator = 21;
pub const CAIRO_OPERATOR_SOFT_LIGHT: _cairo_operator = 22;
pub const CAIRO_OPERATOR_DIFFERENCE: _cairo_operator = 23;
pub const CAIRO_OPERATOR_EXCLUSION: _cairo_operator = 24;
pub const CAIRO_OPERATOR_HSL_HUE: _cairo_operator = 25;
pub const CAIRO_OPERATOR_HSL_SATURATION: _cairo_operator = 26;
pub const CAIRO_OPERATOR_HSL_COLOR: _cairo_operator = 27;
pub const CAIRO_OPERATOR_HSL_LUMINOSITY: _cairo_operator = 28;

pub type _cairo_operator = u32;

pub use self::_cairo_operator as cairo_operator_t;

pub const CAIRO_ANTIALIAS_DEFAULT: _cairo_antialias = 0;
pub const CAIRO_ANTIALIAS_NONE: _cairo_antialias = 1;
pub const CAIRO_ANTIALIAS_GRAY: _cairo_antialias = 2;
pub const CAIRO_ANTIALIAS_SUBPIXEL: _cairo_antialias = 3;
pub const CAIRO_ANTIALIAS_FAST: _cairo_antialias = 4;
pub const CAIRO_ANTIALIAS_GOOD: _cairo_antialias = 5;
pub const CAIRO_ANTIALIAS_BEST: _cairo_antialias = 6;

pub type _cairo_antialias = u32;

pub use self::_cairo_antialias as cairo_antialias_t;

pub const CAIRO_FILL_RULE_WINDING: _cairo_fill_rule = 0;
pub const CAIRO_FILL_RULE_EVEN_ODD: _cairo_fill_rule = 1;

pub type _cairo_fill_rule = u32;

pub use self::_cairo_fill_rule as cairo_fill_rule_t;


pub const CAIRO_LINE_CAP_BUTT: _cairo_line_cap = 0;
pub const CAIRO_LINE_CAP_ROUND: _cairo_line_cap = 1;
pub const CAIRO_LINE_CAP_SQUARE: _cairo_line_cap = 2;

pub type _cairo_line_cap = u32;

pub use self::_cairo_line_cap as cairo_line_cap_t;

pub const CAIRO_LINE_JOIN_MITER: _cairo_line_join = 0;
pub const CAIRO_LINE_JOIN_ROUND: _cairo_line_join = 1;
pub const CAIRO_LINE_JOIN_BEVEL: _cairo_line_join = 2;

pub type _cairo_line_join = u32;

pub use self::_cairo_line_join as cairo_line_join_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub type cairo_rectangle_t = _cairo_rectangle;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_rectangle_list {
    pub status: cairo_status_t,
    pub rectangles: *mut cairo_rectangle_t,
    pub num_rectangles: ::std::os::raw::c_int,
}

pub type cairo_rectangle_list_t = _cairo_rectangle_list;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_scaled_font {
    _unused: [u8; 0],
}

pub type cairo_scaled_font_t = _cairo_scaled_font;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_font_face {
    _unused: [u8; 0],
}

pub type cairo_font_face_t = _cairo_font_face;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_glyph_t {
    pub index: ::std::os::raw::c_ulong,
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_text_cluster_t {
    pub num_bytes: ::std::os::raw::c_int,
    pub num_glyphs: ::std::os::raw::c_int,
}

pub const CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: _cairo_text_cluster_flags = 1;

pub type _cairo_text_cluster_flags = u32;

pub use self::_cairo_text_cluster_flags as cairo_text_cluster_flags_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_text_extents_t {
    pub x_bearing: f64,
    pub y_bearing: f64,
    pub width: f64,
    pub height: f64,
    pub x_advance: f64,
    pub y_advance: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_font_extents_t {
    pub ascent: f64,
    pub descent: f64,
    pub height: f64,
    pub max_x_advance: f64,
    pub max_y_advance: f64,
}

pub const CAIRO_FONT_SLANT_NORMAL: _cairo_font_slant = 0;
pub const CAIRO_FONT_SLANT_ITALIC: _cairo_font_slant = 1;
pub const CAIRO_FONT_SLANT_OBLIQUE: _cairo_font_slant = 2;

pub type _cairo_font_slant = u32;

pub use self::_cairo_font_slant as cairo_font_slant_t;

pub const CAIRO_FONT_WEIGHT_NORMAL: _cairo_font_weight = 0;
pub const CAIRO_FONT_WEIGHT_BOLD: _cairo_font_weight = 1;

pub type _cairo_font_weight = u32;

pub use self::_cairo_font_weight as cairo_font_weight_t;

pub const CAIRO_SUBPIXEL_ORDER_DEFAULT: _cairo_subpixel_order = 0;
pub const CAIRO_SUBPIXEL_ORDER_RGB: _cairo_subpixel_order = 1;
pub const CAIRO_SUBPIXEL_ORDER_BGR: _cairo_subpixel_order = 2;
pub const CAIRO_SUBPIXEL_ORDER_VRGB: _cairo_subpixel_order = 3;
pub const CAIRO_SUBPIXEL_ORDER_VBGR: _cairo_subpixel_order = 4;

pub type _cairo_subpixel_order = u32;

pub use self::_cairo_subpixel_order as cairo_subpixel_order_t;

pub const CAIRO_HINT_STYLE_DEFAULT: _cairo_hint_style = 0;
pub const CAIRO_HINT_STYLE_NONE: _cairo_hint_style = 1;
pub const CAIRO_HINT_STYLE_SLIGHT: _cairo_hint_style = 2;
pub const CAIRO_HINT_STYLE_MEDIUM: _cairo_hint_style = 3;
pub const CAIRO_HINT_STYLE_FULL: _cairo_hint_style = 4;

pub type _cairo_hint_style = u32;

pub use self::_cairo_hint_style as cairo_hint_style_t;

pub const CAIRO_HINT_METRICS_DEFAULT: _cairo_hint_metrics = 0;
pub const CAIRO_HINT_METRICS_OFF: _cairo_hint_metrics = 1;
pub const CAIRO_HINT_METRICS_ON: _cairo_hint_metrics = 2;

pub type _cairo_hint_metrics = u32;

pub use self::_cairo_hint_metrics as cairo_hint_metrics_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_font_options {
    _unused: [u8; 0],
}

pub type cairo_font_options_t = _cairo_font_options;

pub const CAIRO_FONT_TYPE_TOY: _cairo_font_type = 0;
pub const CAIRO_FONT_TYPE_FT: _cairo_font_type = 1;
pub const CAIRO_FONT_TYPE_WIN32: _cairo_font_type = 2;
pub const CAIRO_FONT_TYPE_QUARTZ: _cairo_font_type = 3;
pub const CAIRO_FONT_TYPE_USER: _cairo_font_type = 4;

pub type _cairo_font_type = u32;

pub use self::_cairo_font_type as cairo_font_type_t;

pub type cairo_user_scaled_font_init_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        scaled_font: *mut cairo_scaled_font_t,
        cr: *mut cairo_t,
        extents: *mut cairo_font_extents_t,
    ) -> cairo_status_t,
>;

pub type cairo_user_scaled_font_render_glyph_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        scaled_font: *mut cairo_scaled_font_t,
        glyph: ::std::os::raw::c_ulong,
        cr: *mut cairo_t,
        extents: *mut cairo_text_extents_t,
    ) -> cairo_status_t,
>;

pub type cairo_user_scaled_font_text_to_glyphs_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        scaled_font: *mut cairo_scaled_font_t,
        utf8: *const ::std::os::raw::c_char,
        utf8_len: ::std::os::raw::c_int,
        glyphs: *mut *mut cairo_glyph_t,
        num_glyphs: *mut ::std::os::raw::c_int,
        clusters: *mut *mut cairo_text_cluster_t,
        num_clusters: *mut ::std::os::raw::c_int,
        cluster_flags: *mut cairo_text_cluster_flags_t,
    ) -> cairo_status_t,
>;

pub type cairo_user_scaled_font_unicode_to_glyph_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        scaled_font: *mut cairo_scaled_font_t,
        unicode: ::std::os::raw::c_ulong,
        glyph_index: *mut ::std::os::raw::c_ulong,
    ) -> cairo_status_t,
>;

pub const CAIRO_PATH_MOVE_TO: _cairo_path_data_type = 0;
pub const CAIRO_PATH_LINE_TO: _cairo_path_data_type = 1;
pub const CAIRO_PATH_CURVE_TO: _cairo_path_data_type = 2;
pub const CAIRO_PATH_CLOSE_PATH: _cairo_path_data_type = 3;

pub type _cairo_path_data_type = u32;

pub use self::_cairo_path_data_type as cairo_path_data_type_t;

pub type cairo_path_data_t = _cairo_path_data_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub union _cairo_path_data_t {
    pub header: _cairo_path_data_t__bindgen_ty_1,
    pub point: _cairo_path_data_t__bindgen_ty_2,
    _bindgen_union_align: [u64; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_path_data_t__bindgen_ty_1 {
    pub type_: cairo_path_data_type_t,
    pub length: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_path_data_t__bindgen_ty_2 {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_path {
    pub status: cairo_status_t,
    pub data: *mut cairo_path_data_t,
    pub num_data: ::std::os::raw::c_int,
}

pub type cairo_path_t = cairo_path;

pub const CAIRO_DEVICE_TYPE_DRM: _cairo_device_type = 0;
pub const CAIRO_DEVICE_TYPE_GL: _cairo_device_type = 1;
pub const CAIRO_DEVICE_TYPE_SCRIPT: _cairo_device_type = 2;
pub const CAIRO_DEVICE_TYPE_XCB: _cairo_device_type = 3;
pub const CAIRO_DEVICE_TYPE_XLIB: _cairo_device_type = 4;
pub const CAIRO_DEVICE_TYPE_XML: _cairo_device_type = 5;
pub const CAIRO_DEVICE_TYPE_COGL: _cairo_device_type = 6;
pub const CAIRO_DEVICE_TYPE_WIN32: _cairo_device_type = 7;
pub const CAIRO_DEVICE_TYPE_INVALID: _cairo_device_type = -1;

pub type _cairo_device_type = i32;

pub use self::_cairo_device_type as cairo_device_type_t;

pub const CAIRO_SURFACE_OBSERVER_NORMAL:
cairo_surface_observer_mode_t = 0;
pub const CAIRO_SURFACE_OBSERVER_RECORD_OPERATIONS:
cairo_surface_observer_mode_t = 1;

pub type cairo_surface_observer_mode_t = u32;

pub type cairo_surface_observer_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        observer: *mut cairo_surface_t,
        target: *mut cairo_surface_t,
        data: *mut ::std::os::raw::c_void,
    ),
>;

pub const CAIRO_SURFACE_TYPE_IMAGE: _cairo_surface_type = 0;
pub const CAIRO_SURFACE_TYPE_PDF: _cairo_surface_type = 1;
pub const CAIRO_SURFACE_TYPE_PS: _cairo_surface_type = 2;
pub const CAIRO_SURFACE_TYPE_XLIB: _cairo_surface_type = 3;
pub const CAIRO_SURFACE_TYPE_XCB: _cairo_surface_type = 4;
pub const CAIRO_SURFACE_TYPE_GLITZ: _cairo_surface_type = 5;
pub const CAIRO_SURFACE_TYPE_QUARTZ: _cairo_surface_type = 6;
pub const CAIRO_SURFACE_TYPE_WIN32: _cairo_surface_type = 7;
pub const CAIRO_SURFACE_TYPE_BEOS: _cairo_surface_type = 8;
pub const CAIRO_SURFACE_TYPE_DIRECTFB: _cairo_surface_type = 9;
pub const CAIRO_SURFACE_TYPE_SVG: _cairo_surface_type = 10;
pub const CAIRO_SURFACE_TYPE_OS2: _cairo_surface_type = 11;
pub const CAIRO_SURFACE_TYPE_WIN32_PRINTING: _cairo_surface_type = 12;
pub const CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: _cairo_surface_type = 13;
pub const CAIRO_SURFACE_TYPE_SCRIPT: _cairo_surface_type = 14;
pub const CAIRO_SURFACE_TYPE_QT: _cairo_surface_type = 15;
pub const CAIRO_SURFACE_TYPE_RECORDING: _cairo_surface_type = 16;
pub const CAIRO_SURFACE_TYPE_VG: _cairo_surface_type = 17;
pub const CAIRO_SURFACE_TYPE_GL: _cairo_surface_type = 18;
pub const CAIRO_SURFACE_TYPE_DRM: _cairo_surface_type = 19;
pub const CAIRO_SURFACE_TYPE_TEE: _cairo_surface_type = 20;
pub const CAIRO_SURFACE_TYPE_XML: _cairo_surface_type = 21;
pub const CAIRO_SURFACE_TYPE_SKIA: _cairo_surface_type = 22;
pub const CAIRO_SURFACE_TYPE_SUBSURFACE: _cairo_surface_type = 23;
pub const CAIRO_SURFACE_TYPE_COGL: _cairo_surface_type = 24;

pub type _cairo_surface_type = u32;

pub use self::_cairo_surface_type as cairo_surface_type_t;

pub type cairo_raster_source_acquire_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        pattern: *mut cairo_pattern_t,
        callback_data: *mut ::std::os::raw::c_void,
        target: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t,
>;

pub type cairo_raster_source_release_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        pattern: *mut cairo_pattern_t,
        callback_data: *mut ::std::os::raw::c_void,
        surface: *mut cairo_surface_t,
    ),
>;

pub type cairo_raster_source_snapshot_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        pattern: *mut cairo_pattern_t,
        callback_data: *mut ::std::os::raw::c_void,
    ) -> cairo_status_t,
>;

pub type cairo_raster_source_copy_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        pattern: *mut cairo_pattern_t,
        callback_data: *mut ::std::os::raw::c_void,
        other: *const cairo_pattern_t,
    ) -> cairo_status_t,
>;

pub type cairo_raster_source_finish_func_t = ::std::option::Option<
    unsafe extern "C" fn(pattern: *mut cairo_pattern_t, callback_data: *mut ::std::os::raw::c_void),
>;

pub const CAIRO_PATTERN_TYPE_SOLID: _cairo_pattern_type = 0;
pub const CAIRO_PATTERN_TYPE_SURFACE: _cairo_pattern_type = 1;
pub const CAIRO_PATTERN_TYPE_LINEAR: _cairo_pattern_type = 2;
pub const CAIRO_PATTERN_TYPE_RADIAL: _cairo_pattern_type = 3;
pub const CAIRO_PATTERN_TYPE_MESH: _cairo_pattern_type = 4;
pub const CAIRO_PATTERN_TYPE_RASTER_SOURCE: _cairo_pattern_type = 5;

pub type _cairo_pattern_type = u32;

pub use self::_cairo_pattern_type as cairo_pattern_type_t;

pub const CAIRO_EXTEND_NONE: _cairo_extend = 0;
pub const CAIRO_EXTEND_REPEAT: _cairo_extend = 1;
pub const CAIRO_EXTEND_REFLECT: _cairo_extend = 2;
pub const CAIRO_EXTEND_PAD: _cairo_extend = 3;

pub type _cairo_extend = u32;

pub use self::_cairo_extend as cairo_extend_t;

pub const CAIRO_FILTER_FAST: _cairo_filter = 0;
pub const CAIRO_FILTER_GOOD: _cairo_filter = 1;
pub const CAIRO_FILTER_BEST: _cairo_filter = 2;
pub const CAIRO_FILTER_NEAREST: _cairo_filter = 3;
pub const CAIRO_FILTER_BILINEAR: _cairo_filter = 4;
pub const CAIRO_FILTER_GAUSSIAN: _cairo_filter = 5;

pub type _cairo_filter = u32;

pub use self::_cairo_filter as cairo_filter_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_region {
    _unused: [u8; 0],
}

pub type cairo_region_t = _cairo_region;

pub const CAIRO_REGION_OVERLAP_IN: _cairo_region_overlap = 0;
pub const CAIRO_REGION_OVERLAP_OUT: _cairo_region_overlap = 1;
pub const CAIRO_REGION_OVERLAP_PART: _cairo_region_overlap = 2;

pub type _cairo_region_overlap = u32;

pub use self::_cairo_region_overlap as cairo_region_overlap_t;

#[link(name = "cairo")]
#[link(name = "pixman-1")]
#[link(name = "freetype")]
#[link(name = "png")]
#[link(name = "z")]
extern "C" {
    pub fn cairo_version() -> ::std::os::raw::c_int;
    pub fn cairo_version_string() -> *const ::std::os::raw::c_char;
    pub fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    pub fn cairo_reference(cr: *mut cairo_t) -> *mut cairo_t;
    pub fn cairo_destroy(cr: *mut cairo_t);
    pub fn cairo_get_reference_count(cr: *mut cairo_t) -> ::std::os::raw::c_uint;
    pub fn cairo_get_user_data(cr: *mut cairo_t, key: *const cairo_user_data_key_t) -> *mut ::std::os::raw::c_void;
    pub fn cairo_set_user_data(cr: *mut cairo_t, key: *const cairo_user_data_key_t, user_data: *mut ::std::os::raw::c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    pub fn cairo_save(cr: *mut cairo_t);
    pub fn cairo_restore(cr: *mut cairo_t);
    pub fn cairo_push_group(cr: *mut cairo_t);
    pub fn cairo_push_group_with_content(cr: *mut cairo_t, content: cairo_content_t);
    pub fn cairo_pop_group(cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_pop_group_to_source(cr: *mut cairo_t);
    pub fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
    pub fn cairo_set_source(cr: *mut cairo_t, source: *mut cairo_pattern_t);
    pub fn cairo_set_source_rgb(cr: *mut cairo_t, red: f64, green: f64, blue: f64);
    pub fn cairo_set_source_rgba(cr: *mut cairo_t, red: f64, green: f64, blue: f64, alpha: f64);
    pub fn cairo_set_source_surface(cr: *mut cairo_t, surface: *mut cairo_surface_t, x: f64, y: f64);
    pub fn cairo_set_tolerance(cr: *mut cairo_t, tolerance: f64);
    pub fn cairo_set_antialias(cr: *mut cairo_t, antialias: cairo_antialias_t);
    pub fn cairo_set_fill_rule(cr: *mut cairo_t, fill_rule: cairo_fill_rule_t);
    pub fn cairo_set_line_width(cr: *mut cairo_t, width: f64);
    pub fn cairo_set_line_cap(cr: *mut cairo_t, line_cap: cairo_line_cap_t);
    pub fn cairo_set_line_join(cr: *mut cairo_t, line_join: cairo_line_join_t);
    pub fn cairo_set_dash(cr: *mut cairo_t, dashes: *const f64, num_dashes: ::std::os::raw::c_int, offset: f64);
    pub fn cairo_set_miter_limit(cr: *mut cairo_t, limit: f64);
    pub fn cairo_translate(cr: *mut cairo_t, tx: f64, ty: f64);
    pub fn cairo_scale(cr: *mut cairo_t, sx: f64, sy: f64);
    pub fn cairo_rotate(cr: *mut cairo_t, angle: f64);
    pub fn cairo_transform(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    pub fn cairo_set_matrix(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    pub fn cairo_identity_matrix(cr: *mut cairo_t);
    pub fn cairo_user_to_device(cr: *mut cairo_t, x: *mut f64, y: *mut f64);
    pub fn cairo_user_to_device_distance(cr: *mut cairo_t, dx: *mut f64, dy: *mut f64);
    pub fn cairo_device_to_user(cr: *mut cairo_t, x: *mut f64, y: *mut f64);
    pub fn cairo_device_to_user_distance(cr: *mut cairo_t, dx: *mut f64, dy: *mut f64);
    pub fn cairo_new_path(cr: *mut cairo_t);
    pub fn cairo_move_to(cr: *mut cairo_t, x: f64, y: f64);
    pub fn cairo_new_sub_path(cr: *mut cairo_t);
    pub fn cairo_line_to(cr: *mut cairo_t, x: f64, y: f64);
    pub fn cairo_curve_to(cr: *mut cairo_t, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
    pub fn cairo_arc(cr: *mut cairo_t, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
    pub fn cairo_arc_negative(cr: *mut cairo_t, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
    pub fn cairo_rel_move_to(cr: *mut cairo_t, dx: f64, dy: f64);
    pub fn cairo_rel_line_to(cr: *mut cairo_t, dx: f64, dy: f64);
    pub fn cairo_rel_curve_to(cr: *mut cairo_t, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64);
    pub fn cairo_rectangle(cr: *mut cairo_t, x: f64, y: f64, width: f64, height: f64);
    pub fn cairo_close_path(cr: *mut cairo_t);
    pub fn cairo_path_extents(cr: *mut cairo_t, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
    pub fn cairo_paint(cr: *mut cairo_t);
    pub fn cairo_paint_with_alpha(cr: *mut cairo_t, alpha: f64);
    pub fn cairo_mask(cr: *mut cairo_t, pattern: *mut cairo_pattern_t);
    pub fn cairo_mask_surface(cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: f64, surface_y: f64);
    pub fn cairo_stroke(cr: *mut cairo_t);
    pub fn cairo_stroke_preserve(cr: *mut cairo_t);
    pub fn cairo_fill(cr: *mut cairo_t);
    pub fn cairo_fill_preserve(cr: *mut cairo_t);
    pub fn cairo_copy_page(cr: *mut cairo_t);
    pub fn cairo_show_page(cr: *mut cairo_t);
    pub fn cairo_in_stroke(cr: *mut cairo_t, x: f64, y: f64) -> cairo_bool_t;
    pub fn cairo_in_fill(cr: *mut cairo_t, x: f64, y: f64) -> cairo_bool_t;
    pub fn cairo_in_clip(cr: *mut cairo_t, x: f64, y: f64) -> cairo_bool_t;
    pub fn cairo_stroke_extents(cr: *mut cairo_t, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
    pub fn cairo_fill_extents(cr: *mut cairo_t, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
    pub fn cairo_reset_clip(cr: *mut cairo_t);
    pub fn cairo_clip(cr: *mut cairo_t);
    pub fn cairo_clip_preserve(cr: *mut cairo_t);
    pub fn cairo_clip_extents(cr: *mut cairo_t, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
    pub fn cairo_copy_clip_rectangle_list(cr: *mut cairo_t) -> *mut cairo_rectangle_list_t;
    pub fn cairo_rectangle_list_destroy(rectangle_list: *mut cairo_rectangle_list_t);
    pub fn cairo_tag_begin(cr: *mut cairo_t, tag_name: *const ::std::os::raw::c_char, attributes: *const ::std::os::raw::c_char);
    pub fn cairo_tag_end(cr: *mut cairo_t, tag_name: *const ::std::os::raw::c_char);
    pub fn cairo_glyph_allocate(num_glyphs: ::std::os::raw::c_int) -> *mut cairo_glyph_t;
    pub fn cairo_glyph_free(glyphs: *mut cairo_glyph_t);
    pub fn cairo_text_cluster_allocate(num_clusters: ::std::os::raw::c_int) -> *mut cairo_text_cluster_t;
    pub fn cairo_text_cluster_free(clusters: *mut cairo_text_cluster_t);
    pub fn cairo_font_options_create() -> *mut cairo_font_options_t;
    pub fn cairo_font_options_copy(original: *const cairo_font_options_t) -> *mut cairo_font_options_t;
    pub fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    pub fn cairo_font_options_status(options: *mut cairo_font_options_t) -> cairo_status_t;
    pub fn cairo_font_options_merge(options: *mut cairo_font_options_t, other: *const cairo_font_options_t);
    pub fn cairo_font_options_equal(options: *const cairo_font_options_t, other: *const cairo_font_options_t) -> cairo_bool_t;
    pub fn cairo_font_options_hash(options: *const cairo_font_options_t) -> ::std::os::raw::c_ulong;
    pub fn cairo_font_options_set_antialias(options: *mut cairo_font_options_t, antialias: cairo_antialias_t);
    pub fn cairo_font_options_get_antialias(options: *const cairo_font_options_t) -> cairo_antialias_t;
    pub fn cairo_font_options_set_subpixel_order(options: *mut cairo_font_options_t, subpixel_order: cairo_subpixel_order_t);
    pub fn cairo_font_options_get_subpixel_order(options: *const cairo_font_options_t) -> cairo_subpixel_order_t;
    pub fn cairo_font_options_set_hint_style(options: *mut cairo_font_options_t, hint_style: cairo_hint_style_t);
    pub fn cairo_font_options_get_hint_style(options: *const cairo_font_options_t) -> cairo_hint_style_t;
    pub fn cairo_font_options_set_hint_metrics(options: *mut cairo_font_options_t, hint_metrics: cairo_hint_metrics_t);
    pub fn cairo_font_options_get_hint_metrics(options: *const cairo_font_options_t) -> cairo_hint_metrics_t;
    pub fn cairo_font_options_get_variations(options: *mut cairo_font_options_t) -> *const ::std::os::raw::c_char;
    pub fn cairo_font_options_set_variations(options: *mut cairo_font_options_t, variations: *const ::std::os::raw::c_char);
    pub fn cairo_select_font_face(cr: *mut cairo_t, family: *const ::std::os::raw::c_char, slant: cairo_font_slant_t, weight: cairo_font_weight_t);
    pub fn cairo_set_font_size(cr: *mut cairo_t, size: f64);
    pub fn cairo_set_font_matrix(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    pub fn cairo_get_font_matrix(cr: *mut cairo_t, matrix: *mut cairo_matrix_t);
    pub fn cairo_set_font_options(cr: *mut cairo_t, options: *const cairo_font_options_t);
    pub fn cairo_get_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t);
    pub fn cairo_set_font_face(cr: *mut cairo_t, font_face: *mut cairo_font_face_t);
    pub fn cairo_get_font_face(cr: *mut cairo_t) -> *mut cairo_font_face_t;
    pub fn cairo_set_scaled_font(cr: *mut cairo_t, scaled_font: *const cairo_scaled_font_t);
    pub fn cairo_get_scaled_font(cr: *mut cairo_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_show_text(cr: *mut cairo_t, utf8: *const ::std::os::raw::c_char);
    pub fn cairo_show_glyphs(cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: ::std::os::raw::c_int);
    pub fn cairo_show_text_glyphs(cr: *mut cairo_t, utf8: *const ::std::os::raw::c_char, utf8_len: ::std::os::raw::c_int, glyphs: *const cairo_glyph_t, num_glyphs: ::std::os::raw::c_int, clusters: *const cairo_text_cluster_t, num_clusters: ::std::os::raw::c_int, cluster_flags: cairo_text_cluster_flags_t);
    pub fn cairo_text_path(cr: *mut cairo_t, utf8: *const ::std::os::raw::c_char);
    pub fn cairo_glyph_path(cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: ::std::os::raw::c_int);
    pub fn cairo_text_extents(cr: *mut cairo_t, utf8: *const ::std::os::raw::c_char, extents: *mut cairo_text_extents_t);
    pub fn cairo_glyph_extents(cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: ::std::os::raw::c_int, extents: *mut cairo_text_extents_t);
    pub fn cairo_font_extents(cr: *mut cairo_t, extents: *mut cairo_font_extents_t);
    pub fn cairo_font_face_reference(font_face: *mut cairo_font_face_t) -> *mut cairo_font_face_t;
    pub fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
    pub fn cairo_font_face_get_reference_count(font_face: *mut cairo_font_face_t) -> ::std::os::raw::c_uint;
    pub fn cairo_font_face_status(font_face: *mut cairo_font_face_t) -> cairo_status_t;
    pub fn cairo_font_face_get_type(font_face: *mut cairo_font_face_t) -> cairo_font_type_t;
    pub fn cairo_font_face_get_user_data(font_face: *mut cairo_font_face_t, key: *const cairo_user_data_key_t) -> *mut ::std::os::raw::c_void;
    pub fn cairo_font_face_set_user_data(font_face: *mut cairo_font_face_t, key: *const cairo_user_data_key_t, user_data: *mut ::std::os::raw::c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    pub fn cairo_scaled_font_create(font_face: *mut cairo_font_face_t, font_matrix: *const cairo_matrix_t, ctm: *const cairo_matrix_t, options: *const cairo_font_options_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_reference(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    pub fn cairo_scaled_font_get_reference_count(scaled_font: *mut cairo_scaled_font_t) -> ::std::os::raw::c_uint;
    pub fn cairo_scaled_font_status(scaled_font: *mut cairo_scaled_font_t) -> cairo_status_t;
    pub fn cairo_scaled_font_get_type(scaled_font: *mut cairo_scaled_font_t) -> cairo_font_type_t;
    pub fn cairo_scaled_font_get_user_data(scaled_font: *mut cairo_scaled_font_t, key: *const cairo_user_data_key_t) -> *mut ::std::os::raw::c_void;
    pub fn cairo_scaled_font_set_user_data(scaled_font: *mut cairo_scaled_font_t, key: *const cairo_user_data_key_t, user_data: *mut ::std::os::raw::c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t, extents: *mut cairo_font_extents_t);
    pub fn cairo_scaled_font_text_extents(scaled_font: *mut cairo_scaled_font_t, utf8: *const ::std::os::raw::c_char, extents: *mut cairo_text_extents_t);
    pub fn cairo_scaled_font_glyph_extents(scaled_font: *mut cairo_scaled_font_t, glyphs: *const cairo_glyph_t, num_glyphs: ::std::os::raw::c_int, extents: *mut cairo_text_extents_t);
    pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *mut cairo_scaled_font_t, x: f64, y: f64, utf8: *const ::std::os::raw::c_char, utf8_len: ::std::os::raw::c_int, glyphs: *mut *mut cairo_glyph_t, num_glyphs: *mut ::std::os::raw::c_int, clusters: *mut *mut cairo_text_cluster_t, num_clusters: *mut ::std::os::raw::c_int, cluster_flags: *mut cairo_text_cluster_flags_t) -> cairo_status_t;
    pub fn cairo_scaled_font_get_font_face(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_font_face_t;
    pub fn cairo_scaled_font_get_font_matrix(scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut cairo_matrix_t);
    pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t, ctm: *mut cairo_matrix_t);
    pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *mut cairo_scaled_font_t, scale_matrix: *mut cairo_matrix_t);
    pub fn cairo_scaled_font_get_font_options(scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t);
    pub fn cairo_toy_font_face_create(family: *const ::std::os::raw::c_char, slant: cairo_font_slant_t, weight: cairo_font_weight_t) -> *mut cairo_font_face_t;
    pub fn cairo_toy_font_face_get_family(font_face: *mut cairo_font_face_t) -> *const ::std::os::raw::c_char;
    pub fn cairo_toy_font_face_get_slant(font_face: *mut cairo_font_face_t) -> cairo_font_slant_t;
    pub fn cairo_toy_font_face_get_weight(font_face: *mut cairo_font_face_t) -> cairo_font_weight_t;
    pub fn cairo_user_font_face_create() -> *mut cairo_font_face_t;
    pub fn cairo_user_font_face_set_init_func(font_face: *mut cairo_font_face_t, init_func: cairo_user_scaled_font_init_func_t);
    pub fn cairo_user_font_face_set_render_glyph_func(font_face: *mut cairo_font_face_t, render_glyph_func: cairo_user_scaled_font_render_glyph_func_t);
    pub fn cairo_user_font_face_set_text_to_glyphs_func(font_face: *mut cairo_font_face_t, text_to_glyphs_func: cairo_user_scaled_font_text_to_glyphs_func_t);
    pub fn cairo_user_font_face_set_unicode_to_glyph_func(font_face: *mut cairo_font_face_t, unicode_to_glyph_func: cairo_user_scaled_font_unicode_to_glyph_func_t);
    pub fn cairo_user_font_face_get_init_func(font_face: *mut cairo_font_face_t) -> cairo_user_scaled_font_init_func_t;
    pub fn cairo_user_font_face_get_render_glyph_func(font_face: *mut cairo_font_face_t) -> cairo_user_scaled_font_render_glyph_func_t;
    pub fn cairo_user_font_face_get_text_to_glyphs_func(font_face: *mut cairo_font_face_t) -> cairo_user_scaled_font_text_to_glyphs_func_t;
    pub fn cairo_user_font_face_get_unicode_to_glyph_func(font_face: *mut cairo_font_face_t) -> cairo_user_scaled_font_unicode_to_glyph_func_t;
    pub fn cairo_get_operator(cr: *mut cairo_t) -> cairo_operator_t;
    pub fn cairo_get_source(cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_get_tolerance(cr: *mut cairo_t) -> f64;
    pub fn cairo_get_antialias(cr: *mut cairo_t) -> cairo_antialias_t;
    pub fn cairo_has_current_point(cr: *mut cairo_t) -> cairo_bool_t;
    pub fn cairo_get_current_point(cr: *mut cairo_t, x: *mut f64, y: *mut f64);
    pub fn cairo_get_fill_rule(cr: *mut cairo_t) -> cairo_fill_rule_t;
    pub fn cairo_get_line_width(cr: *mut cairo_t) -> f64;
    pub fn cairo_get_line_cap(cr: *mut cairo_t) -> cairo_line_cap_t;
    pub fn cairo_get_line_join(cr: *mut cairo_t) -> cairo_line_join_t;
    pub fn cairo_get_miter_limit(cr: *mut cairo_t) -> f64;
    pub fn cairo_get_dash_count(cr: *mut cairo_t) -> ::std::os::raw::c_int;
    pub fn cairo_get_dash(cr: *mut cairo_t, dashes: *mut f64, offset: *mut f64);
    pub fn cairo_get_matrix(cr: *mut cairo_t, matrix: *mut cairo_matrix_t);
    pub fn cairo_get_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_get_group_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_copy_path(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_copy_path_flat(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_append_path(cr: *mut cairo_t, path: *const cairo_path_t);
    pub fn cairo_path_destroy(path: *mut cairo_path_t);
    pub fn cairo_status(cr: *mut cairo_t) -> cairo_status_t;
    pub fn cairo_status_to_string(status: cairo_status_t) -> *const ::std::os::raw::c_char;
    pub fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
    pub fn cairo_device_get_type(device: *mut cairo_device_t) -> cairo_device_type_t;
    pub fn cairo_device_status(device: *mut cairo_device_t) -> cairo_status_t;
    pub fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    pub fn cairo_device_release(device: *mut cairo_device_t);
    pub fn cairo_device_flush(device: *mut cairo_device_t);
    pub fn cairo_device_finish(device: *mut cairo_device_t);
    pub fn cairo_device_destroy(device: *mut cairo_device_t);
    pub fn cairo_device_get_reference_count(device: *mut cairo_device_t) -> ::std::os::raw::c_uint;
    pub fn cairo_device_get_user_data(device: *mut cairo_device_t, key: *const cairo_user_data_key_t) -> *mut ::std::os::raw::c_void;
    pub fn cairo_device_set_user_data(device: *mut cairo_device_t, key: *const cairo_user_data_key_t, user_data: *mut ::std::os::raw::c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    pub fn cairo_surface_create_similar(other: *mut cairo_surface_t, content: cairo_content_t, width: ::std::os::raw::c_int, height: ::std::os::raw::c_int) -> *mut cairo_surface_t;
    pub fn cairo_surface_create_similar_image(other: *mut cairo_surface_t, format: cairo_format_t, width: ::std::os::raw::c_int, height: ::std::os::raw::c_int) -> *mut cairo_surface_t;
    pub fn cairo_surface_map_to_image(surface: *mut cairo_surface_t, extents: *const cairo_rectangle_int_t) -> *mut cairo_surface_t;
    pub fn cairo_surface_unmap_image(surface: *mut cairo_surface_t, image: *mut cairo_surface_t);
    pub fn cairo_surface_create_for_rectangle(target: *mut cairo_surface_t, x: f64, y: f64, width: f64, height: f64) -> *mut cairo_surface_t;
    pub fn cairo_surface_create_observer(target: *mut cairo_surface_t, mode: cairo_surface_observer_mode_t) -> *mut cairo_surface_t;
    pub fn cairo_surface_observer_add_paint_callback(abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t, data: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_observer_add_mask_callback(abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t, data: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_observer_add_fill_callback(abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t, data: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_observer_add_stroke_callback(abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t, data: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_observer_add_glyphs_callback(abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t, data: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_observer_add_flush_callback(abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t, data: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_observer_add_finish_callback(abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t, data: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_observer_print(surface: *mut cairo_surface_t, write_func: cairo_write_func_t, closure: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_observer_elapsed(surface: *mut cairo_surface_t) -> f64;
    pub fn cairo_device_observer_print(device: *mut cairo_device_t, write_func: cairo_write_func_t, closure: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_device_observer_elapsed(device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_paint_elapsed(device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_mask_elapsed(device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_fill_elapsed(device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_stroke_elapsed(device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_glyphs_elapsed(device: *mut cairo_device_t) -> f64;
    pub fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    pub fn cairo_surface_finish(surface: *mut cairo_surface_t);
    pub fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    pub fn cairo_surface_get_device(surface: *mut cairo_surface_t) -> *mut cairo_device_t;
    pub fn cairo_surface_get_reference_count(surface: *mut cairo_surface_t) -> ::std::os::raw::c_uint;
    pub fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    pub fn cairo_surface_get_type(surface: *mut cairo_surface_t) -> cairo_surface_type_t;
    pub fn cairo_surface_get_content(surface: *mut cairo_surface_t) -> cairo_content_t;
    pub fn cairo_surface_get_user_data(surface: *mut cairo_surface_t, key: *const cairo_user_data_key_t) -> *mut ::std::os::raw::c_void;
    pub fn cairo_surface_set_user_data(surface: *mut cairo_surface_t, key: *const cairo_user_data_key_t, user_data: *mut ::std::os::raw::c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    pub fn cairo_surface_get_mime_data(surface: *mut cairo_surface_t, mime_type: *const ::std::os::raw::c_char, data: *mut *const ::std::os::raw::c_uchar, length: *mut ::std::os::raw::c_ulong);
    pub fn cairo_surface_set_mime_data(surface: *mut cairo_surface_t, mime_type: *const ::std::os::raw::c_char, data: *const ::std::os::raw::c_uchar, length: ::std::os::raw::c_ulong, destroy: cairo_destroy_func_t, closure: *mut ::std::os::raw::c_void) -> cairo_status_t;
    pub fn cairo_surface_supports_mime_type(surface: *mut cairo_surface_t, mime_type: *const ::std::os::raw::c_char) -> cairo_bool_t;
    pub fn cairo_surface_get_font_options(surface: *mut cairo_surface_t, options: *mut cairo_font_options_t);
    pub fn cairo_surface_flush(surface: *mut cairo_surface_t);
    pub fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t);
    pub fn cairo_surface_mark_dirty_rectangle(surface: *mut cairo_surface_t, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int, width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);
    pub fn cairo_surface_set_device_scale(surface: *mut cairo_surface_t, x_scale: f64, y_scale: f64);
    pub fn cairo_surface_get_device_scale(surface: *mut cairo_surface_t, x_scale: *mut f64, y_scale: *mut f64);
    pub fn cairo_surface_set_device_offset(surface: *mut cairo_surface_t, x_offset: f64, y_offset: f64);
    pub fn cairo_surface_get_device_offset(surface: *mut cairo_surface_t, x_offset: *mut f64, y_offset: *mut f64);
    pub fn cairo_surface_set_fallback_resolution(surface: *mut cairo_surface_t, x_pixels_per_inch: f64, y_pixels_per_inch: f64);
    pub fn cairo_surface_get_fallback_resolution(surface: *mut cairo_surface_t, x_pixels_per_inch: *mut f64, y_pixels_per_inch: *mut f64);
    pub fn cairo_surface_copy_page(surface: *mut cairo_surface_t);
    pub fn cairo_surface_show_page(surface: *mut cairo_surface_t);
    pub fn cairo_surface_has_show_text_glyphs(surface: *mut cairo_surface_t) -> cairo_bool_t;
    pub fn cairo_image_surface_create(format: cairo_format_t, width: ::std::os::raw::c_int, height: ::std::os::raw::c_int) -> *mut cairo_surface_t;
    pub fn cairo_format_stride_for_width(format: cairo_format_t, width: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn cairo_image_surface_create_for_data(data: *mut ::std::os::raw::c_uchar, format: cairo_format_t, width: ::std::os::raw::c_int, height: ::std::os::raw::c_int, stride: ::std::os::raw::c_int) -> *mut cairo_surface_t;
    pub fn cairo_image_surface_get_data(surface: *mut cairo_surface_t) -> *mut ::std::os::raw::c_uchar;
    pub fn cairo_image_surface_get_format(surface: *mut cairo_surface_t) -> cairo_format_t;
    pub fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) -> ::std::os::raw::c_int;
    pub fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) -> ::std::os::raw::c_int;
    pub fn cairo_image_surface_get_stride(surface: *mut cairo_surface_t) -> ::std::os::raw::c_int;
    pub fn cairo_recording_surface_create(content: cairo_content_t, extents: *const cairo_rectangle_t) -> *mut cairo_surface_t;
    pub fn cairo_recording_surface_ink_extents(surface: *mut cairo_surface_t, x0: *mut f64, y0: *mut f64, width: *mut f64, height: *mut f64);
    pub fn cairo_recording_surface_get_extents(surface: *mut cairo_surface_t, extents: *mut cairo_rectangle_t) -> cairo_bool_t;
    pub fn cairo_pattern_create_raster_source(user_data: *mut ::std::os::raw::c_void, content: cairo_content_t, width: ::std::os::raw::c_int, height: ::std::os::raw::c_int) -> *mut cairo_pattern_t;
    pub fn cairo_raster_source_pattern_set_callback_data(pattern: *mut cairo_pattern_t, data: *mut ::std::os::raw::c_void);
    pub fn cairo_raster_source_pattern_get_callback_data(pattern: *mut cairo_pattern_t) -> *mut ::std::os::raw::c_void;
    pub fn cairo_raster_source_pattern_set_acquire(pattern: *mut cairo_pattern_t, acquire: cairo_raster_source_acquire_func_t, release: cairo_raster_source_release_func_t);
    pub fn cairo_raster_source_pattern_get_acquire(pattern: *mut cairo_pattern_t, acquire: *mut cairo_raster_source_acquire_func_t, release: *mut cairo_raster_source_release_func_t);
    pub fn cairo_raster_source_pattern_set_snapshot(pattern: *mut cairo_pattern_t, snapshot: cairo_raster_source_snapshot_func_t);
    pub fn cairo_raster_source_pattern_get_snapshot(pattern: *mut cairo_pattern_t) -> cairo_raster_source_snapshot_func_t;
    pub fn cairo_raster_source_pattern_set_copy(pattern: *mut cairo_pattern_t, copy: cairo_raster_source_copy_func_t);
    pub fn cairo_raster_source_pattern_get_copy(pattern: *mut cairo_pattern_t) -> cairo_raster_source_copy_func_t;
    pub fn cairo_raster_source_pattern_set_finish(pattern: *mut cairo_pattern_t, finish: cairo_raster_source_finish_func_t);
    pub fn cairo_raster_source_pattern_get_finish(pattern: *mut cairo_pattern_t) -> cairo_raster_source_finish_func_t;
    pub fn cairo_pattern_create_rgb(red: f64, green: f64, blue: f64) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_mesh() -> *mut cairo_pattern_t;
    pub fn cairo_pattern_reference(pattern: *mut cairo_pattern_t) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    pub fn cairo_pattern_get_reference_count(pattern: *mut cairo_pattern_t) -> ::std::os::raw::c_uint;
    pub fn cairo_pattern_status(pattern: *mut cairo_pattern_t) -> cairo_status_t;
    pub fn cairo_pattern_get_user_data(pattern: *mut cairo_pattern_t, key: *const cairo_user_data_key_t) -> *mut ::std::os::raw::c_void;
    pub fn cairo_pattern_set_user_data(pattern: *mut cairo_pattern_t, key: *const cairo_user_data_key_t, user_data: *mut ::std::os::raw::c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
    pub fn cairo_pattern_get_type(pattern: *mut cairo_pattern_t) -> cairo_pattern_type_t;
    pub fn cairo_pattern_add_color_stop_rgb(pattern: *mut cairo_pattern_t, offset: f64, red: f64, green: f64, blue: f64);
    pub fn cairo_pattern_add_color_stop_rgba(pattern: *mut cairo_pattern_t, offset: f64, red: f64, green: f64, blue: f64, alpha: f64);
    pub fn cairo_mesh_pattern_begin_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_end_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_curve_to(pattern: *mut cairo_pattern_t, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
    pub fn cairo_mesh_pattern_line_to(pattern: *mut cairo_pattern_t, x: f64, y: f64);
    pub fn cairo_mesh_pattern_move_to(pattern: *mut cairo_pattern_t, x: f64, y: f64);
    pub fn cairo_mesh_pattern_set_control_point(pattern: *mut cairo_pattern_t, point_num: ::std::os::raw::c_uint, x: f64, y: f64);
    pub fn cairo_mesh_pattern_set_corner_color_rgb(pattern: *mut cairo_pattern_t, corner_num: ::std::os::raw::c_uint, red: f64, green: f64, blue: f64);
    pub fn cairo_mesh_pattern_set_corner_color_rgba(pattern: *mut cairo_pattern_t, corner_num: ::std::os::raw::c_uint, red: f64, green: f64, blue: f64, alpha: f64);
    pub fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t, matrix: *const cairo_matrix_t);
    pub fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t);
    pub fn cairo_pattern_set_extend(pattern: *mut cairo_pattern_t, extend: cairo_extend_t);
    pub fn cairo_pattern_get_extend(pattern: *mut cairo_pattern_t) -> cairo_extend_t;
    pub fn cairo_pattern_set_filter(pattern: *mut cairo_pattern_t, filter: cairo_filter_t);
    pub fn cairo_pattern_get_filter(pattern: *mut cairo_pattern_t) -> cairo_filter_t;
    pub fn cairo_pattern_get_rgba(pattern: *mut cairo_pattern_t, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> cairo_status_t;
    pub fn cairo_pattern_get_surface(pattern: *mut cairo_pattern_t, surface: *mut *mut cairo_surface_t) -> cairo_status_t;
    pub fn cairo_pattern_get_color_stop_rgba(pattern: *mut cairo_pattern_t, index: ::std::os::raw::c_int, offset: *mut f64, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> cairo_status_t;
    pub fn cairo_pattern_get_color_stop_count(pattern: *mut cairo_pattern_t, count: *mut ::std::os::raw::c_int) -> cairo_status_t;
    pub fn cairo_pattern_get_linear_points(pattern: *mut cairo_pattern_t, x0: *mut f64, y0: *mut f64, x1: *mut f64, y1: *mut f64) -> cairo_status_t;
    pub fn cairo_pattern_get_radial_circles(pattern: *mut cairo_pattern_t, x0: *mut f64, y0: *mut f64, r0: *mut f64, x1: *mut f64, y1: *mut f64, r1: *mut f64) -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_patch_count(pattern: *mut cairo_pattern_t, count: *mut ::std::os::raw::c_uint) -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_path(pattern: *mut cairo_pattern_t, patch_num: ::std::os::raw::c_uint) -> *mut cairo_path_t;
    pub fn cairo_mesh_pattern_get_corner_color_rgba(pattern: *mut cairo_pattern_t, patch_num: ::std::os::raw::c_uint, corner_num: ::std::os::raw::c_uint, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_control_point(pattern: *mut cairo_pattern_t, patch_num: ::std::os::raw::c_uint, point_num: ::std::os::raw::c_uint, x: *mut f64, y: *mut f64) -> cairo_status_t;
    pub fn cairo_matrix_init(matrix: *mut cairo_matrix_t, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);
    pub fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    pub fn cairo_matrix_init_translate(matrix: *mut cairo_matrix_t, tx: f64, ty: f64);
    pub fn cairo_matrix_init_scale(matrix: *mut cairo_matrix_t, sx: f64, sy: f64);
    pub fn cairo_matrix_init_rotate(matrix: *mut cairo_matrix_t, radians: f64);
    pub fn cairo_matrix_translate(matrix: *mut cairo_matrix_t, tx: f64, ty: f64);
    pub fn cairo_matrix_scale(matrix: *mut cairo_matrix_t, sx: f64, sy: f64);
    pub fn cairo_matrix_rotate(matrix: *mut cairo_matrix_t, radians: f64);
    pub fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    pub fn cairo_matrix_multiply(result: *mut cairo_matrix_t, a: *const cairo_matrix_t, b: *const cairo_matrix_t);
    pub fn cairo_matrix_transform_distance(matrix: *const cairo_matrix_t, dx: *mut f64, dy: *mut f64);
    pub fn cairo_matrix_transform_point(matrix: *const cairo_matrix_t, x: *mut f64, y: *mut f64);
    pub fn cairo_region_create() -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangle(rectangle: *const cairo_rectangle_int_t) -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangles(rects: *const cairo_rectangle_int_t, count: ::std::os::raw::c_int) -> *mut cairo_region_t;
    pub fn cairo_region_copy(original: *const cairo_region_t) -> *mut cairo_region_t;
    pub fn cairo_region_reference(region: *mut cairo_region_t) -> *mut cairo_region_t;
    pub fn cairo_region_destroy(region: *mut cairo_region_t);
    pub fn cairo_region_equal(a: *const cairo_region_t, b: *const cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_status(region: *const cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_get_extents(region: *const cairo_region_t, extents: *mut cairo_rectangle_int_t);
    pub fn cairo_region_num_rectangles(region: *const cairo_region_t) -> ::std::os::raw::c_int;
    pub fn cairo_region_get_rectangle(region: *const cairo_region_t, nth: ::std::os::raw::c_int, rectangle: *mut cairo_rectangle_int_t);
    pub fn cairo_region_is_empty(region: *const cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_contains_rectangle(region: *const cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_region_overlap_t;
    pub fn cairo_region_contains_point(region: *const cairo_region_t, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> cairo_bool_t;
    pub fn cairo_region_translate(region: *mut cairo_region_t, dx: ::std::os::raw::c_int, dy: ::std::os::raw::c_int);
    pub fn cairo_region_subtract(dst: *mut cairo_region_t, other: *const cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_subtract_rectangle(dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_status_t;
    pub fn cairo_region_intersect(dst: *mut cairo_region_t, other: *const cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_intersect_rectangle(dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_status_t;
    pub fn cairo_region_union(dst: *mut cairo_region_t, other: *const cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_union_rectangle(dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_status_t;
    pub fn cairo_region_xor(dst: *mut cairo_region_t, other: *const cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_xor_rectangle(dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_status_t;
    pub fn cairo_debug_reset_static_data();
    pub fn cairo_image_surface_create_from_png(filename: *const ::std::os::raw::c_char) -> *mut cairo_surface_t;
}