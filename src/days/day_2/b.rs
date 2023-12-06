pub fn solution(file: &str) -> u64 {
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

