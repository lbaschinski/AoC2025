use anyhow::{Result, anyhow};

pub fn split_range(range: String) -> Result<(i64, i64)> {
    let mut values = range.split('-');
    let start = values.next().ok_or(anyhow!("no start value in this range"))?.trim().parse::<i64>()?;
    let end = values.next().ok_or(anyhow!("no end value in this range"))?.trim().parse::<i64>()?;
    Ok((start, end))
}
