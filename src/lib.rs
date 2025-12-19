use anyhow::{Result, anyhow};

pub fn split_range(range: String) -> Result<(i64, i64)> {
    let mut values = range.split('-');
    let start = parse_string_to_i64(values.next().ok_or(anyhow!("no start value in this range"))?)?;
    let end = parse_string_to_i64(values.next().ok_or(anyhow!("no end value in this range"))?)?;
    Ok((start, end))
}

pub fn parse_string_to_i64(s: &str) -> Result<i64> {
    s.trim().parse::<i64>()
        .map_err(|e| anyhow!("failed to parse '{s}': {e}"))
}
