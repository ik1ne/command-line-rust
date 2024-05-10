use anyhow::{bail, Result};
use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_name = "YEAR", value_parser=clap::value_parser!(i32).range(1..=9999))]
    year: Option<i32>,
    #[arg(short)]
    month: Option<String>,
    #[arg(short = 'y', long = "year", conflicts_with_all = ["year", "month"])]
    show_current_year: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<()> {
    let today = Local::now().date_naive();
    let mut month = args
        .month
        .as_ref()
        .map(|month| parse_month(month))
        .transpose()?;
    let year = args.year.unwrap_or(today.year());

    if !args.show_current_year {
        month = month.or(Some(today.month()));
    }

    println!("month = {month:?}");
    println!("year  = {year:?}");

    Ok(())
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn parse_month(month: &str) -> Result<u32> {
    if let Ok(month) = month.parse::<u32>() {
        if (1..=12).contains(&month) {
            return Ok(month);
        }
        bail!("month \"{}\" not in the range 1 through 12", month);
    }

    let month = month.to_lowercase();

    let mut possible_months = Vec::new();
    for pair in MONTH_NAMES.iter().enumerate() {
        if pair.1.to_lowercase().starts_with(&month) {
            possible_months.push(pair.0 + 1);
        }
    }

    if possible_months.len() == 1 {
        return Ok(possible_months[0] as u32);
    } else {
        bail!("Invalid month \"{}\"", month);
    }
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last = last_day_in_month(year, month);
    let mut days = std::iter::repeat_with(|| "  ".to_string())
        .take(first.weekday().num_days_from_sunday() as usize)
        .collect::<Vec<_>>();

    todo!()
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    let (year, month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    let day = NaiveDate::from_ymd_opt(year, month, 1).expect("invalid date");
    day.pred_opt().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{format_month, last_day_in_month, parse_month};
    use chrono::NaiveDate;

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"month "0" not in the range 1 through 12"#
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"month "13" not in the range 1 through 12"#
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"Invalid month "foo""#);
    }

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
        let leap_february = vec![
            "   February 2020      ",
            "Su Mo Tu We Th Fr Sa  ",
            "                   1  ",
            " 2  3  4  5  6  7  8  ",
            " 9 10 11 12 13 14 15  ",
            "16 17 18 19 20 21 22  ",
            "23 24 25 26 27 28 29  ",
            "                      ",
        ];
        assert_eq!(format_month(2020, 2, true, today), leap_february);

        let may = vec![
            "        May           ",
            "Su Mo Tu We Th Fr Sa  ",
            "                1  2  ",
            " 3  4  5  6  7  8  9  ",
            "10 11 12 13 14 15 16  ",
            "17 18 19 20 21 22 23  ",
            "24 25 26 27 28 29 30  ",
            "31                    ",
        ];
        assert_eq!(format_month(2020, 5, false, today), may);

        let april_hl = vec![
            "     April 2021       ",
            "Su Mo Tu We Th Fr Sa  ",
            "             1  2  3  ",
            " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29 30     ",
            "                      ",
        ];
        let today = NaiveDate::from_ymd_opt(2021, 4, 7).unwrap();
        assert_eq!(format_month(2021, 4, true, today), april_hl);
    }

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(
            last_day_in_month(2020, 1),
            NaiveDate::from_ymd_opt(2020, 1, 31).unwrap()
        );
        assert_eq!(
            last_day_in_month(2020, 2),
            NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
        );
        assert_eq!(
            last_day_in_month(2020, 4),
            NaiveDate::from_ymd_opt(2020, 4, 30).unwrap()
        );
    }
}
