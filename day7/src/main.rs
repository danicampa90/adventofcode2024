use std::collections::HashSet;

use libutils::{memoizer::Memoizer, read_file_map_lines};

fn main() {
    let problems: Vec<Problem> =
        read_file_map_lines("input.txt", &mut |line: String| parse_line(line)).unwrap();

    let mut memoizer = Memoizer::new();

    let mut part1_calibration_value = 0;
    for problem in problems.iter() {
        let possible_values = memoizer.calculate(try_combinations, &problem.inputs.as_slice());
        println!(
            "With problem {:?} i have target {:?} and got {:?} possible results",
            problem.inputs,
            problem.target,
            possible_values.len()
        );
        if possible_values.contains(&problem.target) {
            println!("Ok!");
            part1_calibration_value += problem.target
        }
    }

    println!("Part 1/2 calibration value: {:?}", part1_calibration_value)
}

struct Problem {
    target: i64,
    inputs: Vec<i64>,
}

fn parse_line(line: String) -> Result<Problem, ()> {
    let mut split_line = line.split(':').into_iter();
    let target = split_line.next().ok_or(())?.parse().map_err(|_| ())?;
    let inputs = split_line.next().ok_or(())?;
    let inputs = inputs
        .split(' ')
        .into_iter()
        .skip(1)
        .map(|str| str.parse().unwrap())
        .collect();

    Ok(Problem { target, inputs })
}

fn try_combinations<'a>(
    memoizer: &mut Memoizer<&'a [i64], HashSet<i64>>,
    input: &&'a [i64],
) -> HashSet<i64> {
    let last_num = input[input.len() - 1];
    let rest = &input[..input.len() - 1];
    if rest.len() == 0 {
        let mut base_case = HashSet::new();
        base_case.insert(last_num);
        return base_case;
    }
    let rest_possible_values = memoizer.calculate(try_combinations, rest);
    let set_with_additions = rest_possible_values.iter().map(|num| num + last_num);
    let set_with_multiplication = rest_possible_values.iter().map(|num| num * last_num);
    let set_with_concatenation = rest_possible_values
        .iter()
        .map(|num| num.to_string() + last_num.to_string().as_str())
        .map(|str| str.parse().unwrap());

    return set_with_additions
        .chain(set_with_multiplication)
        // Comment out the concatenation for part 1
        .chain(set_with_concatenation)
        .collect();
}
