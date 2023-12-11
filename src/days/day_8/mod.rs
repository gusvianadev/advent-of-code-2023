mod a;
mod b;

pub fn solutions() {
    let file = include_str!("input.txt");
    let answer_a = a::solution(file);
    let answer_b = b::solution(file);

    println!("WARNING: Couldn't solve day 8 part b yet, it's an infinite loop.");
    println!("Day 8 - answer a: {answer_a}");
    println!("Day 8 - answer b: {answer_b}");
}

