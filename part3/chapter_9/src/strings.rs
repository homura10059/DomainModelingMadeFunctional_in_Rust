#[derive(Debug, PartialEq)]
pub struct String50(String);

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

impl String50 {
    pub fn new(s: String) -> Self {
        Self(truncate(s.as_ref(), 50).to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
