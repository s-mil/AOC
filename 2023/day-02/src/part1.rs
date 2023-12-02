use crate::custom_error::AocError;
use std::collections::HashMap;

fn line_to_map(line:&str) -> HashMap<&str,u32> {

}
#[tracing::instrument]
pub fn process( input: &str, bounds: &HashMap<&str, u32>) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .map(|line| {
            let mut line_map = line_to_map(line);


        });

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let bounds: HashMap =  HashMap::from([("red",12), ("green",13), ("blue",14)]);

        assert_eq!(8, process(input, bounds)?);
        ok(())
    }
}






