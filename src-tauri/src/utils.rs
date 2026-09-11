use std::path::Path;

pub fn is_valid_mac_filename(name: &str) -> bool {
    // 1. Check for empty string or too long name (max 255 bytes)
    if name.is_empty() || name.len() > 255 {
        return false;
    }

    // 2. Check for reserved relative path tokens
    if name == "." || name == ".." {
        return false;
    }

    // 3. Check for forbidden characters (slash and null byte)
    if name.contains('/') || name.contains('\0') {
        return false;
    }

    // 4. Ensure it doesn't parse into multiple path components if given as a single name
    let path = Path::new(name);
    if path.components().count() != 1 {
        return false;
    }

    true
}