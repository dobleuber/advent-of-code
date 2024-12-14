use std::collections::BTreeMap;

#[derive(Debug)]
struct Field {
    width: usize,
    height: usize,
    regions: BTreeMap<String, Vec<(usize, usize)>>,
}

impl Field {
    fn parse(input: &str) -> Self {
        let mut regions: BTreeMap<String, Vec<(usize, usize)>> = BTreeMap::new();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                regions.entry(c.to_string()).or_default().push((x, y));
            });
        });

        Self {
            width,
            height,
            regions,
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let field = Field::parse(input);
    println!("{:?}", field);
    let total_price = 0;
    Ok(total_price.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("140", process(input)?);
        Ok(())
    }
    #[test]
    fn test_process_nested() -> miette::Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("772", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_big() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
