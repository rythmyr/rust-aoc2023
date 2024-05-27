use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

struct Part1Max {
    red: u8,
    green: u8,
    blue: u8,
}

const PART1MAX: Part1Max = Part1Max {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum: u16 = 0;
        'games: for line in lines.map_while(Result::ok) {
            let id_game_pair = line.split(": ").collect::<Vec<&str>>();
            let game_and_id = id_game_pair[0].split(' ').collect::<Vec<&str>>();
            let id: u8 = game_and_id[1].parse().unwrap();
            println!("{}", line);

            for round in id_game_pair[1].split("; ") {
                for reveal in round.split(", ") {
                    let color_and_count: Vec<&str> = reveal.split(' ').collect();
                    let count: u8 = color_and_count[0].parse().unwrap();
                    let color = color_and_count[1];

                    let max = match color {
                        "red" => PART1MAX.red,
                        "blue" => PART1MAX.blue,
                        "green" => PART1MAX.green,
                        _ => panic!("unexpected value for color {}", color),
                    };

                    if count > max {
                        println!("failed! {}, {}", color, count);
                        continue 'games;
                    }
                }
            }
            sum += id as u16;
        }
        println!("sum of possible ids: {}", sum);
    }
}

fn part_two() {
    println!("unimplemented");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
