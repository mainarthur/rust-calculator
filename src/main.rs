use std::io;

enum State {
    FirstValue,
    Operation,
    SecondValue,
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
                            if first_value.is_nan() || first_value.is_infinite() {
                                state = State::FirstValue;
                                println!("Enter correct number!");
                            }
                        }
                    }
                    State::Operation => {
                        state = State::SecondValue;
                        operator = user_input.pop().unwrap();

                        if !(['+', '-', '*', '/'].contains(&operator)) || bytes_read > 2 {
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
                            if second_value.is_nan() || second_value.is_infinite() {
                                state = State::SecondValue;
                                println!("Enter correct number!");
                            } else {
                                let result: f32 = if operator == '+' {
                                    first_value + second_value
                                } else if operator == '-' {
                                    first_value - second_value
                                } else if operator == '*' {
                                    first_value * second_value
                                } else if operator == '/' {
                                    first_value / second_value
                                } else {
                                    0f32
                                };

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
