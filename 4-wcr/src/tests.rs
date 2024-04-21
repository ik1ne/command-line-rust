use super::*;
use std::io::Cursor;

#[test]
fn test_count() {
    let text = "I don't want the world.\nI just want your half.\r\n";
    let info = count_bufread(Cursor::new(text));
    assert!(info.is_ok());
    let expected = Counts {
        lines: 2,
        words: 10,
        bytes: 48,
        chars: 48,
    };
    assert_eq!(info.unwrap(), expected);
}
