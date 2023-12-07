use crate::custom_error::AocError;

fn get_matches(card: &str) -> u32 {
    let clean_card: &str = card.split(':').last().unwrap().trim();
    let winning_numbers: Vec<u32> = clean_card.split('|').last().unwrap().split(' ').collect::vec<&str>().parse::<u32>().collect();
    let play_numbers: Vec<u32> = clean_card.split('|').collect().first() ;

    2
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let base: u32 = 2;
    let output: String = _input
    .lines()
    .map(|line|
      base.pow(get_matches(line))
    )
    .sum::<u32>().to_string();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
