use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use aoc2025::{parse_string_to_i64, split_range};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn split_data(input: Vec<String>) -> Result<(Vec<(i64, i64)>, Vec<i64>)> {
    let mut fresh_ranges: Vec<(i64, i64)> = vec![];
    let mut available_ingredients: Vec<i64> = vec![];
    let mut ranges: bool = true;

    for line in input {
        // switch between input handling
        if line == "" {
            ranges = false;
            continue;
        }

        if ranges {
            fresh_ranges.push(split_range(line)?);
        } else {
            available_ingredients.push(parse_string_to_i64(&line)?);
        }
    }
    Ok((fresh_ranges, available_ingredients))
}

fn reducer(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|(s, _)| *s);

    let mut reduced = Vec::new();
    let (mut cur_start, mut cur_end) = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= cur_end + 1 {
            // overlap or directly adjacent
            cur_end = cur_end.max(end);
        } else {
            reduced.push((cur_start, cur_end));
            cur_start = start;
            cur_end = end;
        }
    }

    reduced.push((cur_start, cur_end));
    reduced
}

fn get_result(input: Vec<String>) -> Result<(Vec<i64>, i64)> {
    let (fresh_ranges, available_ingredients) = split_data(input)?;
    let mut results: Vec<i64> = vec![];
    let mut total_fresh_ids: i64 = 0;
    let reduced_fresh_ranges = reducer(fresh_ranges);

    for (start, end) in &reduced_fresh_ranges {
        let range = *start..=*end;
        total_fresh_ids += end - start + 1;
        for available_ingredient in &available_ingredients {
            if range.contains(available_ingredient) {
                if ! results.contains(available_ingredient) {
                    results.push(*available_ingredient);
                }
            }
        }
    }

    return Ok((results, total_fresh_ids));
}

fn day5() -> Result<(Vec<i64>, i64)> {
    let input: Vec<String> = read_lines("src/data/input5.txt")?;
    get_result(input)
}

fn main() -> Result<()> {
    let (result1, result2) = day5()?;
    println!("Hello from Day 5:");
    println!("The result of part 1 is {}!", result1.len());
    println!("The result of part 2 is {}!", result2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5() -> Result<()>  {
        let input: Vec<String> = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            "".to_string(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string()
        ];
        let expected_results: Vec<i64> = vec![5, 11, 17];
        let expected_sum = 3;
        let expected_total_fresh_ids = 14;

        let (result1, result2) = get_result(input)?;
        assert_eq!(result1, expected_results);
        assert_eq!(result1.len(), expected_sum);
        assert_eq!(result2, expected_total_fresh_ids);

        Ok(())
    }
}
