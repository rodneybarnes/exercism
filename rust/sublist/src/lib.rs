use std::{collections::HashSet, hash::Hash};
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Eq + Hash + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    // if the first and second lists are the exact same, then return Comparison::Equal
    if first_list == second_list {
        return Comparison::Equal;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist<T: Eq + Hash + Debug>(first_list: &[T], second_list: &[T]) -> bool {
    let mut indexes = Vec::new();
    for first_item in first_list {
        let result = second_list.iter().position(|second_item| second_item == first_item);
        match result {
            Some(index) => indexes.push(index),
            None => return false
        }
    }

    let mut current_index = indexes[0];
    for index in indexes.iter().skip(1) {
        if index - current_index != 1 {
            return false;
        }
        current_index = *index;
    }
    true
}

pub fn bad_sublist<T: Eq + Hash + Debug>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_hashset: HashSet<&T> = _first_list.iter().collect();
    let second_hashset: HashSet<&T> = _second_list.iter().collect();

    if first_hashset == second_hashset {
        println!("{:?}", first_hashset);
        println!("{:?}", second_hashset);
        return Comparison::Equal;
    }

    if first_hashset.is_subset(&second_hashset) {
        return Comparison::Sublist;
    }

    if first_hashset.is_superset(&second_hashset) {
        return Comparison::Superlist;
    }

    return Comparison::Unequal;
}
