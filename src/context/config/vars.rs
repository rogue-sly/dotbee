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
pub fn interpolate(input: &str, vars: &IndexMap<String, String>, context: &str) -> Result<String, Vec<String>> {
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

    if errors.is_empty() { Ok(output) } else { Err(errors) }
}

fn is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::interpolate;
    use indexmap::IndexMap;

    fn vars() -> IndexMap<String, String> {
        IndexMap::from([
            ("config".to_string(), "~/.config".to_string()),
            ("nvim".to_string(), "~/.config/nvim".to_string()),
        ])
    }

    #[test]
    fn substitutes_single_var() {
        assert_eq!(interpolate("{config}/foo", &vars(), "ctx").unwrap(), "~/.config/foo");
    }

    #[test]
    fn substitutes_var_at_start_middle_end() {
        let v = vars();
        assert_eq!(interpolate("{config}", &v, "ctx").unwrap(), "~/.config");
        assert_eq!(interpolate("a/{config}/b", &v, "ctx").unwrap(), "a/~/.config/b");
        assert_eq!(interpolate("a/{config}", &v, "ctx").unwrap(), "a/~/.config");
    }

    #[test]
    fn substitutes_multiple_vars() {
        assert_eq!(
            interpolate("{config}/x {nvim}", &vars(), "ctx").unwrap(),
            "~/.config/x ~/.config/nvim"
        );
    }

    #[test]
    fn leaves_plain_text_untouched() {
        assert_eq!(interpolate("~/.config/nvim", &vars(), "ctx").unwrap(), "~/.config/nvim");
    }

    #[test]
    fn undefined_var_is_an_error() {
        let err = interpolate("{missing}/foo", &vars(), "[p]: link 'a' src").unwrap_err();
        assert_eq!(err, vec!["[p]: link 'a' src: undefined variable 'missing'"]);
    }

    #[test]
    fn collects_all_undefined_vars() {
        let err = interpolate("{a}/{b}", &vars(), "ctx").unwrap_err();
        assert_eq!(err, vec!["ctx: undefined variable 'a'", "ctx: undefined variable 'b'"]);
    }

    #[test]
    fn double_braces_escape() {
        assert_eq!(interpolate("{{config}}", &vars(), "ctx").unwrap(), "{config}");
        assert_eq!(interpolate("a{{b", &vars(), "ctx").unwrap(), "a{b");
        assert_eq!(interpolate("a}}b", &vars(), "ctx").unwrap(), "a}b");
    }

    #[test]
    fn invalid_identifier_is_left_literal() {
        let v = vars();
        assert_eq!(interpolate("{foo bar}", &v, "ctx").unwrap(), "{foo bar}");
        assert_eq!(interpolate("{}", &v, "ctx").unwrap(), "{}");
        assert_eq!(interpolate("{ config }", &v, "ctx").unwrap(), "{ config }");
        assert_eq!(interpolate("{a{b}}", &v, "ctx").unwrap(), "{a{b}}");
    }

    #[test]
    fn unmatched_braces_are_literal() {
        assert_eq!(interpolate("a{b", &vars(), "ctx").unwrap(), "a{b");
        assert_eq!(interpolate("a}", &vars(), "ctx").unwrap(), "a}");
    }

    #[test]
    fn empty_input() {
        assert_eq!(interpolate("", &vars(), "ctx").unwrap(), "");
    }

    #[test]
    fn unicode_is_preserved() {
        assert_eq!(interpolate("café/{config}", &vars(), "ctx").unwrap(), "café/~/.config");
    }
}
