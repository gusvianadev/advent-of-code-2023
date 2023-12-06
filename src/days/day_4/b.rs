enum Phase {
    Winning,
    Player,
}

fn walk(lines: &Vec<&str>, index: usize) -> u64 {
    let mut sum = 0;

    let mut phase = Phase::Winning;
    let mut num = String::new();
    let mut winning_nums = vec![];
    let mut player_nums = vec![];

    let line = lines[index];
    let skip_to = line.find(':').take().unwrap() + 2;
    'chars: for ch in line.chars().skip(skip_to) {
        match ch {
            ':' => {
                phase = Phase::Winning;
                continue 'chars;
            }
            '|' => {
                phase = Phase::Player;
                continue 'chars;
            }
            _ if ch.is_ascii_digit() => {
                num.push(ch);
            }
            _ if ch == ' ' && !num.is_empty() => {
                let num_val = num.parse::<u64>().unwrap();

                match phase {
                    Phase::Winning => {
                        winning_nums.push(num_val);
                    }
                    Phase::Player => {
                        player_nums.push(num_val);
                    }
                }

                num.clear();
            }
            _ => {}
        }
    }

    player_nums.push(num.parse::<u64>().unwrap());

    let mut matches = 0;
    'victories: for num in player_nums {
        if index + matches == lines.len() - 1 {
            break 'victories;
        }

        if winning_nums.contains(&num) {
            matches += 1;
            let walked = walk(lines, index + matches);
            sum += 1 + walked;
        }
    }

    sum
}

pub fn solution(file: &str) -> u64 {
    let lines: Vec<&str> = file.lines().collect();
    let mut sum = 0;

    for (i, _) in lines.iter().enumerate() {
        sum += 1 + walk(&lines, i);
    }

    sum
}

