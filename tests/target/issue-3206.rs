fn apply_arithmetic_operation<'a>(op: ArithmeticOperation, state: &mut State) -> Result<'a, ()> {
    match (state.stack.pop(), state.stack.pop()) {
        (Some(value1), Some(value2)) => match (value1, value2) {
            (Value::Number(num1), Value::Number(num2)) => {
                let result = match op {
                    ArithmeticOperation::Addition => num1 + num2,
                    ArithmeticOperation::Subtraction => num1 - num2,
                    ArithmeticOperation::Multiplication => num1 * num2,
                    ArithmeticOperation::Division => num1 / num2,
                    ArithmeticOperation::Remainder => num1 % num2,
                };
                state.stack.push(Value::Number(result));
                Ok(())
            }
            _ => Err(Error::RuntimeError(
                "cannot sum values on top of stack: the two topmost values on the stack should be numbers",
            )),
        },
        _ => {
            return Err(Error::RuntimeError(
                "cannot sum values on top of stack: there must be at least two values on the stack",
            ));
        }
    }
}
