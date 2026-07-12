use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_rename_file(_: &mut Evaluator, path: String, new_name: String) -> Value {
    let old_path = std::path::Path::new(&path);
    let new_path = match old_path.parent() {
        Some(parent) => parent.join(&new_name),
        None => std::path::PathBuf::from(&new_name),
    };

    if let Err(e) = std::fs::rename(old_path, &new_path) {
        return verr!(vs!(format!(
            "rename_file(): failed to rename \"{}\" to \"{}\": {}",
            path,
            new_path.to_string_lossy(),
            e
        )));
    };

    vok!(vs!(new_path.to_string_lossy().to_string()))
}
