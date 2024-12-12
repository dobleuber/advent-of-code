use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
struct Map {
    map: BTreeMap<(usize, usize), usize>,
    trailheads: Vec<(usize, usize)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut map = BTreeMap::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                map.insert((i, j), c.to_digit(10).unwrap() as usize);
            }
        }
        Self { map, trailheads: Vec::new() }
    }

    fn find_trailheads(&mut self) {
        for (i, j) in self.map.keys() {
            if self.map.get(&(*i, *j)).unwrap() == &0 {
                self.trailheads.push((*i, *j));
            }
        }
    }

    fn calc_trailheads(&self, i: usize, j: usize, trail: Vec<(usize, usize)>) -> Vec<Option<(usize, usize)>> {
        let steps = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
        ];
        let current = (i, j);
        let current_value = self.map.get(&current).unwrap();
        if *current_value == 9 {
            let &top = trail.last().unwrap();
            return vec![Some(top)];
        }

        let total: Vec<_> = steps.iter().flat_map(|&delta|{
            let next = (current.0 as i32 + delta.0, current.1 as i32 + delta.1);
            if let Some(next_value) = self.map.get(&(next.0 as usize, next.1 as usize)) {
                let mut trail = trail.clone();
                trail.push((next.0 as usize, next.1 as usize));
                match next_value.checked_sub(*current_value) {
                    Some(1) => self.calc_trailheads(next.0 as usize, next.1 as usize, trail),
                    _ => vec![None],
                }
            } else {
                vec![None]
            }
        }).collect();

        total
    }

    fn calc_map_score(&self) -> usize {
        let scores: Vec<_> = self.trailheads.iter().flat_map(|(i, j)| {
            self.calc_trailheads(*i, *j, vec![(*i, *j)])
        }).flatten().collect();

        scores.len()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dimensions = self.map.keys().fold((0, 0), |(max_i, max_j), (i, j)| {
            (max_i.max(*i), max_j.max(*j))
        });
        writeln!(f, "Dimensions: {:?}", dimensions)?;
        for i in 0..dimensions.0 {
            for j in 0..dimensions.1 {
                write!(f, "{}", self.map.get(&(i, j)).unwrap())?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Trailheads: {:?}", self.trailheads)?;
        Ok(())
    }
}



#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut map = Map::parse(input);
    map.find_trailheads();
    let score = map.calc_map_score();
    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
