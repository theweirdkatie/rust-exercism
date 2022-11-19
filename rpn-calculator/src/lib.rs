#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut _stack: Vec<i32> = vec![];
    if inputs.len() % 2 == 0 {
        return None;
    } else {
        for com in inputs.iter() {
            match com {
                CalculatorInput::Add => {
                    if inputs.len() >= 2 {
                        let (y, x) = (_stack.pop().unwrap(), _stack.pop().unwrap());
                        _stack.push(x + y);
                    } else {
                        return None;
                    }
                }
                CalculatorInput::Subtract => {
                    if inputs.len() >= 2 {
                        let (y, x) = (_stack.pop().unwrap(), _stack.pop().unwrap());
                        _stack.push(x - y);
                    } else {
                        return None;
                    }
                }
                CalculatorInput::Multiply => {
                    if inputs.len() >= 2 {
                        let (y, x) = (_stack.pop().unwrap(), _stack.pop().unwrap());
                        _stack.push(x * y);
                    } else {
                        return None;
                    }
                }
                CalculatorInput::Divide => {
                    if inputs.len() >= 2 {
                        let (y, x) = (_stack.pop().unwrap(), _stack.pop().unwrap());
                        _stack.push(x / y);
                    } else {
                        return None;
                    }
                }
                CalculatorInput::Value(num) => _stack.push(*num),
            };
        }

        _stack.pop()
    }
}
