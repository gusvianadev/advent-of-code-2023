fn solution_a(file: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in file.lines() {
        let mut left = char::default();
        let mut right = char::default();

        for char in line.chars() {
            let to_digit = char.to_digit(10);

            if to_digit.is_some() {
                if left == char::default() {
                    left = char;
                } else {
                    right = char;
                }
            }
        }

        if right == char::default() {
            right = left;
        }

        let mut num = String::from(left);
        num.push(right);
        let num: u64 = num.parse().unwrap();
        sum += num;
    }

    sum
}

struct SpelledDigit {
    name: &'static str,
    value: u8,
}

fn solution_b(file: &str) -> u64 {
    let spelled_digits = vec![
        SpelledDigit {
            name: "one",
            value: 1,
        },
        SpelledDigit {
            name: "two",
            value: 2,
        },
        SpelledDigit {
            name: "three",
            value: 3,
        },
        SpelledDigit {
            name: "four",
            value: 4,
        },
        SpelledDigit {
            name: "five",
            value: 5,
        },
        SpelledDigit {
            name: "six",
            value: 6,
        },
        SpelledDigit {
            name: "seven",
            value: 7,
        },
        SpelledDigit {
            name: "eight",
            value: 8,
        },
        SpelledDigit {
            name: "nine",
            value: 9,
        },
    ];
    let mut sum: u64 = 0;

    for line in file.lines() {
        struct Side {
            index: usize,
            value: u8,
        }

        let mut left = Side { index: 0, value: 0 };
        let mut right = Side { index: 0, value: 0 };

        for (i, char) in line.char_indices() {
            let digit = char.to_digit(10);

            if let Some(num) = digit {
                if left.value == 0 {
                    left.index = i;
                    left.value = num as u8 * 10;
                } else {
                    right.index = i;
                    right.value = num as u8;
                }
            }
        }

        if right.value == 0 {
            right.index = left.index;
            right.value = left.value / 10;
        }

        for digit in spelled_digits.iter() {
            'chars: for (i, _) in line.char_indices() {
                let digit_len = digit.name.len();

                if i + digit_len > line.len() || line.len() < digit.name.len() {
                    break 'chars;
                }

                let slice = &line[i..i + digit_len];

                if slice == digit.name {
                    if i < left.index {
                        left.index = i;
                        left.value = digit.value * 10;
                    }

                    if i > right.index {
                        right.index = i;
                        right.value = digit.value
                    }
                }
            }
        }

        if right.value == 0 {
            right.value = left.value / 10;
        }

        let num = left.value as u64 + right.value as u64;
        sum += num;
    }

    sum
}

pub fn solutions() {
    let file = std::fs::read_to_string("inputs/day2.txt").unwrap();
    let answer_a = solution_a(&file);
    let answer_b = solution_b(&file);
    println!("Day 1 - answer a: {answer_a}");
    println!("Day 1 - answer b: {answer_b}");
}

