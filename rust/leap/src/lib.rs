pub fn is_leap_year(year: u64) -> bool {
    most_starred_solution(year)
}

/// This was the solution I submitted
fn my_solution(year: u64) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true
            } else {
                return false
            }
        }
        return true
    } else {
        return false
    }
}

/// fuuuck this is great
fn most_starred_solution(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}