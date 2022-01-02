fn main() {
    solve_part_1(aoc::input());
}

fn parse_input(contents: String) -> (Vec<char>, Vec<Vec<char>>) {
    let mut lines = contents.lines();
    let iea: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }
    (iea, grid)
}

fn enhance(iea: Vec<char>, image: Vec<Vec<char>>, dot_state: bool) -> (Vec<Vec<char>>, i32) {
    let mut new_image: Vec<Vec<char>> = Vec::new();
    let mut empty_row: Vec<char> = Vec::new();
    for _ in 0..image[0].len() + 2 {
        let c = if dot_state { '.' } else { '#' };
        empty_row.push(c);
    }
    let mut num_lit: i32 = 0;
    for y in -1..=image.len() as i32 {
        let mut new_row: Vec<char> = Vec::new();
        for x in -1..=image[0].len() as i32 {
            let mut bin_string: String = "".to_string();
            for ynum in y as i32 - 1..=y as i32 + 1 {
                for xnum in x as i32 - 1..=x as i32 + 1 {
                    if ynum < 0
                        || xnum < 0
                        || ynum >= image.len() as i32
                        || xnum >= image[0].len() as i32
                    {
                        bin_string = format!("{}{}", bin_string, if dot_state { 0 } else { 1 });
                    } else {
                        bin_string = format!(
                            "{}{}",
                            bin_string,
                            match image[ynum as usize][xnum as usize] {
                                '.' => 0,
                                '#' => 1,
                                _ => 7,
                            }
                        );
                    }
                }
            }
            let ind = usize::from_str_radix(&bin_string, 2).unwrap();
            let mut next_char = iea[ind];
            if next_char == '#' {
                num_lit += 1;
            }
            new_row.push(next_char);
        }
        new_image.push(new_row);
    }
    (new_image, num_lit)
}

fn solve_part_1(contents: String) {
    let (iea, image) = parse_input(contents);
    let (image, _) = enhance(iea.clone(), image, true);
    let (mut image, mut num_lit) = enhance(iea.clone(), image, false);
    println!("Part 1: {}", num_lit);
    let mut dot_state = true;
    for _ in 0..48 {
        (image, num_lit) = enhance(iea.clone(), image.clone(), dot_state);
        dot_state = !dot_state;
    }
    println!("Part 2: {}", num_lit);
}
