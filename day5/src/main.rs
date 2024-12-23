use std::ops::Range;

struct Map {
    in_start: usize,
    out_start: usize,
    len: usize,
}

impl Map {
    fn map(&self, seed: &usize) -> usize {
        assert!(self.contains(seed));
        *seed - self.in_start + self.out_start
    }

    fn contains(&self, seed: &usize) -> bool {
        (self.in_start..(self.in_start+self.len)).contains(seed)
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    // Vector of references to slices of references to str
    let groups: Vec<&[&str]> = lines.split(|s| s.is_empty()).collect::<Vec<_>>();
    let mut groups_iter = groups.iter();

    let str_vec = groups_iter.next().unwrap();
    let seeds_line = str_vec.first().unwrap();

    // TODO parse seed numbers
    let (_, seeds_str) = seeds_line.split_once(": ").unwrap();
    let seeds: Vec<usize> = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let map_groups: Vec<Vec<Map>> = groups_iter
        .map(|group| {
            let mut group_iter = group.iter();
            let _ = group_iter.next().unwrap(); // get the map name line out

            group_iter
                .map(|map_str| {
                    let map_nums: Vec<usize> = map_str
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();
                    Map {
                        in_start: map_nums[1],
                        out_start: map_nums[0],
                        len: map_nums[2],
                    }
                })
                .collect()
        })
        .collect();

    // apply the maps
    let new_seeds = map_groups
        .iter()
        .fold(seeds, |seeds, map_group: &Vec<Map>| {
            let new_seeds = seeds.into_iter().map(|seed| -> usize {
                let map_opt = map_group.iter().find(|map| map.contains(&seed));
                if let Some(map) = map_opt {
                    map.map(&seed)
                } else {
                    seed
                }
            }).collect();
            println!("{:?}", new_seeds);
            new_seeds
        });

    println!("{:?}", new_seeds);

    println!("{}", new_seeds.iter().min().unwrap());
}

fn part_two() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    // Vector of references to slices of references to str
    let groups: Vec<&[&str]> = lines.split(|s| s.is_empty()).collect::<Vec<_>>();
    let mut groups_iter = groups.iter();

    let str_vec = groups_iter.next().unwrap();
    let seeds_line = str_vec.first().unwrap();

    // TODO parse seed numbers
    let (_, seeds_str) = seeds_line.split_once(": ").unwrap();
    let seeds: Vec<usize> = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let seeds_info = seeds.chunks(2).map(|seed_info: &[usize]| {
        seed_info[0]..seed_info[0]+seed_info[1]
    });

    let map_groups: Vec<Vec<Map>> = groups_iter
        .map(|group| {
            let mut group_iter = group.iter();
            let _ = group_iter.next().unwrap(); // get the map name line out

            group_iter
                .map(|map_str| {
                    let map_nums: Vec<usize> = map_str
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();
                    Map {
                        in_start: map_nums[1],
                        out_start: map_nums[0],
                        len: map_nums[2],
                    }
                })
                .collect()
        })
        .collect();

    // apply the maps
    let new_seeds = map_groups
        .iter()
        .fold(seeds_info, |seeds, map_group: &Vec<Map>| {
            let new_seeds = seeds.into_iter().map(|seed_info| -> usize {
                let map_opt = map_group.iter().find(|map| map.contains(&seed));
                if let Some(map) = map_opt {
                    map.map(&seed)
                } else {
                    seed
                }
            }).collect();
            println!("{:?}", new_seeds);
            new_seeds
        });

    println!("{:?}", new_seeds);

    println!("{}", new_seeds.iter().min().unwrap());
}
