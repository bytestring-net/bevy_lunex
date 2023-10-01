// ===========================================================
// === CRATE SPECIFIC ===

/// Returns if a string represents a numerical ID in the tree
pub(crate) fn is_numerical_id(str: &str) -> bool {
    match str.chars().nth(0) {
        Some(value) => value == '#',
        None => false,
    }
}


/// ### Extract ID
/// This will extract id from numeric path
pub(crate) fn extract_id(str: &str) -> Result<usize, String> {
    match str.chars().nth(0) {
        Some(_) => match str::parse::<usize>(&str[1..]) {
            Ok (value) => Ok (value),
            Err (_) => Err (format!("{} caused syntax error!", str))
        },
        None => Err (format!("This is not a numeric path!")),
    }
}
