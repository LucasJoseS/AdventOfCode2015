use std::collections::HashMap;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Pos(i32, i32);

fn replace(actual: &mut Pos, direct: char) {
    match direct {
        '^' => {actual.0 += 1},
        'v' => {actual.0 -= 1},
        '>' => {actual.1 += 1},
        '<' => {actual.1 -= 1},
        _ => {}
    }
}

fn main() {
    let mut table: HashMap<Pos, u32> = HashMap::new();
    let start = Pos(0, 0);
    let mut santa = start.clone();
    let mut robot = start.clone();
    let mut fp = File::open("input")
        .expect("Can't open the input file");

    let mut content = String::new();
    fp.read_to_string(&mut content).expect("Can't read the input file");

    table.insert(start.clone(), 1);
    for (index, char) in content.char_indices() {
        if index % 2 == 0 {
            replace(&mut santa, char);
            table.insert(santa.clone(), 1);
        }
        else {
            replace(&mut robot, char);
            table.insert(robot.clone(), 1);
        }
    }

    println!("Visited houses: {:?}", table.len());
}
