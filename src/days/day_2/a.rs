pub fn solution(file: &str) -> u64 {
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

