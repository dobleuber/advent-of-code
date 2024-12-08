use itertools::{Itertools, izip};

#[derive(Debug)]
struct Equation {
    operands: Vec<i64>,
    result: i64,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Equation {
    fn is_valid(&self) -> bool {
        let operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];
        let total_operators = self.operands.len() - 1;
        // Crear iteradores para el producto cartesiano
        let operator_ranges = std::iter::repeat(&operators).take(total_operators);

        // Generar combinaciones con repetición como un producto cartesiano
        let combinations = operator_ranges
            .map(|ops| ops.iter()) 
            .multi_cartesian_product(); 

        // Imprimir las combinaciones generadas
        for combination in combinations {
            let mut result = self.operands[0];
            for (operator, operand) in izip!(combination, self.operands.iter().skip(1)) {
                match operator {
                    Operator::Add => result += operand,
                    Operator::Multiply => result *= operand,
                    Operator::Concatenate => {
                        let mut result_str = result.to_string();
                        result_str.push_str(&operand.to_string());
                        result = result_str.parse::<i64>().unwrap();
                    }
                }
            }

            if result == self.result {
                return true;
            }
        }
        
        false
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().expect("Result must be valid").parse::<i64>().expect("Result must be a number");
            let operands = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|operand| operand.parse::<i64>().unwrap())
                .collect();
            Equation { operands, result }
        })
        .collect();

    let mut total = 0;

    for equation in equations.iter() {
        if equation.is_valid() {
            println!("{:?} is valid", equation);
            total += equation.result;
        }
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_292() -> miette::Result<()> {
        let input = "190: 10 19
292: 11 6 16 20";
        assert_eq!("482", process(input)?);
        Ok(())
    }
}
