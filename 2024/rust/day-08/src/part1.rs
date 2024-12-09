use std::collections::{BTreeMap, HashSet};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Antinode(usize, usize);

fn find_antinodes(antena_locations: Vec<(usize, usize)>) -> Vec<(i32, i32)>{
    antena_locations.iter().combinations(2).flat_map(|c| {
        let (a, b) = (c[0], c[1]);
        let diff_x = a.0 as i32 - b.0 as i32;
        let diff_y = a.1 as i32 - b.1 as i32;

        let first_antinode = (a.0 as i32 + diff_x, a.1 as i32 + diff_y);
        let second_antinode = (b.0 as i32 - diff_x, b.1 as i32 - diff_y);
        vec![first_antinode, second_antinode]
    }).collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut antena_locations: BTreeMap<char, Vec<(usize, usize)>> = BTreeMap::new();
    let mut antinodes: HashSet<Antinode> = HashSet::new();
    let limits = (input.lines().count(), input.lines().next().unwrap().chars().count());

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                antena_locations.entry(c).or_default().push((x, y));
            }
        });
    });

    for (_, locations) in antena_locations.iter() {
        let antinodes_for_antena = find_antinodes(locations.clone());
        let an_hs: HashSet<Antinode> = antinodes_for_antena
            .into_iter()
            .filter(|&an| {
                an.0 >= 0 && an.0 < limits.1 as i32 && an.1 >= 0 && an.1 < limits.0 as i32
            }).map(|an| Antinode(an.0 as usize, an.1 as usize)).collect();
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
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
