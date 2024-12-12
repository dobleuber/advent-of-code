#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut stones: Vec<_> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        let changes: Vec<_> = stones
            .iter()
            .flat_map(|&s| match s {
                0 => vec![1],
                x if x.to_string().len() % 2 == 0 => {
                    let stone: String = x.to_string();
                    let half: usize = stone.len() / 2;
                    let left: usize = stone[..half].to_string().parse().unwrap();
                    let right: usize = stone[half..].to_string().parse().unwrap();
                    vec![left as u64, right as u64]
                }
                x => vec![x * 2024],
            })
            .collect();

        // println!("{:?}", changes);

        stones = changes;
    }

    Ok(stones.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
