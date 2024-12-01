#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left_values = vec![];
    let mut right_values = vec![];
    for line in input.lines() {
        let mut values = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        left_values.push(values.next().unwrap());
        right_values.push(values.next().unwrap());
    }

    let similarity: i32 = left_values.iter().map(|left| {
       let count = right_values.iter().filter(|&right| left == right).count();
       count as i32 * left
    }).sum();

    Ok(similarity.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
