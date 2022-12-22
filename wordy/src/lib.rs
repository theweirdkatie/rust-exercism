pub fn answer(command: &str) -> Option<i32> {
    if let Some((mut arguments, actions)) = parse_command(command) {
        arguments.reverse();
        let mut result = arguments.pop()?;
        for act in actions {
            match act.as_str() {
                "plus" => result += arguments.pop()?,
                "minus" => result -= arguments.pop()?,
                "multiplied" => result *= arguments.pop()?,
                "divided" => result /= arguments.pop()?,
                "raised" => result = result.pow(arguments.pop()? as u32),
                "by" | "to" | "the" | "power" => {}
                _ => return None,
            }
        }
        if arguments.is_empty() {
            return Some(result);
        }
    }
    None
}

pub fn parse_command(command: &str) -> Option<(Vec<i32>, Vec<String>)> {
    let mut arg: Vec<i32> = vec![];
    let mut act: Vec<String> = vec![];
    for word in command.split([' ', '?']).filter(|x| x.len() > 0) {
        // ignore beginning
        if word.to_ascii_lowercase() == "what"
            || word.to_ascii_lowercase() == "is"
        {
        // grab numerical arguments
        } else if let Ok(num) = word.parse::<i32>() {
            if !arg.is_empty() && act.is_empty() {
                return None;
            }
            arg.push(num);
        } else if word.contains("th") || word.contains("nd") {
            let nums: Vec<u32> = word.chars().filter_map(|x| x.to_digit(10)).collect();
            if nums.len() > 0 {
                arg.push(nums.iter().sum::<u32>() as i32);
            }
        // grab action commands
        } else if !word.contains("?") {
            if arg.is_empty() {
                return None;
            }
            act.push(word.to_ascii_lowercase());
        // ? is end of line
        } else {
            break;
        }
    }
    Some((arg, act))
}
