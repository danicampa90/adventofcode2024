use libutils::{map2d::Map2D, read_file_map_lines};

fn main() {
    let lines: Vec<Vec<Option<char>>> = read_file_map_lines("input.txt", &mut |line: String| {
        Ok::<_, ()>(line.chars().map(|ch| Some(ch)).collect())
    })
    .unwrap();

    let mut input = Map2D::new(None, lines.first().unwrap().len());

    for line in lines {
        input.add_row(line);
    }

    println!("Loaded map: x={}, y={}", input.size_x(), input.size_y());
    let mut part1_count = 0;

    for (x, y) in input.coordinates_with_filter(|item| *item == Some('X')) {
        for (move_x, move_y) in input.directions() {
            if input.matches_in_straight_direction(
                x,
                y,
                *move_x,
                *move_y,
                &[Some('M'), Some('A'), Some('S')],
            ) {
                part1_count += 1
            }
        }
    }

    println!("There are {} XMASes in input", part1_count);

    let mut part2_count = 0;
    for (x, y) in input.coordinates_with_filter(|input| *input == Some('A')) {
        let diag_1_matches = match (input.get_value(x - 1, y - 1), input.get_value(x + 1, y + 1)) {
            (Some('M'), Some('S')) => true,
            (Some('S'), Some('M')) => true,
            _ => false,
        };

        let diag_2_matches = match (input.get_value(x - 1, y + 1), input.get_value(x + 1, y - 1)) {
            (Some('M'), Some('S')) => true,
            (Some('S'), Some('M')) => true,
            _ => false,
        };
        if diag_1_matches && diag_2_matches {
            part2_count += 1;
        }
    }
    println!("There are {} X-shaped MAS in the input", part2_count);
}
