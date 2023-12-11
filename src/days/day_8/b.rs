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
    let mut initials = vec![];

    for line in file.lines().skip(2) {
        let mut line = line.split('=');
        let key = line.next().unwrap().trim();

        if key.ends_with('A') {
            initials.push(key);
        }

        let chars = &mut line.next().unwrap().trim().chars();
        chars.next();
        chars.next_back();
        let mut chars = chars.as_str().split(',');
        let value = [chars.next().unwrap(), chars.next().unwrap().trim()];

        node_map.insert(key, value);
    }

    let mut nodes: Vec<&[&str; 2]> = initials
        .iter()
        .map(|initial| node_map.get(initial).unwrap())
        .collect();

    let mut i = 0;
    loop {
        sum += 1;
        let mut completed = true;
        let mut new_nodes = vec![];
        let direction = directions[i];

        for node in &nodes {
            let item = node[direction];

            new_nodes.push(node_map.get(item).unwrap());

            if !item.ends_with('Z') {
                completed = false;
            }
        }

        if completed {
            break;
        }

        if i < directions.len() - 1 {
            i += 1;
        } else {
            i = 0;
        }

        nodes = new_nodes;
    }

    sum
}

