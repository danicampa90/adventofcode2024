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

    let mut guard_pos = input
        .coordinates_with_filter(|cell| cell.is_guard())
        .into_iter()
        .next()
        .unwrap();

    let mut guard_direction = (0, -1);
    while !input.get_value(guard_pos.0, guard_pos.1).is_out_of_map() {
        input.set_value(guard_pos.0, guard_pos.1, MapCell::Visited);

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

    let part1 = input.coordinates_with_filter(|c| *c == MapCell::Visited);
    println!("There are {} visited steps", part1.len())
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
