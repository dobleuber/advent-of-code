use regex::Regex;

#[derive(Debug, Default, Clone)]
struct RobotState {
    position: glam::IVec2,
    velocity: glam::IVec2,
}

const LIMITS: glam::IVec2 = glam::IVec2::new(101, 103);
const TOTAL_STEPS: u32 = 100;

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

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut robots = input
        .lines()
        .filter_map(RobotState::parse)
        .collect::<Vec<_>>();

    // println!("robots: {:#?}", robots);

    for _ in 0..TOTAL_STEPS {
        robots.iter_mut().for_each(|r| r.step());
    }

    let mut quandrants: Vec<Vec<RobotState>> = vec![vec![]; 4];
    let half_limits = LIMITS / 2;

    for robot in robots
        .into_iter()
        .filter(|r| r.position.x != half_limits.x && r.position.y != half_limits.y)
    {
        let quadrant_index = if robot.position.x < half_limits.x {
            if robot.position.y < half_limits.y {
                0
            } else {
                1
            }
        } else if robot.position.y < half_limits.y {
            2
        } else {
            3
        };
        quandrants[quadrant_index].push(robot);
    }

    let safety_factor = quandrants.iter().fold(1, |acc, q| acc * q.len());

    // println!("robots_in_quadrant: {:#?}", quandrants);
    Ok(safety_factor.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}
