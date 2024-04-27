pub fn solution(file: &str) -> i64 {
    let mut sum = 0;

    for line in file.lines() {
        let mut nums = vec![line
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()];

        let mut completed = true;
        let mut index = 0;
        let mut final_nums = vec![*nums[0].last().unwrap()];

        loop {
            let mut points = vec![];
            let current = &nums[index];

            for (i, num) in current.iter().enumerate() {
                if i == current.len() - 1 {
                    break;
                }
                let next = current[i + 1];

                let difference = (next.abs() - num.abs()).abs();
                points.push(difference);

                completed = difference == 0;
            }

            final_nums.push(*points.last().unwrap());
            nums.push(points);

            index += 1;
            if completed {
                break;
            }
        }
        if line.starts_with('-') {
            dbg!(&nums);
        }
        sum += final_nums.iter().sum::<i64>();
    }

    sum
}

