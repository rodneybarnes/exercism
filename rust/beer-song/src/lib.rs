pub fn verse(n: u32) -> String {
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => {
            let one_less = n - 1;
            let bottle_word = if one_less > 1 { "bottles" } else { "bottle" };
            format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {one_less} {bottle_word} of beer on the wall.\n")
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .fold(String::new(), |ans, x| {
            let new_line = if x == end { "" } else { "\n" };
            format!("{ans}{}{new_line}", &verse(x))
        })
}
