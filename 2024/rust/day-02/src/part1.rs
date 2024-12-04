#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines = input.lines();
    let reports:Vec<_> = lines
        .map(|line| line.split_whitespace()
            .filter_map(|num|num.parse::<i32>().ok())
            .collect::<Vec<i32>>()
        ).collect();

    let mut valids = 0;
    for report in reports {
        let mut asc: Option<bool> = None;
        let mut is_valid = true;
        for w in report.windows(2) {
            let (a, b) = (w[0], w[1]);
            let diff = a - b;
            let abs_diff = diff.abs();
            match abs_diff {
                1..4 => {
                    if let Some(asc) = asc {
                        if asc != (diff > 0) {
                            is_valid = false;
                            break;
                        }
                    } else {
                        asc = Some(diff > 0);
                    }
                },
                _ => {
                    is_valid = false;
                    break;
                }
            }
        };

        if is_valid {
            valids += 1;
        }
    }
    Ok(valids.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
