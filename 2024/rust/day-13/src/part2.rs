use glam::f64::DVec2;
use glam::f64::DMat2;
use regex::Regex;

#[derive(Debug, Default, Clone)]
struct EquationSystem {
    results: DVec2,
    matrix: DMat2,
    solution: Option<DVec2>,
}

fn is_almost_integer(value: f64) -> bool {
    const EPSILON: f64 = 0.0001;
    (value - value.round()).abs() < EPSILON
}

impl EquationSystem {
    fn parse(input: &str) -> Vec<Self> {
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        let re_button = Regex::new(r"Button (?<name>\w+): X\+(?<x>\d+), Y\+(?<y>\d+)").expect("Invalid regex");
        let re_prize = Regex::new(r"X=(?<x>\d+), Y=(?<y>\d+)").expect("Invalid regex");
        let mut eq = EquationSystem::default();
        let mut cols: Vec<DVec2> = Vec::new();
        let mut eqs: Vec<EquationSystem> = Vec::new();
        let mut is_completed = false;
        for line in input.lines() {
            if let Some(caps) = re_button.captures(line) {
                let x = caps["x"].parse::<f64>().unwrap();
                let y = caps["y"].parse::<f64>().unwrap();
                cols.push(DVec2::new(x, y));
            }

            if let Some(caps) = re_prize.captures(line) {
                let x = caps["x"].parse::<f64>().unwrap();
                let y = caps["y"].parse::<f64>().unwrap();
                eq.results = DVec2::new(x + 10000000000000.0, y + 10000000000000.0);
                is_completed = true;
            }

            if is_completed {
                eq.matrix = DMat2::from_cols(cols[0], cols[1]);
                eqs.push(eq.clone());
                eq = EquationSystem::default();
                is_completed = false;
                cols = Vec::new();
            }
        }
        eqs
    }

    fn solve(&mut self) -> DVec2 {
        self.solution = Some(self.matrix.inverse() * self.results);
        self.solution.unwrap()
    }

    fn is_valid(&self) -> bool {
        let solution = self.solution.unwrap().fract();
        is_almost_integer(solution.x) && is_almost_integer(solution.y)
    }

    fn price(&self) -> Option<u64> {
        if !self.is_valid() {
            return None;
        }
        self.solution.map(|s| ((s.x * 3.0) + s.y).round() as u64)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut eqs = EquationSystem::parse(input);
    let total: u64 = eqs.iter_mut().filter_map(|eq| {
        eq.solve();
        eq.price()
    }).sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_no_solution() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        assert_eq!("0", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_with_solution() -> miette::Result<()> {
        let input = "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        assert_eq!("459236326669", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_full_example() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("875318608908", process(input)?);
        Ok(())
    }
}
