use crate::custom_error::AocError;
use itertools::Itertools;
use regex;
use std::collections::HashMap;

fn line_to_map(line: &str) -> HashMap<&str, u32> {
    let mut result: HashMap<&str, u32> = HashMap::new();
    let mut game_str: Vec<&str> = line.split(':').collect();
    let re = regex::Regex::new(r",|;").unwrap();
    let line_vec: Vec<&str> = re.split(game_str[1]).collect_vec();

    //Handle Game
    let game: Vec<&str> = game_str.remove(0).split(' ').collect();
    result.insert(game[0].trim(), game[1].trim().parse::<u32>().unwrap());


    // Handle rounds
    for item in line_vec {
        let inner_item = item.trim().split(' ').collect_vec();
        let key: &str = inner_item[1];
        let value: u32 = inner_item[0].trim().parse::<u32>().unwrap();

        if result.get(key).is_none() {
            result.insert(key.trim(), value);
        }
        else if result.get(key).unwrap() < &value {
            *result.get_mut(key).unwrap() = value;
        }
    }

    result
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let output = input.lines().map(|line| {
        let line_map = line_to_map(line);

        let power = line_map.get("red").unwrap() * line_map.get("blue").unwrap() * line_map.get("green").unwrap();
        power
    })
    .sum::<u32>();
    Ok(output)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";


        assert_eq!(2286, process(input)?);
        Ok(())
    }

    #[test]
    fn test_line2map() -> miette::Result<()> {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let output: HashMap<&str, u32> = line_to_map(input);
        let expected: HashMap<&str, u32> =
            HashMap::from([("Game", 1), ("blue", 6), ("red", 4), ("green", 2)]);

        for key in output.keys(){
            assert_eq!(output.get(key), expected.get(key))
        }

        Ok(())
    }
}
