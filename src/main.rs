use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn split_first(s: &str) -> Result<(char, i64)> {
    let mut chars = s.chars();
    let first = chars.next().ok_or(anyhow!("empty string"))?;
    let rest = chars.as_str().trim();
    let value = rest.parse::<i64>()?;
    Ok((first, value))
}

fn step(mut current: i64, rotation: &str, max: i64) -> Result<i64> {
    let (rot, distance) = split_first(rotation)?;

    if rot == 'R' || rot == 'r' {
        current = (current + distance) % max;
    } else if rot == 'L' || rot == 'l' {
        current = ((current - distance) % max + max) % max;
    } else {
        return Err(anyhow!(format!("Line has no rotation: {}", rotation)));
    }

    return Ok(current);
}

fn get_password(lines: Vec<String>) -> Result<(i32, Vec<i64>)> {
    let mut zeroes: i32 = 0;
    let mut current: i64 = 50;
    let mut results: Vec<i64> = vec![];

    for line in lines {
        current = step(current, &line, 100)?;
        results.push(current);
        if current == 0 {
            zeroes += 1;
        }
    }
    return Ok((zeroes, results));
}

fn day1() -> Result<(i32, Vec<i64>)> {
    let lines: Vec<String> = read_lines("src/input.txt")?;
    get_password(lines)
}

fn main() -> Result<()> {
    println!("Hello from Day 1:");
    let (password, _) = day1()?;
    println!("Password is {}!", password);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() -> Result<()>  {
        let result = step(5, "L103", 10)?;
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn test_get_password() -> Result<()>  {
        let lines: Vec<String> = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string()
        ];
        let expected_results = vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32];
        let expected_password = 3;

        let (password, results) = get_password(lines)?;

        assert_eq!(password, expected_password);
        assert_eq!(results, expected_results);
        Ok(())
    }
}
