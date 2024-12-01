use tracing::debug;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let mut left_values = vec![];
    let mut right_values = vec![];
    for line in input.lines() {
        let mut values = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        left_values.push(values.next().unwrap());
        right_values.push(values.next().unwrap());
    }

    left_values.sort();
    right_values.sort();
    let distance = std::iter::zip(left_values, right_values)
        .fold(0, |acc, (left, right)| {
            let diff = right - left;
            debug!("{} - {} = {}", right, left, diff);
            acc + diff.abs()
        });

    Ok(distance)
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
        assert_eq!(11, process(input)?);
        Ok(())
    }
}
