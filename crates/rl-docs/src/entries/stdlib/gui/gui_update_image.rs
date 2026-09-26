use crate::entry::FnEntry;

pub static GUI_UPDATE_IMAGE: FnEntry = FnEntry {
    signature: "gui_update_image(image, width, height, rgba)",
    description: "replaces an image widget's pixels (and size), bumping its version so the next frame re-uploads the texture exactly once. Static images never call this and upload once ever; animated ones pay per changed frame only",
    example: r#"get std::gui::gui_image
get std::gui::gui_update_image

dec handle dot = result_unwrap(gui_image(main, 8, 8, solid(8, 8, 0, 255, 0), 500, 100))
gui_update_image(dot, 8, 8, solid(8, 8, 255, 255, 0))?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if dimensions are not positive, if the pixel count mismatches width times height times 4, if `image` is an unknown handle, or if it isn't an image",
    ),
    see_also: &["gui_image", "gui_on_frame"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
