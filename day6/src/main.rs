use std::collections::HashSet;

use libutils::{map2d::Map2D, read_file_map_lines};

fn main() {
    let lines: Vec<Vec<MapCell>> = read_file_map_lines("input.txt", &mut |line: String| {
        Ok::<_, ()>(line.chars().map(parse_char).collect())
    })
    .unwrap();

    let mut input = Map2D::new(MapCell::OutOfMap, lines.first().unwrap().len());

    for line in lines {
        input.add_row(line);
    }

    let (_, mut walked_path) = run_simulation(input.clone());
    println!("There are {} visited steps", walked_path.len());

    let guard_pos = input
        .coordinates_with_filter(|cell| cell.is_guard())
        .into_iter()
        .next()
        .unwrap();

    walked_path.retain(|i| *i != guard_pos);

    let mut looping_options = 0;
    for (x, y) in walked_path {
        let mut modified_map = input.clone();
        modified_map.set_value(x, y, MapCell::Obstacle);
        //println!("Run simulation with {},{}", x, y);
        let (is_loop, _) = run_simulation(modified_map);
        if is_loop {
            looping_options += 1;
        }
    }
    println!(
        "There are {} possible options for placing an obstacle",
        looping_options
    );
}

fn run_simulation(mut input: Map2D<MapCell>) -> (bool, Vec<(i32, i32)>) {
    let mut guard_pos = input
        .coordinates_with_filter(|cell| cell.is_guard())
        .into_iter()
        .next()
        .unwrap();

    let mut guard_direction = (0, -1);
    let mut guard_historical_positions = HashSet::new();
    let mut loop_detected = false;
    while !input.get_value(guard_pos.0, guard_pos.1).is_out_of_map() {
        input.set_value(guard_pos.0, guard_pos.1, MapCell::Visited);
        if (guard_historical_positions.contains(&(guard_direction, guard_pos))) {
            loop_detected = true;
            break;
        }
        guard_historical_positions.insert((guard_direction, guard_pos));

        loop {
            let mut next_guard_pos = guard_pos;
            next_guard_pos.0 += guard_direction.0;
            next_guard_pos.1 += guard_direction.1;
            if input.get_value(next_guard_pos.0, next_guard_pos.1) == MapCell::Obstacle {
                guard_direction = (-guard_direction.1, guard_direction.0);
                continue; // go back to checking the next guard position;
            }
            guard_pos = next_guard_pos;
            break;
        }
    }

    return (
        loop_detected,
        input.coordinates_with_filter(|c| *c == MapCell::Visited),
    );
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum MapCell {
    Guard(i32, i32), // facing direction, as vector
    Empty,
    Obstacle,
    Visited,
    OutOfMap,
}

impl MapCell {
    pub fn is_guard(&self) -> bool {
        match self {
            MapCell::Guard(_, _) => true,
            _ => false,
        }
    }
    pub fn is_out_of_map(&self) -> bool {
        match self {
            MapCell::OutOfMap => true,
            _ => false,
        }
    }
}

fn parse_char(ch: char) -> MapCell {
    match ch {
        '^' => MapCell::Guard(0, -1),
        '#' => MapCell::Obstacle,
        'X' => MapCell::Visited,
        '.' => MapCell::Empty,
        _ => panic!("WTF is this character? {}", ch),
    }
}
