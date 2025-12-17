use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn splitter(input: Vec<String>) -> Result<Vec<(i64, i64)>> {
    let mut id_ranges: Vec<(i64, i64)> = vec![];
    let mut ranges = input[0].split(',');
    for range in ranges.by_ref() {
        let mut values = range.split('-');
        let start = values.next().ok_or(anyhow!("no start value in this range"))?.trim().parse::<i64>()?;
        let end = values.next().ok_or(anyhow!("no end value in this range"))?.trim().parse::<i64>()?;
        id_ranges.push((start, end));
    }
    Ok(id_ranges)
}

fn step(id: i64) -> Option<i64> {
    let id_str = id.to_string();
    let length = id_str.chars().count();
    if length % 2 == 0 {
        let (first, second) = id_str.split_at(length / 2);
        if first == second {
            return Some(id);
        }
    }
    return None;
}

fn find_invalid_ids(ranges: Vec<(i64, i64)>) -> Result<(Vec<i64>, i64)> {
    let mut results: Vec<i64> = vec![];
    let mut sum: i64 = 0;

    for (start, end) in ranges {
        for id in start..end+1 {
            if let Some(invalid_id) = step(id) {
                results.push(invalid_id);
                sum += invalid_id;
            }
        }
    }
    return Ok((results, sum));
}

fn day2() -> Result<(Vec<i64>, i64)> {
    let input: Vec<String> = read_lines("src/data/input2.txt")?;
    let ranges: Vec<(i64, i64)> = splitter(input)?;
    find_invalid_ids(ranges)
}

fn main() -> Result<()> {
    let (_, sum) = day2()?;
    println!("Hello from Day 2:");
    println!("First solution is {}!", sum);
    // println!("Second Password is {}!", loops+password);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() -> Result<()>  {
        let input: Vec<String> = read_lines("src/data/testInput2.txt")?;
        let ranges: Vec<(i64, i64)> = splitter(input)?;
        let (_, sum) = find_invalid_ids(ranges)?;
        assert_eq!(sum, 1227775554);
        Ok(())
    }
}
