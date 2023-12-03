fn solution_a(file: &str) -> u64 {
    let mut sum: u64 = 0;
    let red_cubes: u64 = 12;
    let green_cubes: u64 = 13;
    let blue_cubes: u64 = 14;

    for line in file.lines() {
        let mut line_id = String::new();
        let mut id_done = false;
        let mut is_possible = true;

        let mut digit = String::new();
        let mut skip = 0;
        'chars: for ch in line.chars().skip(5) {
            if skip > 0 {
                skip -= 1;
                continue 'chars;
            }

            let cube_amount: u64 = if !digit.is_empty() {
                digit.parse().unwrap()
            } else {
                0
            };

            match ch {
                ':' => {
                    id_done = true;
                    continue 'chars;
                }
                _ if ch.is_ascii_digit() => {
                    if !id_done {
                        line_id.push(ch);
                        continue 'chars;
                    }
                    digit.push(ch);
                }
                'r' => {
                    skip = 2;
                    if cube_amount > red_cubes {
                        is_possible = false;
                    }
                    digit = String::new();
                }
                'b' => {
                    skip = 3;
                    if cube_amount > blue_cubes {
                        is_possible = false;
                    }
                    digit = String::new();
                }
                'g' => {
                    skip = 4;
                    if cube_amount > green_cubes {
                        is_possible = false;
                    }
                    digit = String::new();
                }
                _ => {
                    continue 'chars;
                }
            }
        }

        if is_possible {
            let num: u64 = line_id.parse().unwrap();
            sum += num;
        }
    }

    sum
}

fn solution_b(file: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in file.lines() {
        let mut min_red_cubes: u64 = 0;
        let mut min_green_cubes: u64 = 0;
        let mut min_blue_cubes: u64 = 0;

        let mut digit = String::new();
        let mut skip = 0;
        let mut id_done = false;
        'chars: for ch in line.chars().skip(5) {
            if skip > 0 {
                skip -= 1;
                continue 'chars;
            }

            if ch == ':' {
                id_done = true;
                skip = 1;
                continue 'chars;
            }

            if !id_done {
                continue 'chars;
            }

            let cube_amount: u64 = if !digit.is_empty() {
                digit.parse().unwrap()
            } else {
                0
            };

            match ch {
                ' ' => {
                    continue 'chars;
                }
                'r' => {
                    skip = 4;
                    if cube_amount > min_red_cubes {
                        min_red_cubes = cube_amount;
                    }
                    digit = String::new();
                }
                'b' => {
                    skip = 5;
                    if cube_amount > min_blue_cubes {
                        min_blue_cubes = cube_amount;
                    }
                    digit = String::new();
                }
                'g' => {
                    skip = 6;
                    if cube_amount > min_green_cubes {
                        min_green_cubes = cube_amount;
                    }
                    digit = String::new();
                }
                _ => {
                    digit.push(ch);
                }
            }
        }

        sum += min_red_cubes * min_blue_cubes * min_green_cubes;
    }

    sum
}

pub fn solutions() {
    let file = std::fs::read_to_string("inputs/day1.txt").unwrap();
    let answer_a = solution_a(&file);
    let answer_b = solution_b(&file);
    println!("Day 2 - answer a: {answer_a}");
    println!("Day 2 - answer b: {answer_b}");
}

