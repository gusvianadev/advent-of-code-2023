type Item = Vec<(String, usize, Option<usize>)>;
pub fn solution(file: &str) -> u64 {
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

