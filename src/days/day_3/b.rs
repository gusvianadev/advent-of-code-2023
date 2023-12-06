use std::collections::HashMap;

pub fn solution(file: &str) -> u64 {
    let mut sum = 0;
    let mut nums: HashMap<usize, Vec<(u64, usize, usize)>> = HashMap::new();
    let mut asterisks = vec![];
    let lines = file.lines().enumerate();

    let mut counting = false;
    let (mut num, mut num_left) = (String::new(), 0);

    for (line_i, line) in lines {
        'chars: for (ch_i, ch) in line.char_indices() {
            let is_digit = ch.is_ascii_digit();

            if is_digit {
                if !counting {
                    counting = true;
                    num_left = if ch_i == 0 { 0 } else { ch_i - 1 };
                }
                num.push(ch);
                continue 'chars;
            }

            if counting {
                let to_add = (num.parse::<u64>().unwrap(), num_left, ch_i);
                let existing = nums.get_mut(&line_i);

                if let Some(existing) = existing {
                    existing.push(to_add);
                } else {
                    nums.insert(line_i, vec![to_add]);
                }
                num.clear();
                counting = false;
            }

            if ch == '*' {
                asterisks.push((line_i, ch_i));
            }
        }

        if counting {
            let to_add = (num.parse::<u64>().unwrap(), num_left, line.len() - 1);
            let existing = nums.get_mut(&line_i);

            if let Some(existing) = existing {
                existing.push(to_add);
            } else {
                nums.insert(line_i, vec![to_add]);
            }
            num.clear();
            counting = false;
        }
    }

    for (my_line_i, my_position) in asterisks {
        let mut used_nums = vec![];
        let line_options = nums.get(&my_line_i);
        let top_options = nums.get(&(my_line_i - 1));
        let bottom_options = nums.get(&(my_line_i + 1));

        if let Some(line_options) = line_options {
            for (num, num_left, num_right) in line_options {
                if *num_left == my_position || *num_right == my_position {
                    used_nums.push(*num);
                }
            }
        }

        if let Some(top_options) = top_options {
            for (num, num_left, num_right) in top_options {
                if *num_left <= my_position && *num_right >= my_position {
                    used_nums.push(*num);
                }
            }
        }

        if let Some(bottom_options) = bottom_options {
            for (num, num_left, num_right) in bottom_options {
                if *num_left <= my_position && *num_right >= my_position {
                    used_nums.push(*num);
                }
            }
        }

        if used_nums.len() == 2 {
            let product: u64 = used_nums.iter().product();
            sum += product;
        }
    }

    sum
}

