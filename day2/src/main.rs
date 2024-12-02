use libutils::read_file_map_lines;
use thiserror::Error;

fn main() {
    let input = read_file_map_lines("input.txt", &mut parse_line).unwrap();
    let mut safe_lists = vec![];
    for list in input.into_iter() {
        if let Some(_) = find_with_list_permutations(&list, |item| {
            if is_safe(item) {
                Some(item.len())
            } else {
                None
            }
        }) {
            safe_lists.push(list);
        } else {
        }
    }
    println!("There are {:?} safe lists.", safe_lists.len())
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Parsing error:")]
    ParseError(#[from] std::num::ParseIntError),
}

fn parse_line(str: String) -> Result<Vec<i32>, ParseError> {
    let elements = str
        .split_ascii_whitespace()
        .map(|str| str.parse::<i32>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    Ok(elements)
}

fn is_safe(list: &Vec<i32>) -> bool {
    let decreasing = (list.first().unwrap() - list.last().unwrap()) > 0;
    let mut prev_number: Option<i32> = None;
    let mut is_safe = true;

    for number in list {
        match prev_number {
            None => {
                prev_number = Some(*number);
            }
            Some(prev) if (decreasing && prev > *number && prev <= number + 3) => {
                prev_number = Some(*number);
            }
            Some(prev) if (!decreasing && prev < *number && prev >= number - 3) => {
                prev_number = Some(*number);
            }
            _ => {
                is_safe = false;
                break;
            }
        }
    }
    return is_safe;
}

pub fn find_with_list_permutations<'b, T, TRes>(
    vector: &'b std::vec::Vec<T>,
    check: fn(&Vec<T>) -> Option<TRes>,
) -> Option<TRes>
where
    T: Clone + 'static,
    TRes: 'static,
{
    if let Some(res) = check(vector) {
        return Some(res);
    }
    for i in 0..vector.len() {
        let mut new_arr = (*vector).clone();
        new_arr.remove(i);
        if let Some(res) = check(&new_arr) {
            return Some(res);
        }
    }
    return None;
}
