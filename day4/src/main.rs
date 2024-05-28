fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let lines = include_str!("../input.txt").lines();

    let mut sum = 0;

    lines.for_each(|line| {
        let id_number_split: Vec<&str> = line.split(": ").collect();
        let winning_vs_ours_split: Vec<&str> = id_number_split[1].split(" | ").collect();
        let mut winning_numbers: Vec<usize> = winning_vs_ours_split[0]
            .split(' ')
            .filter_map(|num: &str| {
                if num.is_empty() {
                    return None;
                }
                Some(num.parse::<usize>().unwrap())
            })
            .collect();
        winning_numbers.sort();
        let mut our_numbers: Vec<usize> = winning_vs_ours_split[1]
            .split(' ')
            .filter_map(|num: &str| {
                if num.is_empty() {
                    return None;
                }
                Some(num.parse::<usize>().unwrap())
            })
            .collect();
        our_numbers.sort();

        let mut winning_iter = winning_numbers.iter();
        let our_iter = our_numbers.iter();

        let mut num_matches: usize = 0;

        let mut winning_num = winning_iter.next().unwrap();

        for our_num in our_iter {
            while winning_num < our_num {
                match winning_iter.next() {
                    Some(val) => winning_num = val,
                    None => break,
                }
            }
            if our_num == winning_num {
                num_matches += 1;
            }
        }

        let score = if num_matches == 0 {
            0
        } else {
            u32::pow(2, num_matches as u32 - 1)
        };

        sum += score;

        println!("{:?}", winning_numbers);
        println!("{:?}", our_numbers);
        println!("{}", score);
        println!("{}", line);
    });

    println!("{}", sum);
}

fn get_card_matches(card: &str) -> usize {
    let id_number_split: Vec<&str> = card.split(": ").collect();
    let winning_vs_ours_split: Vec<&str> = id_number_split[1].split(" | ").collect();
    let mut winning_numbers: Vec<usize> = winning_vs_ours_split[0]
        .split(' ')
        .filter_map(|num: &str| {
            if num.is_empty() {
                return None;
            }
            Some(num.parse::<usize>().unwrap())
        })
        .collect();
    winning_numbers.sort();
    let mut our_numbers: Vec<usize> = winning_vs_ours_split[1]
        .split(' ')
        .filter_map(|num: &str| {
            if num.is_empty() {
                return None;
            }
            Some(num.parse::<usize>().unwrap())
        })
        .collect();
    our_numbers.sort();

    let mut winning_iter = winning_numbers.iter();
    let our_iter = our_numbers.iter();

    let mut num_matches: usize = 0;

    let mut winning_num = winning_iter.next().unwrap();

    for our_num in our_iter {
        while winning_num < our_num {
            match winning_iter.next() {
                Some(val) => winning_num = val,
                None => break,
            }
        }
        if our_num == winning_num {
            num_matches += 1;
        }
    }
    num_matches
}

fn process_card(cached: &mut Vec<Option<usize>>, cards: &Vec<&str>, i: usize) -> usize {
    if let Some(val) = cached[i] {
        print!("(cache hit, {}: {})", i, val);
        return val;
    }
    let card = cards[i];
    let num_matches = get_card_matches(card);

    let mut total_cards = 1;

    for x in 1..=num_matches {
        total_cards += process_card(cached, cards, i + x)
    }

    cached[i] = Some(total_cards);

    println!("(cache miss {}, {} cards) end", i, total_cards);

    total_cards
}

fn part_two() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();

    let mut sum = 0;

    let mut cached: Vec<Option<usize>> = Vec::new();
    cached.resize(lines.len(), None);

    lines.iter().enumerate().for_each(|(i, _)| {
        sum += process_card(&mut cached, &lines, i);
        println!("processed {} fully", i);
    });
    println!("{}", sum);
}
