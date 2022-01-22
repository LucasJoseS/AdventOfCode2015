use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::PartialEq;

#[derive(Debug)]
enum Action {
    TurnOff,
    TurnOn,
    Toggle,
}

#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn from(x: u32, y: u32) -> Coordinate {
        Coordinate {x, y}
    }
}

#[derive(Debug)]
struct House {
    lit: HashMap<Coordinate, bool>,
}

impl House {
    fn new() -> House {
        House { lit: HashMap::new() }
    }

    fn turn_on(&mut self, start: Coordinate, end: Coordinate) {
        self.board(start, end, &|_| { Some(true) });
    }

    fn turn_off(&mut self, start: Coordinate, end: Coordinate) {
        self.board(start, end, &|_| { None });
    }

    fn toggle(&mut self, start: Coordinate, end: Coordinate) {
        self.board(start, end, &|value: Option<&bool>| {
            match value {
                Some(_) => None,
                None => Some(true),
            }
        });
    }

    fn board(&mut self, start: Coordinate, end: Coordinate, value: &dyn Fn(Option<&bool>) -> Option<bool>) {
        for y in start.y..=end.y {
            self.line(Coordinate::from(start.x, y), end, value);
        }
    }

    fn line(&mut self, start: Coordinate, end: Coordinate, value: &dyn Fn(Option<&bool>) -> Option<bool>) {
        for x in start.x..=end.x {
            self.pos(Coordinate::from(x, start.y), value);
        }
    }

    fn pos(&mut self, pos: Coordinate, value: &dyn Fn(Option<&bool>) -> Option<bool>) {
        match value(self.lit.get(&pos)) {
            None => self.lit.remove(&pos),
            Some(_) => self.lit.insert(pos, true),
        };
    }
}

fn parse(line: &str) -> Option<(Action, Coordinate, Coordinate)> {
    let re: Regex = Regex::new(r"(turn (on|off)|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let cap = re.captures(line)?;

    let action: Action = match cap.get(2){
        Some(on_off) => {
            match on_off.as_str() {
                "on" => Action::TurnOn,
                "off" => Action::TurnOff,
                _ => panic!("Unexpected turn value")
            }
        }
        None => Action::Toggle,
    };

    let start = Coordinate::from(cap[3].parse().unwrap(), cap[4].parse().unwrap());
    let end   = Coordinate::from(cap[5].parse().unwrap(), cap[6].parse().unwrap());

    Some((action, start, end))
}

fn main() {
    let mut fp = File::open("input")
        .expect("Can't open input file");

    let mut lines = String::new();
    fp.read_to_string(&mut lines)
        .expect("Can't read input file");
    
    let mut lines: Vec<&str> = lines.split("\n").collect();
    let mut house = House::new();

    for line in lines {
        let (action, start, end) = parse(line).expect("Unexpected line format");

        match action {
            Action::TurnOn => house.turn_on(start, end),
            Action::TurnOff => house.turn_off(start, end),
            Action::Toggle => house.toggle(start, end),
        }
    }

    println!("Lit lights: {}", house.lit.len());
}
