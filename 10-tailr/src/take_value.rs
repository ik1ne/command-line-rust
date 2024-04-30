use anyhow::{Context, Result};
use regex::Regex;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TakeValue {
    FromEnd(usize),
    FromStart(usize),
}

static RE: OnceLock<Regex> = OnceLock::new();

impl FromStr for TakeValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)*$").unwrap());

        let caps = re.captures(s).with_context(|| s.to_string())?;

        let sign = caps.get(1).map_or("-", |m| m.as_str());
        let num = caps
            .get(2)
            .with_context(|| format!("{} is not a number", s))?
            .as_str()
            .parse::<usize>()?;

        let value = match sign {
            "+" => TakeValue::FromStart(num),
            _ => TakeValue::FromEnd(num),
        };

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::TakeValue::*;
    use crate::take_value::TakeValue;
    use std::str::FromStr;

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = TakeValue::from_str("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromEnd(3));

        // A leading "+" should result in a positive number
        let res = TakeValue::from_str("+3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromStart(3));

        // An explicit "-" value should result in a negative number
        let res = TakeValue::from_str("-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromEnd(3));

        // Zero is zero
        let res = TakeValue::from_str("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromEnd(0));

        // Plus zero is special
        let res = TakeValue::from_str("+0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromStart(0));

        // Test boundaries
        let res = TakeValue::from_str(&usize::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromEnd(usize::MAX));

        let res = TakeValue::from_str(&(usize::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromEnd(usize::MIN + 1));

        let res = TakeValue::from_str(&format!("+{}", usize::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromStart(usize::MAX));

        let res = TakeValue::from_str(&usize::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), FromEnd(usize::MIN));

        // A floating-point value is invalid
        let res = TakeValue::from_str("3.14");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any non-integer string is invalid
        let res = TakeValue::from_str("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}
