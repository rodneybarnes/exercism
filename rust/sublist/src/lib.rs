use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => if a.windows(n).any(|v| v == b) {Superlist} else {Unequal},
        (m, n) if m < n => if b.windows(m).any(|v| v == a) {Sublist} else {Unequal},
        (_, _) => if a == b {Equal} else {Unequal},
    }
}

pub fn my_sublist<T: Eq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    // if the first and second lists are the exact same, then return Comparison::Equal
    if first_list == second_list {
        return Comparison::Equal;
    }

    if first_list.is_empty() {
        return Comparison::Sublist;
    }

    if second_list.is_empty() {
        return Comparison::Superlist;
    }

    let mut first_list_string = format!("{:?}", first_list);
    let mut second_list_string = format!("{:?}", second_list);
    first_list_string.remove(0);
    first_list_string.pop();
    second_list_string.remove(0);
    second_list_string.pop();

    if first_list_string.contains(second_list_string.as_str()) {
        return Comparison::Superlist;
    }

    if second_list_string.contains(first_list_string.as_str()) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
