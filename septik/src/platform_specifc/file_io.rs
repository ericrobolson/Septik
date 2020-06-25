use std::path::Path;

pub fn assets_path(file: String) -> String {
    let path = format!("./assets/{}", file);
    path
}
