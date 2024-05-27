use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum: usize = 0;
        let all_lines: Vec<Vec<char>> = lines
            .map_while(Result::ok)
            .map(|line| line.chars().collect())
            .collect();

        for (l, line) in all_lines.iter().enumerate() {
            let mut was_num = false;
            let mut start = 0;
            for (mut c, char) in line.iter().enumerate() {
                let mut should_add = false;
                let is_num = char.is_ascii_digit();
                if is_num && !was_num {
                    was_num = true;
                    start = c;
                } else if !is_num && was_num {
                    should_add = true;
                    was_num = false;
                }
                // number ended at the end of a line
                if is_num && line.len() - 1 == c {
                    c += 1;
                    should_add = true;
                }
                if should_add {
                    let is_part = is_part_number(&all_lines, l, start, c);
                    print!("{}, {} - {}, {}", l, start, c, is_part);
                    let num = line
                        .iter()
                        .take(c)
                        .skip(start)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    print!(", {}", num);
                    if is_part {
                        sum += num;
                    }
                    println!();
                }
            }
        }
        println!("{}", sum);
    }
}

fn is_part_number(all_lines: &[Vec<char>], line: usize, start: usize, end: usize) -> bool {
    let start_line = if line != 0 { line - 1 } else { 0 };
    let end_line = if line != all_lines.len() - 1 {
        line + 1
    } else {
        line
    };

    let mut start_col = std::cmp::min(start, end);
    let mut end_col = std::cmp::max(start, end);

    start_col = if start_col != 0 { start_col - 1 } else { 0 };
    end_col = if end_col as usize != all_lines[line].len() - 1 {
        end_col
    } else {
        end_col - 1
    };

    for line in all_lines.iter().take(end_line + 1).skip(start_line) {
        for char in line.iter().take(end_col + 1).skip(start_col) {
            if char.is_ascii_digit() || *char == '.' {
                continue;
            }
            return true;
        }
    }
    false
}

#[derive(Debug)]
struct AdjacentNum {
    line: usize,
    col_start: usize,
    col_end: usize,
}

fn part_two() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum: usize = 0;
        let all_lines: Vec<Vec<char>> = lines
            .map_while(Result::ok)
            .map(|line| line.chars().collect())
            .collect();

        for (l, line) in all_lines.iter().enumerate() {
            for (c, char) in line.iter().enumerate() {
                if *char == '*' {
                    let adjacent_nums = get_adjacent_nums(&all_lines, l, c);
                    if adjacent_nums.len() != 2 {
                        continue;
                    }
                    let mut mult: usize = 1;
                    for adj in &adjacent_nums {
                        // we know we have 2 numbers, now we just need to know the numbers
                        let val = (adj.col_start..=adj.col_end)
                            .map(|x: usize| all_lines[adj.line][x])
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        print!("{}, ", val);
                        mult *= val;
                    }
                    sum += mult;
                }
            }
        }
        println!("{}", sum);
    }
}

fn get_adjacent_nums(
    all_lines: &[Vec<char>],
    line_number: usize,
    col_number: usize,
) -> Vec<AdjacentNum> {
    let mut to_return: Vec<AdjacentNum> = vec![];
    let line_offsets: Vec<usize> = if line_number == 0 {
        [1, 2].to_vec()
    } else if line_number == all_lines.len() - 1 {
        [0, 1].to_vec()
    } else {
        [0, 1, 2].to_vec()
    };
    let col_offsets: Vec<usize> = if col_number == 0 {
        [1, 2].to_vec()
    } else if col_number == all_lines[0].len() - 1 {
        [0, 1].to_vec()
    } else {
        [0, 1, 2].to_vec()
    };

    for line_offset in line_offsets.iter() {
        for col_offset in col_offsets.iter() {
            let l = line_number + line_offset - 1;
            let mut c = col_number + col_offset - 1;

            let mut char = all_lines[l][c];
            if !char.is_ascii_digit() {
                continue;
            }
            while c > 0 {
                c -= 1;
                char = all_lines[l][c];
                if !char.is_ascii_digit() {
                    c += 1;
                    char = all_lines[l][c];
                    break;
                }
            }
            let col_start = c;
            while char.is_ascii_digit() && c < all_lines[0].len() - 1 {
                c += 1;
                char = all_lines[l][c];
                if !char.is_ascii_digit() {
                    c -= 1;
                    break;
                }
            }
            let col_end = c;
            to_return.push(AdjacentNum {
                col_start,
                col_end,
                line: l,
            });
        }
    }

    to_return.sort_by(|num1: &AdjacentNum, num2: &AdjacentNum| {
        let val1 = num1.line * all_lines[0].len() + num1.col_start;
        let val2 = num2.line * all_lines[0].len() + num2.col_start;
        val1.cmp(&val2)
    });

    to_return.dedup_by(|num1: &mut AdjacentNum, num2: &mut AdjacentNum| {
        num1.line == num2.line && num1.col_start == num2.col_start && num1.col_end == num2.col_end
    });

    to_return
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
