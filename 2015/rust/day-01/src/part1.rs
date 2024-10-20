use crate::custom_error::AocError;




#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> miette::Result<()> {
        let input = "(())";
        assert_eq!("0", process(input)?);
        Ok(())
    }
    #[test]
    fn test_2() -> miette::Result<()> {
        let input = "(((";
        assert_eq!("3", process(input)?);
        Ok(())
    }

    #[test]
    fn test_3() -> miette::Result<()> {
        let input = ")))";
        assert_eq!("-3", process(input)?);
        Ok(())
    }

}
