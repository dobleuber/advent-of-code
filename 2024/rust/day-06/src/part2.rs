use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTile {
    Obstacle,
    Guard(Direction),
    Visited,
    NewObstacle,
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
    original_position: Position,
    map: BTreeMap<(usize, usize), MapTile>,
    position: Position,
    limits: (usize, usize),
    changes: Vec<(usize, usize, Direction)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 , direction: Direction::North }
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
                    .filter_map(move |(col, title)| match title {
                        '#' => Some(((row, col), MapTile::Obstacle)),
                        '.' => None,
                        '^'|'<'|'v'|'>' => {
                            Some(((row, col), MapTile::Guard(match title {
                                '^' => Direction::North,
                                '<' => Direction::West,
                                'v' => Direction::South,
                                '>' => Direction::East,
                                _ => unreachable!(),
                            })))
                        },
                        _ => panic!("unexpected character in map"),
                    })
            })
            .collect();

        let position = map.iter()
            .find(|(_, tile)| matches!(tile, MapTile::Guard(_)))
            .map(|((row, col), tile)| {
                Position {
                    x: *col as i32,
                    y: *row as i32,
                    direction: match tile {
                        MapTile::Guard(direction) => *direction,
                        _ => unreachable!(),
                    }
                }
            }).unwrap_or_default();
        
        let num_rows = input.lines().count();
        let num_cols = input.lines()
            .next()
            .map(|line| line.chars().count())
            .unwrap_or(0);

        Self {
            map,
            original_position: position,
            position,
            limits : (num_rows, num_cols),
            changes: Vec::new(),
        }
    }

    fn is_empty(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return true;
        }
        self.map.get(&(y as usize, x as usize)).map_or(true, |tile| matches!(tile, MapTile::Visited))
    }

    fn advance(&mut self) -> bool {
        let old_position = self.position;
        let mut is_change = false;

        loop {
            let (x, y) = match self.position.direction {
                Direction::North => (self.position.x, self.position.y - 1),
                Direction::East => (self.position.x + 1, self.position.y),
                Direction::South => (self.position.x, self.position.y + 1),
                Direction::West => (self.position.x - 1, self.position.y),
            };

            if self.is_empty(x, y) {
                if is_change {
                    if self.changes.contains(&(old_position.x as usize, old_position.y as usize, self.position.direction)) {
                        return true;
                    }
                    self.changes.push((old_position.x as usize, old_position.y as usize, self.position.direction));
                }
                self.position.set(x, y);
                self.map.entry((y as usize, x as usize)).and_modify(|title|
                    *title = MapTile::Guard(self.position.direction)
                ).or_insert(MapTile::Guard(self.position.direction));
                self.map.entry((old_position.y as usize, old_position.x as usize)).and_modify(|title|
                    *title = MapTile::Visited
                );
                break;
            }
            else {
                is_change = true;
                self.position.direction = match self.position.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
            }
        }

        false
    }

    fn solve(&mut self) -> (usize, bool) {
        let mut is_cycle = false;
        loop {
            if self.advance() {
                is_cycle = true;
                break;
            }
            let position = self.position;
            if position.x < 0 || position.y < 0 || position.x >= self.limits.1 as i32 || position.y >= self.limits.0 as i32 {
                break;
            }
        }
        let visited = self.map.iter()
            .filter(|(_, tile)| matches!(tile, MapTile::Visited))
            .count();
        (visited, is_cycle)
    }

    fn add_obstacle(&self, x: usize, y: usize) -> Option<Self> {
        let title = self.map.get(&(x, y));
        if (x, y) == (self.original_position.y as usize, self.original_position.x as usize) {
            return None;
        }
        match title {
            Some(MapTile::Obstacle) => None,
            _ => {
                let mut new_map = self.clone();
                new_map.map.retain(|_, tile| !matches!(tile, MapTile::Visited));
                new_map.map.insert((x, y), MapTile::NewObstacle);
                new_map.position = new_map.original_position;
                new_map.map.insert((new_map.position.y as usize, new_map.position.x as usize), MapTile::Guard(new_map.position.direction));
                new_map.changes.clear();
                Some(new_map)
            }   
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for x in 0..self.limits.0 {
            for y in 0..self.limits.1 {
                let tile_char = match self.map.get(&(x, y)) {
                    Some(MapTile::Obstacle) => '#',       
                    Some(MapTile::Guard(direction)) => match direction {
                        Direction::North => '^',          
                        Direction::South => 'v',          
                        Direction::East => '>',          
                        Direction::West => '<',           
                    },
                    Some(MapTile::Visited) => 'x',        
                    Some(MapTile::NewObstacle) => 'O',   
                    None => '.',                          
                };
                write!(f, "{}", tile_char)?; 
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut map = Map::parse(input);
    map.solve();
    let mut alternatives =  0;



    for x in 0..map.limits.1 {
        for y in 0..map.limits.0 {
            if let Some(mut try_map) = map.add_obstacle(y, x) {
                let (_, is_cycle) = try_map.solve();
                if is_cycle {
                    // println!("try: {}", try_map);
                    alternatives += 1;
                }
            }
        }
    }

    Ok(alternatives.to_string())
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
