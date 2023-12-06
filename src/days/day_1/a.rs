pub fn solution(file: &str) -> u64 {
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

