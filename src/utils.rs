use std::path::Path;
use std::ffi::OsStr;



const SUPPORTED_TYPES: &[&str] = &["py", "txt", "rs", "json"];




pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    let res = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);

    assert!(
        res.map_or(false, |ext| SUPPORTED_TYPES.contains(&ext)),
        "File type not supported. Must be one of {:?}", SUPPORTED_TYPES
    );
    res
}