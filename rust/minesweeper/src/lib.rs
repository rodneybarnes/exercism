use std::str;
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (row_index, row) in minefield.iter().enumerate() {
        let mut row_string = "".to_string();
        for (point_index, point) in row.as_bytes().iter().enumerate() {
            match point {
                32 => {
                    let mut count = 0;
                    if row_index > 0 {
                        let last_row = minefield[row_index - 1];
                        if *last_row.as_bytes().iter().nth(point_index).unwrap_or_else(|| &32) as char == '*' {
                            count = count + 1;
                        }
                        if point_index > 0
                            && *last_row.as_bytes().iter().nth(point_index - 1).unwrap_or_else(|| &32) as char
                                == '*'
                        {
                            count = count + 1;
                        }
                        if point_index < last_row.len()
                            && *last_row.as_bytes().iter().nth(point_index + 1).unwrap_or_else(|| &32) as char
                                == '*'
                        {
                            count = count + 1;
                        }
                    }
                    if row_index < minefield.len() - 1 {
                        let next_row = minefield[row_index + 1];
                        if *next_row.as_bytes().iter().nth(point_index).unwrap_or_else(|| &32) as char == '*' {
                            count = count + 1;
                        }
                        if point_index > 0
                            && *next_row.as_bytes().iter().nth(point_index - 1).unwrap_or_else(|| &32) as char
                                == '*'
                        {
                            count = count + 1;
                        }
                        if point_index < next_row.len()
                            && *next_row.as_bytes().iter().nth(point_index + 1).unwrap_or_else(|| &32) as char
                                == '*'
                        {
                            count = count + 1;
                        }
                    }
                    if point_index > 0
                        && *row.as_bytes().iter().nth(point_index - 1).unwrap_or_else(|| &32) as char == '*'
                    {
                        count = count + 1;
                    }
                    if point_index < row.len()
                        && *row.as_bytes().iter().nth(point_index + 1).unwrap_or_else(|| &32) as char == '*'
                    {
                        count = count + 1;
                    }
                    if count == 0 {
                        row_string = format!("{}{}", row_string.to_string(), ' ');
                    } else {
                        row_string = format!("{}{}", row_string.to_string(), count);
                    }
                }
                _ => row_string = format!("{}{}", row_string.to_string(), *point as char),
            };
        }
        result.push(row_string);
    }
    result
}
