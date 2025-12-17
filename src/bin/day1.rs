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

fn step(mut current: i64, rotation: &str, max: i64) -> Result<(i64, i64)> {
    let (rot, distance) = split_first(rotation)?;
    let mut loops: i64;

    if rot == 'R' || rot == 'r' {
        loops = (current + distance) / max;
        current = (current + distance) % max;
        // if the new value is 0, reduce loops by one since we count the zero later separately
        if current == 0 {
            loops -= 1;
        }
    } else if rot == 'L' || rot == 'l' {
        loops = (current - distance).div_euclid(max);
        if loops < 0 {
            loops = loops * -1;
        }
        // if we are currently on 0, somehow loops is 1 too high
        if current == 0 {
            loops -= 1;
        }
        current = ((current - distance) % max + max) % max;
    } else {
        return Err(anyhow!(format!("Line has no rotation: {}", rotation)));
    }

    return Ok((current, loops));
}

fn get_password(lines: Vec<String>) -> Result<(i64, i64, Vec<i64>)> {
    let mut zeroes: i64 = 0;
    let mut current: i64 = 50;
    let mut all_loops: i64 = 0;
    let mut results: Vec<i64> = vec![];

    for line in lines {
        let loops: i64;
        (current, loops) = step(current, &line, 100)?;
        results.push(current);
        if current == 0 {
            zeroes += 1;
        }
        all_loops += loops;
    }
    return Ok((zeroes, all_loops, results));
}

fn day1() -> Result<(i64, i64, Vec<i64>)> {
    let lines: Vec<String> = read_lines("src/data/input.txt")?;
    get_password(lines)
}

fn main() -> Result<()> {
    let (password, loops, _) = day1()?;
    println!("Hello from Day 1:");
    println!("First Password is {}!", password);
    println!("Second Password is {}!", loops+password);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() -> Result<()>  {
        let (result, _) = step(5, "L103", 10)?;
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
        let expected_loops = 6;

        let (password, loops, results) = get_password(lines)?;

        assert_eq!(password, expected_password);
        assert_eq!(loops+password, expected_loops);
        assert_eq!(results, expected_results);
        Ok(())
    }

    #[test]
    fn test_loops() -> Result<()>  {
        // this is from day 1 part 2
        let (_, loops, _) = get_password(vec!["R1000".to_string()])?;
        assert_eq!(loops, 10);

        // this is from reddit
        let lines: Vec<String> = vec![
            "R1000".to_string(),
            "L1000".to_string(),
            "L50".to_string(),
            "R1".to_string(),
            "L1".to_string(),
            "L1".to_string(),
            "R1".to_string(),
            "R100".to_string(),
            "R1".to_string()
        ];
        let (password, loops, _) = get_password(lines)?;
        assert_eq!(loops+password, 24);

        Ok(())
    }
}
