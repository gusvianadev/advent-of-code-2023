mod a;
mod b;

pub fn solutions() {
    let file = include_str!("input.txt");
    let answer_a = a::solution(file);
    let answer_b = b::solution(file);

    println!("Day 9 - answer a: {answer_a}");
    println!("Day 9 - answer b: {answer_b}");
}

