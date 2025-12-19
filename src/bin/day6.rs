use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use aoc2025::parse_string_to_i64;

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn group_problems_part1(input: Vec<String>) -> Result<(Vec<Vec<i64>>, Vec<String>)> {
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

fn common_space_indices(grid: Vec<Vec<char>>) -> Vec<usize> {
    let cols = grid[0].len();

    (0..cols)
        .filter(|&i| grid.iter().all(|row| row[i] == ' '))
        .collect()
}

// this does a lot of transformation on at a time and can definitely be improved by a lot
// but it gets the job done and it's late soooo... take with care :D
fn group_problems_part2(input: Vec<String>) -> Result<(Vec<Vec<i64>>, Vec<String>)> {
    // first format everything into a list of list of chars (to not loose positioning)
    let mut char_input: Vec<Vec<char>> = vec![];
    for i in 0..input.len()-1 {
        char_input.push(input[i].chars().collect());
    }
    let problem_boundaries = common_space_indices(char_input.clone());

    // then group the numbers that belong to one problem together
    let mut problems: Vec<Vec<Vec<char>>> = vec![];
    // [3, 7, 11] => ..3, 4..7, 8..11, 12..
    let problem_amount: usize = problem_boundaries.len() + 1;
    // initialize with empty vectors for easy push
    for j in 0..problem_amount {
        problems.push(vec![]);
        for _ in 0..char_input.len() {
            problems[j].push(vec![]);
        }
    }
    let first: usize = problem_boundaries[0];
    let last: usize = problem_boundaries[problem_boundaries.len() - 1] + 1; // get rif of the " "
    for i in 0..char_input.len() {
        problems[0][i] = char_input[i][..first].to_vec();
        for j in 1..problem_amount - 1 {
            let start = problem_boundaries[j-1] + 1; // get rid of the " " again
            let end = problem_boundaries[j];
            problems[j][i] = char_input[i][start..end].to_vec();
        }
        problems[problem_amount-1][i] = char_input[i][last..].to_vec();
    }

    // make everything "square", so same hight and length, so that the index swap later is possible
    // this was not necessary for the test (since it was 3x3), but for the real data it is (2x3, 4x3)
    let mut squared_problems: Vec<Vec<Vec<char>>> = vec![];
    let problem_height: usize = input.len() - 1; // minus operand line
    for i in 0..problems.len() {
        let inner = &problems[i];
        squared_problems.push(vec![]);
        for j in 0..inner.len() {
            let size = inner[j].len();
            squared_problems[i].push(vec![]);
            for k in 0..size {
                squared_problems[i][j].push(problems[i][j][k]);
            }
            // widen the matrix if it currently has less width than hight
            for _ in size..problem_height {
                squared_problems[i][j].push(' ');
            }
        }
    }

    // now change the orientation since we read right to left and top to bottom
    let mut ordered_problems: Vec<Vec<Vec<char>>> = vec![];
    for i in 0..squared_problems.len() {
        let inner = &squared_problems[i];
        ordered_problems.push(vec![]);
        for j in 0..inner.len() {
            let size = inner[j].len();
            ordered_problems[i].push(vec![]);
            for k in 0..size {
                ordered_problems[i][j].push(' ');
                ordered_problems[i][j][k] = squared_problems[i][k][j]; // this only works, if problems have the same height and width
            }
        }
    }

    let operands: Vec<String> = input[input.len()-1].split_whitespace().map(ToString::to_string).collect::<Vec<String>>();

    // now format this back to Vec<Vec<i64>> for further calculations
    // replace completely empty number slots with either 0 or 1 depending on the operand used
    let mut numbered_problems: Vec<Vec<i64>> = vec![];
    for i in 0..ordered_problems.len() {
        let inner = &ordered_problems[i];
        numbered_problems.push(vec![]);
        for j in 0..inner.len() {
            let string: String = inner[j].clone().into_iter().collect::<String>();
            let num: i64 = (
                if string.trim().is_empty() {
                    match operands[i].as_str() {
                        "+" => {
                            Ok(0)
                        }
                        "*" => {
                            Ok(1)
                        }
                        _ => { return Err(anyhow!("operand is wrong: {}", operands[i])); }
                    }
                } else {
                    parse_string_to_i64(string.as_str())
                }
            )?;
            numbered_problems[i].push(num);
        }
    }

    Ok((numbered_problems, operands))
}

fn calculate_problems(problems: Vec<Vec<i64>>, operands: Vec<String>) -> Result<Vec<i64>> {
    let mut solutions: Vec<i64> = vec![];
    if problems.len() != operands.len() {
        return Err(anyhow!("Operands ({}) and problem ({}) length must be the same!", operands.len(), problems.len()));
    }
    for i in 0..problems.len() {
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

fn get_result(input: Vec<String>) -> Result<(Vec<i64>, Vec<i64>)> {
    let mut _results: Vec<i64> = vec![];
    let (problems1, operands1) = group_problems_part1(input.clone())?;
    let (problems2, operands2) = group_problems_part2(input)?;
    let solutions1 = calculate_problems(problems1, operands1)?;
    let solutions2 = calculate_problems(problems2, operands2)?;
    return Ok((solutions1, solutions2));
}

fn day_6() -> Result<(Vec<i64>, Vec<i64>)> {
    let input: Vec<String> = read_lines("src/data/input6.txt")?;
    let (solutions1, solutions2) = get_result(input)?;
    return Ok((solutions1, solutions2));
}

fn main() -> Result<()> {
    let (solutions1, solutions2) = day_6()?;
    println!("Hello from Day 6:");
    println!("The result of part 1 is {}!", solutions1.iter().sum::<i64>());
    println!("The result of part 2 is {}!", solutions2.iter().sum::<i64>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6() -> Result<()>  {
        let input: Vec<String> = read_lines("src/data/testInput6.txt")?;
        let expected_solutions1: Vec<i64> = vec![33210, 490, 4243455, 401];
        let expected_sum1: i64 = 4277556;
        let mut expected_solutions2: Vec<i64> = vec![1058, 3253600, 625, 8544];
        expected_solutions2.sort();
        let expected_sum2: i64 = 3263827;

        let (solutions1, mut solutions2) = get_result(input)?;
        solutions2.sort();

        assert_eq!(solutions1, expected_solutions1);
        assert_eq!(solutions1.iter().sum::<i64>(), expected_sum1);
        assert_eq!(solutions2, expected_solutions2);
        assert_eq!(solutions2.iter().sum::<i64>(), expected_sum2);

        Ok(())
    }
}
