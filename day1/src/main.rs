use libutils::{frequencies, read_file_map_lines};
use thiserror::Error;

fn main() {
    let numbers = read_file_map_lines("input.txt", &mut parse_line).unwrap();
    let mut list1 = vec![];
    let mut list2 = vec![];
    for (num1, num2) in numbers.into_iter() {
        list1.push(num1);
        list2.push(num2);
    }

    list1.sort();
    list2.sort();

    let mut sum_of_differences = 0;
    for i in 0..list1.len() {
        let diff = list1[i] - list2[i];
        sum_of_differences += if diff > 0 { diff } else { -diff };
    }

    println!("Sum of differences: {}", sum_of_differences);

    let list2_freqs = frequencies(list2);
    let mut similarity_score = 0;
    for i in list1.into_iter() {
        similarity_score += i * (*list2_freqs.get(&i).unwrap_or(&0) as i32);
    }

    println!("Similarity score: {}", similarity_score);
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Parsing error:")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Index error")]
    IndexError,
}

fn parse_line(str: String) -> Result<(i32, i32), ParseError> {
    let mut elements = str.split_ascii_whitespace();
    Ok((
        elements.next().ok_or(ParseError::IndexError)?.parse()?,
        elements.next().ok_or(ParseError::IndexError)?.parse()?,
    ))
}
