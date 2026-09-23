use indexmap::IndexMap;

/// Interpolates `{name}` placeholders in `input` using the values in `vars`.
///
/// - `{name}` where `name` is a valid identifier (`[A-Za-z0-9_]+`) is replaced
///   with the variable's value.
/// - An undefined but well-formed `{name}` produces an error message carrying
///   the provided `context` (e.g. which profile/link/field it belongs to).
/// - `{{` and `}}` escape to literal `{` and `}`.
/// - Anything that is not a well-formed identifier (spaces, empty, unmatched
///   braces) is left as literal text.
///
/// All errors are collected and returned at once so the user sees every
/// undefined variable in a single run.
pub fn interpolate(
    input: &str,
    vars: &IndexMap<String, String>,
    context: &str,
) -> Result<String, Vec<String>> {
    let mut errors = Vec::new();
    let mut output = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '{' => {
                if chars.get(i + 1) == Some(&'{') {
                    output.push('{');
                    i += 2;
                    continue;
                }

                let mut j = i + 1;
                while j < chars.len() && chars[j] != '}' {
                    j += 1;
                }

                if j == chars.len() {
                    output.push('{');
                    i += 1;
                    continue;
                }

                let name: String = chars[i + 1..j].iter().collect();
                if is_valid_name(&name) {
                    match vars.get(&name) {
                        Some(value) => output.push_str(value),
                        None => errors.push(format!("{context}: undefined variable '{}'", name)),
                    }
                } else {
                    output.push('{');
                    output.push_str(&name);
                    output.push('}');
                }
                i = j + 1;
            }
            '}' => {
                if chars.get(i + 1) == Some(&'}') {
                    output.push('}');
                    i += 2;
                } else {
                    output.push('}');
                    i += 1;
                }
            }
            c => {
                output.push(c);
                i += 1;
            }
        }
    }

    if errors.is_empty() {
        Ok(output)
    } else {
        Err(errors)
    }
}

fn is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
