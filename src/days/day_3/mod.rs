use std::collections::HashMap;

type Item = Vec<(String, usize, Option<usize>)>;

fn solution_a(file: &str) -> u64 {
    let mut sum = 0;

    let [mut prev_line, mut curr_line]: [Item; 2] = [vec![], vec![]];
    for line in file.lines() {
        let mut counting = false;
        let (mut num, mut num_start) = (String::new(), 0);

        let mut add_to_sum = |num: &String| {
            sum += num.parse::<u64>().unwrap();
        };

        'chars: for (ch_i, ch) in line.char_indices() {
            let is_digit = ch.is_ascii_digit();

            if is_digit {
                if !counting {
                    counting = true;
                    num_start = ch_i;
                }
                num.push(ch);
                continue 'chars;
            }

            if counting {
                curr_line.push((num.clone(), num_start, Some(ch_i - 1)));
                num.clear();
                counting = false;
            }

            if ch == '.' {
                continue 'chars;
            }

            curr_line.push((ch.to_string(), ch_i, None));
        }

        if counting {
            curr_line.push((num.clone(), num_start, Some(line.len() - 1)));
            num.clear();
        }

        let mut prev_used_nums = vec![];
        let mut curr_used_nums = vec![];
        'curr_line_items: for (i, (item, my_start, my_end)) in curr_line.iter().enumerate() {
            if let Some(my_end) = my_end {
                let my_left = if *my_start == 0 { 0 } else { my_start - 1 };
                let my_right = my_end + 1;

                if i > 0 {
                    let (_, prev_start, prev_end) = curr_line[i - 1];

                    if prev_end.is_none() && prev_start == my_left {
                        add_to_sum(item);
                        curr_used_nums.push(i);
                        continue 'curr_line_items;
                    }
                }

                if i < curr_line.len() - 1 {
                    let (_, next_start, next_end) = curr_line[i + 1];

                    if next_end.is_none() && next_start == my_right {
                        add_to_sum(item);
                        curr_used_nums.push(i);
                        continue 'curr_line_items;
                    }
                }

                for (_, symbol_position, prev_end) in prev_line.iter() {
                    if prev_end.is_none()
                        && *symbol_position >= my_left
                        && *symbol_position <= my_right
                    {
                        add_to_sum(item);
                        curr_used_nums.push(i);
                        continue 'curr_line_items;
                    }
                }
            } else {
                for (num_i, (num, num_start, num_end)) in prev_line.iter().enumerate() {
                    if let Some(num_end) = num_end {
                        let num_left = if *num_start == 0 { 0 } else { num_start - 1 };
                        let num_right = num_end + 1;

                        if num_left <= *my_start && *my_start <= num_right {
                            add_to_sum(num);
                            prev_used_nums.push(num_i);
                        }
                    }
                }
            }
        }

        for (i, prev) in prev_used_nums.iter().enumerate() {
            let prev = if *prev == 0 || i == 0 {
                *prev
            } else {
                prev - i
            };

            prev_line.remove(prev);
        }

        for (i, curr) in curr_used_nums.iter().enumerate() {
            let curr = if *curr == 0 || i == 0 {
                *curr
            } else {
                curr - i
            };

            curr_line.remove(curr);
        }

        prev_line.clear();
        prev_line.append(&mut curr_line);
    }

    sum
}

fn solution_b(file: &str) -> u64 {
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

pub fn solutions() {
    let file = include_str!("input.txt");
    let answer_a = solution_a(file);
    let answer_b = solution_b(file);

    println!("Day 3 - answer a: {answer_a}");
    println!("Day 3 - answer a: {answer_b}");
}

