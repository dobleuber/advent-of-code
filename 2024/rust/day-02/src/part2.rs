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
        let mut window = report.iter();
        let mut a = window.next().unwrap();
        let mut has_error = false;
        let mut prev_asc: Option<bool> = None;
        let mut prev_a = a;
        for b in window {
            let diff = a - b;
            let abs_diff = diff.abs();
            // dbg!(a, b, abs_diff, has_error, is_valid);
            match (abs_diff, has_error) {
                (1..4, _) => {
                    if asc.is_some() {
                        if asc != Some(diff > 0) {
                            if has_error {
                                is_valid = false;
                                break;
                            } else {
                                has_error = true;
                                asc = prev_asc;
                                a = prev_a;
                                continue;
                            }
                        }
                    } else {
                        prev_asc = asc;
                        asc = Some(diff > 0);
                    }
                },
                (_, false) => {
                    has_error = true;
                    continue;
                },
                _ => {
                    is_valid = false;
                    break;
                }
            }

            prev_a = a;
            a = b;
        }

        if is_valid {
            valids += 1;
        } else {
            println!("{:?}", report);
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
1 3 6 7 9
";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_ds() -> miette::Result<()> {
        let input = "37 36 39 40 41 44 45
54 57 54 55 55
";
        assert_eq!("1", process(input)?);
        Ok(())
    }
}
