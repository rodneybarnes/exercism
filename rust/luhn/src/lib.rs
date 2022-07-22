use std::os::unix::process;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() < 2 {
        return false;
    }
    let process_result: Result<Vec<u32>, _> = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(index, num)| {
            match num {
                Some(num) => {
                    if index % 2 == 0 {
                        Ok(num)
                    } else {
                        if (num * 2) > 9 {
                            Ok(num * 2 - 9)
                        } else {
                            Ok(num * 2)
                        }
                    }
                },
                None => Err(())
            }
            
        })
        .collect();
        //.fold(0, |acc, num| acc + num);
    match process_result {
        Ok(numbers) => {
            let sum = numbers
                .iter()
                .fold(0, |acc, num| acc + num);
            sum % 10 == 0
        },
        _ => false
    }
}

pub fn most_starred(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}