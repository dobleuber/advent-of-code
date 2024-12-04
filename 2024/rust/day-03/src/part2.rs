use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let main_re = Regex::new(r"do(n't)?\(\)").expect("Invalid regex");
    let mut enabled = true;
    let mut start = 0;
    let mut end = 0;
    let mut result:i32 = main_re.find_iter(input).map(|m| {
        match m.as_str() {
            "do()" => {
                if enabled {
                    end = m.start();
                    let fragment = &input[start..end];
                    sum_fragment(fragment)
                } else {
                    start = m.end();
                    enabled = true;
                    0
                }
            },
            "don't()" => {
                if enabled {
                    end = m.start();
                    let fragment = &input[start..end];
                    enabled = false;
                    sum_fragment(fragment)
                } else {
                    0
                }
            },
            _ => 0
        }
    }).sum();

    if enabled {
        result += sum_fragment(&input[start..]);
    } 
    
    Ok(result.to_string())
}

fn sum_fragment(fragment: &str) -> i32 {
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").expect("Invalid regex");
     re.captures_iter(fragment).fold(0, |acc, c| {
            c["first"].parse::<i32>().unwrap() * c["second"].parse::<i32>().unwrap() + acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
