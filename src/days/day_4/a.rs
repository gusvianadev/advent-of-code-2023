enum Phase {
    Winning,
    Player,
}

pub fn solution(file: &str) -> u64 {
    let mut sum = 0;
    let lines = file.lines();

    for line in lines {
        let mut points = 0;
        let mut phase = Phase::Winning;
        let mut num = String::new();
        let mut winning_nums = vec![];
        let mut player_nums = vec![];

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

        for num in player_nums {
            if winning_nums.contains(&num) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        sum += points;
    }

    sum
}

