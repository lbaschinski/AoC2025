use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use aoc2025::parse_string_to_i64;

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn group_problems(input: Vec<String>) -> Result<(Vec<Vec<i64>>, Vec<String>)> {
    let mut problems: Vec<Vec<i64>> = vec![];
    let first_row: Vec<i64> = input[0].split_whitespace().map(parse_string_to_i64).collect::<Result<Vec<i64>, _>>()?;
    for elem in first_row {
        problems.push(vec![elem]);
    }
    for i in 1..input.len()-1 {
        let row: Vec<i64> = input[i].split_whitespace().map(parse_string_to_i64).collect::<Result<Vec<i64>, _>>()?;
        for j in 0..row.len() {
            problems[j].push(row[j]);
        }
    }
    let operands: Vec<String> = input[input.len()-1].split_whitespace().map(ToString::to_string).collect::<Vec<String>>();
    Ok((problems, operands))
}

fn calculate_problems(problems: Vec<Vec<i64>>, operands: Vec<String>) -> Result<Vec<i64>> {
    let mut solutions: Vec<i64> = vec![];
    for i in 0..operands.len() {
        let solution = match operands[i].as_str() {
            "+" => {
                problems[i].iter().sum()
            }
            "*" => {
                problems[i].iter().product()
            }
            _ => { return Err(anyhow!("operand is wrong: {}", operands[i])); }
        };
        solutions.push(solution);
    }
    Ok(solutions)
}

fn get_result(input: Vec<String>) -> Result<Vec<i64>> {
    let mut _results: Vec<i64> = vec![];
    let (problems, operands) = group_problems(input)?;
    let solutions = calculate_problems(problems, operands)?;
    return Ok(solutions);
}

fn day_6() -> Result<Vec<i64>> {
    let input: Vec<String> = read_lines("src/data/input6.txt")?;
    let solutions = get_result(input)?;
    return Ok(solutions);
}

fn main() -> Result<()> {
    let solutions = day_6()?;
    println!("Hello from Day 6:");
    println!("The result is {}!", solutions.iter().sum::<i64>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6() -> Result<()>  {
        let input: Vec<String> = read_lines("src/data/testInput6.txt")?;
        let expected_solutions: Vec<i64> = vec![33210, 490, 4243455, 401];
        let expected_sum: i64 = 4277556;

        let solutions = get_result(input)?;
        assert_eq!(solutions, expected_solutions);
        assert_eq!(solutions.iter().sum::<i64>(), expected_sum);

        Ok(())
    }
}
