use csv::StringRecord;
use std::ops::Range;

pub fn extract_fields(record: &StringRecord, field_pos: &[Range<usize>]) -> Vec<String> {
    let record = record.iter().collect::<Vec<_>>();

    field_pos
        .iter()
        .flat_map(|range| {
            record
                .iter()
                .skip(range.start)
                .take(range.end - range.start)
        })
        .map(|field| field.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(extract_fields(&rec, &[0..1, 2..3]), &["Captain", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    }
}
