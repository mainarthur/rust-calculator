use std::io;

enum State {
    FirstValue,
    Operation,
    SecondValue,
}

fn is_opeartor(operator: char) -> bool {
    ['+', '-', '*', '/'].contains(&operator)
}

fn is_invalid_number(number: f32) -> bool {
    number.is_nan() || number.is_infinite()
}

fn calculate(a: f32, b: f32, operator: char) -> f32 {
    if operator == '+' {
        a + b
    } else if operator == '-' {
        a - b
    } else if operator == '*' {
        a * b
    } else if operator == '/' {
        a / b
    } else {
        0f32
    }
}

fn main() {
    let mut first_value: f32 = 0f32;
    let mut operator: char = '\0';
    let mut second_value: f32;
    let mut state = State::FirstValue;
    loop {
        let mut user_input = String::new();

        match io::stdin().read_line(&mut user_input) {
            Ok(bytes_read) => {
                // removing \n from input
                user_input.pop();

                match state {
                    State::FirstValue => {
                        state = State::Operation;
                        let first_value_parsed = user_input.parse();
                        if first_value_parsed.is_err() {
                            state = State::FirstValue;
                            println!("Enter correct number!");
                        } else {
                            first_value = first_value_parsed.unwrap();
                            if is_invalid_number(first_value) {
                                state = State::FirstValue;
                                println!("Enter correct number!");
                            }
                        }
                    }
                    State::Operation => {
                        state = State::SecondValue;
                        operator = user_input.pop().unwrap();

                        if !is_opeartor(operator) || bytes_read > 2 {
                            state = State::Operation;
                            println!("Enter correct operator!");
                        }
                    }
                    State::SecondValue => {
                        state = State::FirstValue;
                        let second_value_parsed = user_input.parse();

                        if second_value_parsed.is_err() {
                            state = State::SecondValue;
                            println!("Enter correct number!");
                        } else {
                            second_value = second_value_parsed.unwrap();
                            if is_invalid_number(second_value) {
                                state = State::SecondValue;
                                println!("Enter correct number!");
                            } else {
                                let result: f32 = calculate(first_value, second_value, operator);

                                println!(
                                    "{first} {operator} {second} = {result}",
                                    first = first_value,
                                    operator = operator,
                                    second = second_value,
                                    result = result
                                );
                            }
                        }
                    }
                }
            }
            Err(error) => {
                println!("Err: {}", error);
            }
        }
    }
}
