use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
#[repr(u8)]
enum HandTypes {
    High = 1,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

type HandMap<T> = BTreeMap<u8, T>;
type NestedHands = BTreeMap<u8, HandMap<HandMap<HandMap<HandMap<HandMap<u16>>>>>>;

pub fn solution(file: &str) -> u64 {
    let mut sum: u64 = 0;
    let mut hands: NestedHands = BTreeMap::new();
    let key_vals: [(char, u8); 5] = [('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10)];
    let dictionary = HashMap::from(key_vals);

    let make_ch = |ch: char| match ch {
        _ if ch.is_ascii_digit() => ch.to_digit(10).unwrap() as u8,
        _ => *dictionary.get(&ch).unwrap(),
    };

    for line in file.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (cards, bid) = (parts[0], parts[1].parse::<u16>().unwrap());
        let mut chars = cards.chars();
        let mut card_count = HashMap::new();

        let mut add_to_count = |ch: char| {
            card_count
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1_u8);
        };

        let first_ch = chars.next().unwrap();
        add_to_count(first_ch);
        let second_ch = chars.next().unwrap();
        add_to_count(second_ch);
        let third_ch = chars.next().unwrap();
        add_to_count(third_ch);
        let fourth_ch = chars.next().unwrap();
        add_to_count(fourth_ch);
        let fifth_ch = chars.next().unwrap();
        add_to_count(fifth_ch);

        let mut hand_type = HandTypes::High;
        match card_count.len() {
            1 => {
                hand_type = HandTypes::Five;
            }
            2 => {
                let (_, count) = card_count.iter().next().unwrap();

                match count {
                    1 | 4 => {
                        hand_type = HandTypes::Four;
                    }
                    _ => hand_type = HandTypes::Full,
                }
            }
            3 => {
                hand_type = HandTypes::Three;
                for (_, count) in card_count {
                    if count == 2 {
                        hand_type = HandTypes::Two;
                        break;
                    }
                }
            }
            4 => {
                hand_type = HandTypes::One;
            }
            _ => {}
        }
        let curr_card = hands.entry(hand_type as u8).or_default();
        let curr_card = curr_card.entry(make_ch(first_ch)).or_default();
        let curr_card = curr_card.entry(make_ch(second_ch)).or_default();
        let curr_card = curr_card.entry(make_ch(third_ch)).or_default();
        let curr_card = curr_card.entry(make_ch(fourth_ch)).or_default();
        curr_card.insert(make_ch(fifth_ch), bid);
    }

    let mut ranking = 0_u32;
    for hands in hands.values() {
        for hands in hands.values() {
            for hands in hands.values() {
                for hands in hands.values() {
                    for hands in hands.values() {
                        for bid in hands.values() {
                            ranking += 1;
                            sum += *bid as u64 * ranking as u64;
                        }
                    }
                }
            }
        }
    }

    sum
}

