pub fn is_armstrong_number(num: u32) -> bool {
    let sum = num
        .to_string()
        .chars()
        .fold(0, |acc, x| acc + x.to_digit(10).unwrap().pow(num.to_string().len().try_into().unwrap()));
    sum == num
}

pub fn one_of_the_most_starred_solutions(num: u32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len() as u32;
    let sum = num_str.chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(num_len))
        .sum();
    num == sum
}