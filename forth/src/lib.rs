use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<i32>,
    custom_commands: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let new_input = input
            .to_uppercase()
            .split_whitespace()
            .map(|st| st.to_owned())
            .collect::<Vec<String>>();

        let mut result = Ok(());
        let mut iter = new_input.iter().enumerate();

        loop {
            match iter.next() {
                None => break,
                Some((pos, inp)) => {
                    result = match inp.as_str() {
                        ":" => {
                            if let Some(pos_end) = new_input[pos..].iter().position(|x| x == ";") {
                                for _ in pos..(pos + pos_end) { iter.next(); }
                                self.define_word(new_input[pos..=(pos+pos_end)].to_vec())
                            } else {
                                Err(Error::InvalidWord)
                            }
                        }
                        _ => self.eval_token(inp.clone()),
                    };
                    if !result.is_ok() {
                        break;
                    }
                }
            }
        }
        result
    }

    fn eval_token(&mut self, token: String) -> Result {
        if let Ok(num) = token.parse() {
            self.stack.push(num);
            return Ok(());
        }
        match token.as_str() {
            word if self.custom_commands.contains_key(word) => {
                self.custom_action(self.custom_commands.get(word).unwrap().clone())
            }
            "+" => self.add(),
            "-" => self.sub(),
            "*" => self.mul(),
            "/" => self.div(),
            "DUP" => self.dup(),
            "DROP" => self.drop(),
            "SWAP" => self.swap(),
            "OVER" => self.over(),
            _ => Err(Error::UnknownWord),
        }
    }

    fn add(&mut self) -> Result {
        let x1 = self.stack.pop();
        let x2 = self.stack.pop();

        if x1.is_some() && x2.is_some() {
            self.stack.push(x1.unwrap() + x2.unwrap());
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn sub(&mut self) -> Result {
        let x2 = self.stack.pop();
        let x1 = self.stack.pop();

        if x1.is_some() && x2.is_some() {
            self.stack.push(x1.unwrap() - x2.unwrap());
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn mul(&mut self) -> Result {
        let x1 = self.stack.pop();
        let x2 = self.stack.pop();

        if x1.is_some() && x2.is_some() {
            self.stack.push(x1.unwrap() * x2.unwrap());
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn div(&mut self) -> Result {
        let x2 = self.stack.pop();
        let x1 = self.stack.pop();

        if x1.is_some() && x2.is_some() {
            if x2 == Some(0) {
                return Err(Error::DivisionByZero);
            }
            self.stack.push(x1.unwrap() / x2.unwrap());
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn dup(&mut self) -> Result {
        if self.stack.len() > 0 {
            let x = self.stack[self.stack.len() - 1];
            self.stack.push(x);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn drop(&mut self) -> Result {
        if self.stack.len() > 0 {
            let _ = self.stack.pop();
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn swap(&mut self) -> Result {
        let x1 = self.stack.pop();
        let x2 = self.stack.pop();

        if x1.is_some() && x2.is_some() {
            self.stack.push(x1.unwrap());
            self.stack.push(x2.unwrap());
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn over(&mut self) -> Result {
        if self.stack.len() > 1 {
            let x = self.stack[self.stack.len() - 2];
            self.stack.push(x);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn define_word(&mut self, new_word: Vec<String>) -> Result {
        let mut word_iter = new_word.iter();

        // double check first char
        if word_iter.next() != Some(&":".to_string()) || new_word.len() < 4 {
            return Err(Error::InvalidWord);
        }

        // save word command
        if let Some(new_command) = word_iter.next() {
            if new_command.parse::<i32>().is_ok() {
                return Err(Error::InvalidWord);
            }
            let mut series = vec![];
            while let Some(com) = word_iter.next() {
                if com != ";" {
                    if self.custom_commands.contains_key(com) {
                        series.append(&mut self.custom_commands.get(com).unwrap().clone());
                    } else {
                        series.push(com.to_owned());
                    }
                }
            }
            self.custom_commands.insert(new_command.to_string(), series);

            Ok(())
        } else {
            Err(Error::InvalidWord)
        }
    }

    fn custom_action(&mut self, command: Vec<String>) -> Result {
        for action in command {
            let result = self.eval_token(action);
            if result.is_err() {
                return result;
            }
        }
        Ok(())
    }
}


