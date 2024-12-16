use std::collections::{HashMap, HashSet};

use libutils::read_file_map_lines;
fn main() {
    let parsed: Vec<ParsedLine> = read_file_map_lines("input.txt", &mut parse_line).unwrap();
    let mut rules_succeeds = HashMap::new();
    let mut rules_precedes = HashMap::new();
    let mut page_sequences: Vec<Vec<i32>> = vec![];
    for parsed_line in parsed {
        match parsed_line {
            ParsedLine::OrderingRule(prev, next) => {
                rules_precedes
                    .entry(prev)
                    .or_insert(HashSet::new())
                    .insert(next);
                rules_succeeds
                    .entry(next)
                    .or_insert(HashSet::new())
                    .insert(prev);
            }
            ParsedLine::PageList(vec) => page_sequences.push(vec),
            ParsedLine::EmptySeparator => {}
        }
    }

    let mut mid_sum = 0;
    for sequence in page_sequences.iter() {
        if is_valid(&sequence, &rules_precedes) {
            //println!("{sequence:?}");
            mid_sum += sequence[sequence.len() / 2];
        }
    }
    println!("Part 1 answer: {}", mid_sum);

    let mut mid_sum = 0;
    for sequence in page_sequences.iter() {
        if !is_valid(sequence, &rules_precedes) {
            let fixed_sequence = fix_sequence(sequence.clone(), &rules_precedes);
            //println!("{:?} -> {:?}", sequence, fixed_sequence);
            mid_sum += fixed_sequence[fixed_sequence.len() / 2];
        }
    }

    println!("Part 2 answer {}", mid_sum);
}

pub fn is_valid(sequence: &Vec<i32>, rules_precedes: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut seen_pages = HashSet::new();
    for page in sequence {
        match rules_precedes.get(page) {
            Some(must_precede_these_pages)
                if seen_pages
                    .intersection(must_precede_these_pages)
                    .next()
                    .is_some() =>
            {
                return false
            }
            Some(_) => {}
            None => {}
        }
        seen_pages.insert(*page);
    }
    return true;
}

pub fn fix_sequence(
    mut sequence: Vec<i32>,
    rules_precedes: &HashMap<i32, HashSet<i32>>,
) -> Vec<i32> {
    let mut did_swap = true;
    while did_swap {
        did_swap = false;
        'outer_loop: for idx in 0..sequence.len() {
            let page = sequence[idx];
            match rules_precedes.get(&page) {
                Some(must_precede_these_pages) => {
                    for previous_page_idx in 0..idx {
                        let prev_page = sequence[previous_page_idx];
                        if must_precede_these_pages.contains(&prev_page) {
                            sequence.swap(idx, previous_page_idx);
                            did_swap = true;
                            break 'outer_loop;
                        }
                    }
                }
                None => {}
            }
        }
    }
    return sequence;
}
enum ParsedLine {
    OrderingRule(i32, i32), // number, successor
    PageList(Vec<i32>),
    EmptySeparator,
}

fn parse_line(str: String) -> Result<ParsedLine, ()> {
    if str.contains('|') {
        let split: Vec<i32> = str.split('|').map(|s| s.parse().unwrap()).collect();
        assert_eq!(split.len(), 2);
        Ok(ParsedLine::OrderingRule(split[0], split[1]))
    } else if str == "" {
        Ok(ParsedLine::EmptySeparator)
    } else if str.contains(",") {
        let list = str.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(ParsedLine::PageList(list))
    } else {
        panic!("Unexpected input: {}", str);
    }
}
