pub fn solution(file: &str) -> u32 {
    let mut total = 0.0;
    let mut lines = file.lines();
    let time = lines.next().take().unwrap();
    let distance = lines.next().take().unwrap();
    let mut race = (0_f32, 0_f32);

    let mut num = String::new();
    for ch in time.chars().skip(11) {
        if ch == ' ' {
            continue;
        }

        num.push(ch);
    }

    race.0 = num.parse::<f32>().unwrap();
    num.clear();

    for ch in distance.chars().skip(11) {
        if ch == ' ' {
            continue;
        }

        num.push(ch);
    }

    race.1 = num.parse::<f32>().unwrap();
    num.clear();

    let (limit, record) = race;
    let inner = limit.powi(2) - 4_f32 * record;
    let min = (((-limit + inner.sqrt()) / -2_f32) + 1_f32).floor();
    let max = (((-limit - inner.sqrt()) / -2_f32) - 1_f32).ceil();

    if total == 0.0 {
        total += max - min + 1_f32;
    } else {
        total *= max - min + 1_f32;
    }

    total as u32
}

