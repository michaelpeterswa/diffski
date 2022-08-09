use serde::Serialize;
use similar::{ChangeTag, TextDiff};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct DiffData {
    diff: Vec<String>,
}

// simple addition function to test WASM
#[wasm_bindgen]
pub fn diff(a: &str, b: &str) -> String {
    let diff = TextDiff::from_lines(a, b);

    let mut diffs: Vec<String> = Vec::new();
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        diffs.push(format!("{}{}", sign, change));
    }

    let diff_data = DiffData { diff: diffs };

    return serde_json::to_string(&diff_data).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::diff;

    #[test]
    fn test_diff_mismatch() {
        let result = diff("hello\ngoodbye", "hello\nworld");
        assert_eq!(result, "{\"diff\":[\" hello\\n\",\"-goodbye\\n\",\"+world\\n\"]}");
    }

    #[test]
    fn test_diff_match() {
        let result = diff("hello\ngoodbye", "hello\ngoodbye");
        assert_eq!(result, "{\"diff\":[\" hello\\n\",\" goodbye\\n\"]}");
    }
}
