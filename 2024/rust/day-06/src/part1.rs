use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTile {
    Obstacle,
    Empty,
    Guard(Direction),
    Visited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Map {
    map: BTreeMap<(usize, usize), MapTile>,
    position: Position,
    limits: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::North,
        }
    }
}

impl Position {
    fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

impl Map {
    fn parse(input: &str) -> Self {
        let map: BTreeMap<(usize, usize), _> = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, title)| match title {
                        '#' => ((row, col), MapTile::Obstacle),
                        '.' => ((row, col), MapTile::Empty),
                        '^' | '<' | 'v' | '>' => (
                            (row, col),
                            MapTile::Guard(match title {
                                '^' => Direction::North,
                                '<' => Direction::West,
                                'v' => Direction::South,
                                '>' => Direction::East,
                                _ => unreachable!(),
                            }),
                        ),
                        _ => panic!("unexpected character in map"),
                    })
            })
            .collect();

        let position = map
            .iter()
            .find(|(_, tile)| matches!(tile, MapTile::Guard(_)))
            .map(|((row, col), tile)| Position {
                x: *col as i32,
                y: *row as i32,
                direction: match tile {
                    MapTile::Guard(direction) => *direction,
                    _ => unreachable!(),
                },
            })
            .unwrap_or_default();

        let num_rows = input.lines().count();
        let num_cols = input
            .lines()
            .next()
            .map(|line| line.chars().count())
            .unwrap_or(0);

        Self {
            map,
            position,
            limits: (num_rows, num_cols),
        }
    }

    fn is_empty(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return true;
        }
        self.map
            .get(&(y as usize, x as usize))
            .map_or(true, |tile| {
                matches!(tile, MapTile::Empty | MapTile::Visited)
            })
    }

    fn advance(&mut self) {
        let old_position = self.position;

        loop {
            let (x, y) = match self.position.direction {
                Direction::North => (self.position.x, self.position.y - 1),
                Direction::East => (self.position.x + 1, self.position.y),
                Direction::South => (self.position.x, self.position.y + 1),
                Direction::West => (self.position.x - 1, self.position.y),
            };

            if self.is_empty(x, y) {
                self.position.set(x, y);
                self.map
                    .entry((y as usize, x as usize))
                    .and_modify(|title| *title = MapTile::Guard(self.position.direction));
                self.map
                    .entry((old_position.y as usize, old_position.x as usize))
                    .and_modify(|title| *title = MapTile::Visited);
                break;
            } else {
                self.position.direction = match self.position.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
            }
        }
    }

    fn solve(&mut self) -> usize {
        let mut steps = 0;
        loop {
            self.advance();
            steps += 1;
            let position = self.position;
            if position.x < 0
                || position.y < 0
                || position.x >= LIMITS.1 as i32
                || position.y >= LIMITS.0 as i32
            {
                break;
            }
        }
        println!("steps: {}", steps);
        self.map
            .iter()
            .filter(|(_, tile)| matches!(tile, MapTile::Visited))
            .count()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut map = Map::parse(input);
    let steps = map.solve();

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
