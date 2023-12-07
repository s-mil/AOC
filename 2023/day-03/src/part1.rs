use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str,) -> miette::Result<u32, AocError> {
    let : u32 = _input
        .lines()
        .map
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
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
