/// Compute the visible display width of a string, stripping ANSI escape codes.
/// Uses simple ESC + '[' prefix detection for SGR/color sequences.
pub fn visible_width(s: &str) -> usize {
    let mut chars = s.chars().peekable();
    let mut width = 0usize;

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            let _ = chars.next();
            for esc in chars.by_ref() {
                if esc.is_ascii_alphabetic() {
                    break;
                }
            }
            continue;
        }
        width += 1;
    }

    width
}

/// Pad `s` on the right with spaces until `visible_width(s) == target_width`.
/// If already at or over width, returns the string unchanged.
pub fn pad_right(s: &str, target_width: usize) -> String {
    let current = visible_width(s);
    if current >= target_width {
        return s.to_owned();
    }
    let mut out = s.to_owned();
    out.extend(std::iter::repeat_n(' ', target_width - current));
    out
}

/// Truncate `s` to `max_width` visible chars, appending `...` if truncated.
pub fn truncate(s: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }
    let len = s.chars().count();
    if len <= max_width {
        return s.to_owned();
    }
    if max_width <= 3 {
        return ".".repeat(max_width);
    }
    let keep = max_width - 3;
    let mut out = String::with_capacity(max_width);
    out.extend(s.chars().take(keep));
    out.push_str("...");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_string() {
        assert_eq!(visible_width("hello"), 5);
    }

    #[test]
    fn ansi_stripped() {
        let s = "\x1b[1mhello\x1b[0m";
        assert_eq!(visible_width(s), 5);
    }

    #[test]
    fn pad_right_pads() {
        let s = pad_right("hi", 6);
        assert_eq!(s, "hi    ");
    }

    #[test]
    fn truncate_short() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 8), "hello...");
    }
}
