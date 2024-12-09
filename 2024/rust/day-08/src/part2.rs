use std::collections::{BTreeMap, HashSet};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Antinode(usize, usize);

fn find_antinodes(antena_locations: Vec<(usize, usize)>, limits: (usize, usize)) -> Vec<(i32, i32)>{
    antena_locations.iter().combinations(2).flat_map(|c| {
        let (a, b) = (c[0], c[1]);
        let diff_x = a.0 as i32 - b.0 as i32;
        let diff_y = a.1 as i32 - b.1 as i32;

        let firsts_antinode: Vec<_> = (0..)
            .map(|i| (a.0 as i32 + (i * diff_x), a.1 as i32 + (i * diff_y)))
            .take_while(|(x, y)| *x >= 0 && *x < limits.0 as i32 && *y >= 0 && *y < limits.1 as i32)
            .collect();
        let seconds_antinode: Vec<_>= (0..)
            .map(|i| (b.0 as i32 - (i * diff_x), b.1 as i32 - (i * diff_y)))
            .take_while(|(x, y)| *x >= 0 && *x < limits.0 as i32 && *y >= 0 && *y < limits.1 as i32)
            .collect();
        firsts_antinode.into_iter().chain(seconds_antinode)
    }).collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut antena_locations: BTreeMap<char, Vec<(usize, usize)>> = BTreeMap::new();
    let mut antinodes: HashSet<Antinode> = HashSet::new();
    let limits = (input.lines().count(), input.lines().next().unwrap().chars().count());

    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c != '.' {
                antena_locations.entry(c).or_default().push((x, y));
            }
        });
    });

    for (_, locations) in antena_locations.iter() {
        let antinodes_for_antena = find_antinodes(locations.clone(), limits);
        let an_hs: HashSet<Antinode> = antinodes_for_antena
            .into_iter()
            .map(|an| Antinode(an.0 as usize, an.1 as usize)).collect();
        antinodes.extend(an_hs);
    }

    Ok(antinodes.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
