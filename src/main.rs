use std::{fs, collections::HashMap};
use md5;
use regex::Regex;

fn main() {
    //day01();
    //day02();
    //day03();
    day04();
}
fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}
fn day01() {
    let input = read_file("1.txt");
    
    let mut level = 0;
    let mut first_basement_index = 0;
    for c in input.trim().char_indices() {
        match c.1 {
            '(' => level += 1,
            ')' => level -= 1,
            _ => ()
        }
        if level == -1 && first_basement_index == 0 {
            first_basement_index = c.0 + 1;
        }
    }
    println!("Part 1: {}, Part 2: {}", level, first_basement_index);
}
struct Present {
    l: u32,
    w: u32,
    h: u32
}
impl Present {
    fn calc_area(&self) -> u32 {
        let all: Vec<u32> = vec![self.l*self.w, self.w*self.h, self.h*self.l];
        let (area, min) = all
            .iter()
            .fold((0, u32::MAX), |acc, &x| (acc.0 + 2*x, x.min(acc.1)));
        return area + min;
    }
    fn calc_ribbon(&self) -> u32 {
        let mut sides = [self.l, self.w, self.h];
        sides.sort_unstable();
        return 2 * (sides[0] + sides[1]) + sides[0] * sides[1] * sides[2];//sides.iter().fold(0, |acc, &x| acc * x)
    }
    fn new(line: &str) -> Self {
        let mut sizes = line.split('x').map(|c| c.parse().expect("Not a number!"));
        Present {
            l: sizes.next().unwrap(),
            w: sizes.next().unwrap(),
            h: sizes.next().unwrap(),
        }
    }
}
fn day02() {
    let input: Vec<Present> = include_str!("../2.txt")
        .lines()
        .map(Present::new)
        .collect();
    
    let total_area: u32 = input.iter().map(|x| x.calc_area()).sum();
    let total_ribbon: u32 = input.iter().map(|x| x.calc_ribbon()).sum();
    
    println!("Part 1: {}, Part 2: {}", total_area, total_ribbon);
}
fn day03() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> usize {
        let mut pos_map: HashMap<(i32, i32), bool> = HashMap::new();
        pos_map.insert((0, 0,), true);
        let mut cur_pos = (0, 0);
        fn visit(pos: (i32, i32), pos_map: &mut HashMap<(i32, i32), bool>) {
            pos_map.entry(pos).or_insert(true);
        }
        for m in input.chars() {
            match m {
                '>' => { cur_pos.0 += 1; visit(cur_pos, &mut pos_map); },
                '<' => { cur_pos.0 -= 1; visit(cur_pos, &mut pos_map); },
                '^' => { cur_pos.1 += 1; visit(cur_pos, &mut pos_map); },
                'v' => { cur_pos.1 -= 1; visit(cur_pos, &mut pos_map); },
                _ => ()
            }
        }
        pos_map.keys().count()
    }

    fn part2(input: &String) -> usize {
        let mut pos_map: HashMap<(i32, i32), bool> = HashMap::new();
        pos_map.insert((0, 0,), true);
        let mut santa_pos = (0, 0);
        let mut robo_santa_pos = (0, 0);
        fn visit(pos: (i32, i32), pos_map: &mut HashMap<(i32, i32), bool>) {
            pos_map.entry(pos).or_insert(true);
        }
        let mut is_santa = true;
        for m in input.chars() {
            if (is_santa) {
                match m {
                    '>' => { santa_pos.0 += 1; visit(santa_pos, &mut pos_map); },
                    '<' => { santa_pos.0 -= 1; visit(santa_pos, &mut pos_map); },
                    '^' => { santa_pos.1 += 1; visit(santa_pos, &mut pos_map); },
                    'v' => { santa_pos.1 -= 1; visit(santa_pos, &mut pos_map); },
                    _ => ()
                }
            } else {
                match m {
                    '>' => { robo_santa_pos.0 += 1; visit(robo_santa_pos, &mut pos_map); },
                    '<' => { robo_santa_pos.0 -= 1; visit(robo_santa_pos, &mut pos_map); },
                    '^' => { robo_santa_pos.1 += 1; visit(robo_santa_pos, &mut pos_map); },
                    'v' => { robo_santa_pos.1 -= 1; visit(robo_santa_pos, &mut pos_map); },
                    _ => ()
                }
            }
            is_santa = !is_santa;
        }
        pos_map.keys().count()
    }
    
    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day04() {
    let input = String::from("bgvyzdsv");

    fn start_w0(t: String, num: u32) -> bool {
        let mut s = String::from("");
        for i in 0..num {
            s.push('0');
        }
        return t.find(&s) == Some(0)
    }
    fn part1(input: &String) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let salted_input = format!("{}{}", input, index);
            if start_w0(format!("{:x}", md5::compute(salted_input)), 5) {
                return index;
            }
        }
    }
    fn part2(input: &String) -> i32 {
        let mut index = 0;
        loop {
            index += 1;
            let salted_input = format!("{}{}", input, index);
            if start_w0(format!("{:x}", md5::compute(salted_input)), 6) {
                return index;
            }
        }
    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day05() {
    let input: Vec<String> = include_str!("../5.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    fn part1(input: &Vec<String>) -> i32 {
        let mut count = 0;
        
        input
            .iter()
            .map(|line| line.find());

        return count;
    }
    fn part2(input: &Vec<String>) -> i32 {
        return 0;
    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}

/*
fn day06() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day07() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day08() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day09() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day10() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day11() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day12() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day13() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day14() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day15() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day16() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day17() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day18() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day19() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day20() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day21() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day22() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day23() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day24() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
fn day25() {
    let input: String = include_str!("../3.txt")
        .lines()
        .collect();

    fn part1(input: &String) -> i32 {

    }
    fn part2(input: &String) -> i32 {

    }

    println!("Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}
*/

