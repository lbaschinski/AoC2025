use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn step(bank: String) -> Result<i64> {
    let battery_amount = bank.chars().count();
    let batteries: Vec<char> = bank.chars().collect();
    let mut tens: i8 = 0;
    let mut ones: i8 = 0;

    for index in 0..battery_amount-1 {
        let new_tens = batteries.get(index).ok_or(anyhow!("index issues"))?.to_string().trim().parse::<i8>()?;
        let new_ones = batteries.get(index+1).ok_or(anyhow!("index issues"))?.to_string().trim().parse::<i8>()?;
        if new_tens > tens {
            tens = new_tens;
            // reset this since we can't use a "high" number left of the highest tenner number for the ones
            ones = 0;
        }
        if new_ones > ones {
            ones = new_ones;
        }
    }
    let joltage = format!("{}{}", tens, ones).parse::<i64>()?;
    return Ok(joltage);
}

fn get_joltages(lines: Vec<String>) -> Result<(Vec<i64>, i64)> {
    let mut results: Vec<i64> = vec![];
    let mut sum: i64 = 0;

    for line in lines {
        let joltage = step(line)?;
        results.push(joltage);
        sum += joltage;
    }
    return Ok((results, sum));
}

fn day3() -> Result<(Vec<i64>, i64)> {
    let lines: Vec<String> = read_lines("src/data/input3.txt")?;
    get_joltages(lines)
}

fn main() -> Result<()> {
    let (_, sum) = day3()?;
    println!("Hello from Day 3:");
    println!("Joltages sum is {}!", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joltages() -> Result<()>  {
        let lines: Vec<String> = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string()
        ];
        let expected_results = vec![98, 89, 78, 92];
        let expected_sum = 357;

        let (results, sum) = get_joltages(lines)?;

        assert_eq!(sum, expected_sum);
        assert_eq!(results, expected_results);
        Ok(())
    }
}
