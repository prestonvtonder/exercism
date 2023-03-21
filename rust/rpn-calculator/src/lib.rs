#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Divide,
    Multiply,
    Subtract,
    Value(i32),
}

impl CalculatorInput {
    fn calc(&self, a: i32, b: i32) -> i32 {
        match self {
            CalculatorInput::Add => a + b,
            CalculatorInput::Divide => a / b,
            CalculatorInput::Multiply => a * b,
            CalculatorInput::Subtract => a - b,
            _ => unreachable!(),
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    // any even number is invalid, since we are only dealing with binary operators
    if inputs.len() % 2 == 0 {
        None
    } else {
        let mut stack = Vec::new();
        for i in inputs {
            match i {
                CalculatorInput::Value(v) => stack.push(v.clone()),
                _ => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(i.calc(a, b));
                }
            }
        }
        stack.pop()
    }
}
