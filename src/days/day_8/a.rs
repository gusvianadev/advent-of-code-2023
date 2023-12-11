use std::collections::HashMap;

pub fn solution(file: &str) -> u64 {
    let mut sum = 0;
    let mut lines = file.lines();
    let directions: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| if ch == 'L' { 0 } else { 1 })
        .collect();

    let mut node_map = HashMap::new();

    for line in file.lines().skip(2) {
        let mut line = line.split('=');
        let key = line.next().unwrap().trim();

        let chars = &mut line.next().unwrap().trim().chars();
        chars.next();
        chars.next_back();
        let mut chars = chars.as_str().split(',');
        let value = [chars.next().unwrap(), chars.next().unwrap().trim()];

        node_map.insert(key, value);
    }

    let mut node = node_map.get("AAA").unwrap();
    let mut i = 0;

    loop {
        sum += 1;
        let direction = directions[i];
        let item = node[direction];

        if item == "ZZZ" {
            break;
        }

        node = node_map.get(item).unwrap();

        if i < directions.len() - 1 {
            i += 1;
        } else {
            i = 0;
        }
    }

    sum
}

