// Gets the absolute path to icon files from its filename.
pub fn get_icon_path(icon_name: &str) -> String {
    std::fs::canonicalize(format!("./icons/{icon_name}.png"))
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_string()
}
