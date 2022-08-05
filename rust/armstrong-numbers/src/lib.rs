pub fn is_armstrong_number(num: u32) -> bool {
    let sum = num
        .to_string()
        .chars()
        .fold(0, |acc, x| acc + x.to_digit(10).unwrap().pow(num.to_string().len().try_into().unwrap()));
    sum == num
}
