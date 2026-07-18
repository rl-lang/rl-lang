use crate::entry::FnEntry;

pub static MAP_RANGE: FnEntry = FnEntry {
    signature: "map_range(x, in_min, in_max, out_min, out_max)",
    description: "re-map x from one range to another",
    example: "get std::math::map_range\n\nmap_range(5.0, 0.0, 10.0, 0.0, 100.0)",
    expected_output: Some("50.0"),
    returns: "float",
    errors: None,
    see_also: &["lerp", "clamp"],
    since: Some("v0.1.5"),
};
