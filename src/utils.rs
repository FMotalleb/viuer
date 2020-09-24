use std::env;

const DEFAULT_TERM_SIZE: (u16, u16) = (80, 24);

pub fn truecolor_available() -> bool {
    if let Ok(value) = env::var("COLORTERM") {
        value.contains("truecolor") || value.contains("24bit")
    } else {
        false
    }
}

/// Try to get the terminal size. If unsuccessful, fallback to constant dimensions.
///
/// Uses [crossterm::terminal::size].
/// ## Example
/// TODO: center an image example (requires offsets)
/// ```
/// use viuer::Config;
///
/// let conf = Config {
///     width: Some(30),
///     x: 40,
///     y: 40,
///     transparent: true,
///     ..Default::default()
/// };
/// ```
#[cfg(not(test))]
pub fn terminal_size() -> (u16, u16) {
    match crossterm::terminal::size() {
        Ok(s) => s,
        Err(_) => DEFAULT_TERM_SIZE,
    }
}

// Return a constant when running the tests
#[cfg(test)]
pub fn terminal_size() -> (u16, u16) {
    DEFAULT_TERM_SIZE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truecolor() {
        env::set_var("COLORTERM", "truecolor");
        assert!(truecolor_available());
        env::set_var("COLORTERM", "");
        assert!(!truecolor_available());
    }
}
