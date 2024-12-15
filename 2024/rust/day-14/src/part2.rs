use regex::Regex;
use std::{collections::HashMap, fmt};

#[derive(Debug, Default, Clone)]
struct RobotState {
    position: glam::IVec2,
    velocity: glam::IVec2,
}

struct Map {
    locations: HashMap<glam::IVec2, Vec<RobotState>>,
}

const LIMITS: glam::IVec2 = glam::IVec2::new(101, 103);

impl RobotState {
    fn parse(input: &str) -> Option<Self> {
        // p=0,4 v=3,-3
        let re = Regex::new(r"p=(?<x>-?\d+),(?<y>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)")
            .expect("Invalid regex");
        if let Some(caps) = re.captures(input) {
            let x = caps["x"].parse::<i32>().unwrap();
            let y = caps["y"].parse::<i32>().unwrap();
            let vx = caps["vx"].parse::<i32>().unwrap();
            let vy = caps["vy"].parse::<i32>().unwrap();
            Some(Self {
                position: glam::IVec2::new(x, y),
                velocity: glam::IVec2::new(vx, vy),
            })
        } else {
            None
        }
    }

    fn step(&mut self) {
        self.position = (self.position + self.velocity).rem_euclid(LIMITS);
    }
}

impl Map {
    fn create(robots: Vec<RobotState>) -> Self {
        let mut map = Self {
            locations: HashMap::new(),
        };

        for robot in robots {
            map.locations.entry(robot.position).or_default().push(robot);
        }

        map
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();
        writeln!(f, "Map:\n")?;
        for y in 0..LIMITS.y {
            for x in 0..LIMITS.x {
                let position = glam::IVec2::new(x, y);
                let robots = self.locations.get(&position);
                if let Some(robots) = robots {
                    if robots.is_empty() {
                        buffer.push('.');
                    } else if robots.len() == 1 {
                        buffer.push('X');
                    } else {
                        buffer.push('O');
                    }
                } else {
                    buffer.push('.');
                }
            }
            writeln!(f, "{}", buffer)?;
            buffer.clear();
        }
        Ok(())
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut robots = input
        .lines()
        .filter_map(RobotState::parse)
        .collect::<Vec<_>>();

    let mut i = 0;
    loop {
        robots.iter_mut().for_each(|r| r.step());
        i += 1;
        let map = Map::create(robots.clone());
        if map.locations.values().all(|v| v.len() == 1) {
            println!("{}", map);
            break;
        }
    }

    Ok(i.to_string())
}
