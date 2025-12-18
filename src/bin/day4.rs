use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut input: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line: String = line?;
        input.push(line.chars().collect());
    }
    return Ok(input);
}

fn get_neighbor(input: &mut Vec<Vec<char>>, x: isize, y: isize) -> i64 {
    // early return when we are out of bounds
    if x < 0 || y < 0 {
        return 0;
    }

    input
        .get(x as usize)
        .and_then(|row| row.get(y as usize))
        .map(|&c| (c == '@') as i64)
        .unwrap_or(0)
}

fn find_reachable_rolls(mut input: &mut Vec<Vec<char>>) -> i64 {
    // let mut results: Vec<i64> = vec![];
    let mut sum: i64 = 0;
    let width = input[0].len();
    let depth = input.len();

    for i in 0..depth {
        for j in 0..width {
            let mut neighbors = 0;
            let current = input[i][j];
            if current == '@' {
                // check 8 surrounding positions
                neighbors += get_neighbor(&mut input, i as isize -1, j as isize -1);
                neighbors += get_neighbor(&mut input, i as isize -1, j as isize);
                neighbors += get_neighbor(&mut input, i as isize -1, j as isize +1);
                neighbors += get_neighbor(&mut input, i as isize, j as isize -1);
                neighbors += get_neighbor(&mut input, i as isize, j as isize +1);
                neighbors += get_neighbor(&mut input, i as isize +1, j as isize -1);
                neighbors += get_neighbor(&mut input, i as isize +1, j as isize);
                neighbors += get_neighbor(&mut input, i as isize +1, j as isize +1);

                if neighbors < 4 {
                    sum += 1;
                    input[i][j] = 'x';
                }
            }
        }
    }
    return sum;
}

fn loop_finding_rolls(mut input: Vec<Vec<char>>) -> i64 {
    let mut sum: Vec<i64> = vec![];
    loop {
        sum.push(find_reachable_rolls(&mut input));
        if sum.get(sum.len()-1) == Some(&0)  {
            break;
        }
    }
    sum.iter().sum()
}

fn day4() -> Result<i64> {
    let input: Vec<Vec<char>> = read_lines("src/data/input4.txt")?;
    Ok(loop_finding_rolls(input))
}

fn main() -> Result<()> {
    let amount = day4()?;
    println!("Hello from Day 4:");
    println!("The amount of reachable roles is {}!", amount);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4() -> Result<()>  {
        let input: Vec<Vec<char>> = read_lines("src/data/testInput4.txt")?;
        let amount = loop_finding_rolls(input);
        assert_eq!(amount, 43);
        Ok(())
    }
}
