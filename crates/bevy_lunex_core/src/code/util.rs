// ===========================================================
// === CRATE SPECIFIC ===

/// Returns if a string represents a numerical ID in the tree
pub(crate) fn is_numerical_id(str: &str) -> bool {
    match str.chars().nth(0) {
        Some(value) => value == '#',
        None => false,
    }
}

/// Same as `split_once`, but inverted.
pub(crate) fn split_last(string: &str, delimiter: &str) -> (String, String) {
    let str_list: Vec<&str> = string.split(delimiter).collect();
    let mut output = String::new();
    let mut is_first = true;
    for x in str_list.iter().take(str_list.len() - 1) {
        if !is_first {
            output += delimiter
        } else {
            is_first = false
        };
        output += x;
    }
    (output, String::from(str_list[str_list.len() - 1]))
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