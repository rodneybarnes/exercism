use crate::CalculatorInput::{Add, Divide, Multiply, Subtract, Value};
#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    my_submitted_answer(inputs)
}

fn my_submitted_answer(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut values = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Value(i) => values.push(*i),
            CalculatorInput::Add => {
                let second_number = values.pop();
                let first_number = values.pop();
                if second_number == None { () }
                if first_number == None { return None }
                let value = first_number.unwrap() + second_number.unwrap();
                values.push(value);
            }
            CalculatorInput::Subtract => {
                let second_number = values.pop();
                let first_number = values.pop();
                if second_number == None { () }
                if first_number == None { return None }
                let value = first_number.unwrap() - second_number.unwrap();
                values.push(value);
            }
            CalculatorInput::Multiply => {
                let second_number = values.pop();
                let first_number = values.pop();
                if second_number == None { () }
                if first_number == None { return None }
                let value = first_number.unwrap() * second_number.unwrap();
                values.push(value);
            }
            CalculatorInput::Divide => {
                let second_number = values.pop();
                let first_number = values.pop();
                if second_number == None { () }
                if first_number == None { return None }
                let value = first_number.unwrap() / second_number.unwrap();
                values.push(value);
            }
        }
    }
    match values.len() {
        1 => Some(values[0]),
        _ => None
    }
}

fn top_community_answer(inputs: &[CalculatorInput]) -> Option<i32> {
    let v: Vec<i32> = vec![];
    let mut result = inputs.iter().fold(v, |mut stack, input| {
        if let Some(new) = match input {
            Add => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b + a))),
            Subtract => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b - a))),
            Multiply => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b * a))),
            Divide => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b / a))),
            Value(value) => Some(*value),
        } {
            stack.push(new);
        }
        stack
    });
    result
        .pop()
        .and_then(|x| if result.is_empty() { Some(x) } else { None })
}