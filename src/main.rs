use std::{fs, collections::HashMap};

fn main() {
    //day01();
    //day02();
    day03();
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
    let mut pos_map: HashMap<(u32, u32), bool> = HashMap::new();
    for m in input.chars() {
        match m {
            
        }
    }
}