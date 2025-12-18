use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn step(bank: String, battery_amount: usize) -> Result<i64> {
    let bank_size = bank.chars().count();
    let batteries: Vec<char> = bank.chars().collect();
    let mut results: Vec<i64> = vec![0; battery_amount];
    let mut current_index = 0;

    for place in 0..battery_amount {
        // calculate new "end": we need to reduce the max_index by the battery_amount
        // (leave enough space for all the other places), but adjust this number
        // by the amount of places we already filled up
        // so for an 8 digit long number, where we want to find the highest 4-digit number:
        // 0..5 (8-4+1) => x..6 (8-4+2) => y..7 (8-4+3) => z..8 (8-4+4)
        let max_index = bank_size-battery_amount+(place+1);
        for index in current_index..max_index {
            // convert char from Vector to string, so that we can parse to integer
            let new_value = batteries.get(index).ok_or(anyhow!("index issues"))?.to_string().trim().parse::<i64>()?;
            if new_value > results[place] {
                results[place] = new_value;
                current_index = index+1; // next place should only start looking at 1 index after this place
            }
        }
    }
    // flatten: convert the Vector of integers to a String, so that we can convert this to an integer again^^
    let joltage = results.iter().map(ToString::to_string).collect::<String>().parse::<i64>()?;
    return Ok(joltage);
}

fn get_joltages(lines: Vec<String>, battery_amount: usize) -> Result<(Vec<i64>, i64)> {
    let mut results: Vec<i64> = vec![];
    let mut sum: i64 = 0;

    for line in lines {
        let joltage = step(line, battery_amount)?;
        results.push(joltage);
        sum += joltage;
    }
    return Ok((results, sum));
}

fn day3() -> Result<(i64, i64)> {
    let lines: Vec<String> = read_lines("src/data/input3.txt")?;
    let (_, joltage2) = get_joltages(lines.clone(), 2)?;
    let (_, joltage12) = get_joltages(lines, 12)?;
    return Ok((joltage2, joltage12));
}

fn main() -> Result<()> {
    let (sum2, sum12) = day3()?;
    println!("Hello from Day 3:");
    println!("Joltages sum at 2 is {}!", sum2);
    println!("Joltages sum at 12 is {}!", sum12);
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
        let expected_sum_part1 = 357;
        let expected_sum_part2 = 3121910778619;

        let (results, sum) = get_joltages(lines.clone(), 2)?;
        assert_eq!(results, expected_results);
        assert_eq!(sum, expected_sum_part1);

        let (_, sum) = get_joltages(lines, 12)?;
        assert_eq!(sum, expected_sum_part2);

        Ok(())
    }
}
