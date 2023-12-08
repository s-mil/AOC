use std::collections::BTreeMap;

use crate::custom_error::AocError;

#[derive(Debug)]
enum Value {
    Empty,
    Number(u32),
    Symbol(char),
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                (
                    (y as i32, x as i32),
                    match character {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Number(c.to_digit(10).expect("should be a number"))
                        }
                        c => Value::Symbol(c),
                    },
                )
                    },
                )
        })
        .collect::<BTreeMap<(i32, i32), Value>>();
    let mut numbers: Vec<Vec<((i32,i32), u32)>> = vec![];

    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numvers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num
                }
            }
        }
    }

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
