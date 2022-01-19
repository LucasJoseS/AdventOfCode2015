use std::collections::HashMap;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Pos(i32, i32);

fn main() {
    let mut table: HashMap<Pos, u32> = HashMap::new();
    let mut pos = Pos(0, 0);
    let mut fp = File::open("input")
        .expect("Can't open the input file");

    let mut content = String::new();
    fp.read_to_string(&mut content).expect("Can't read the input file");

    table.insert(pos.clone(), 1);
    for char in content.chars() {
        match char {
            '^' => {pos.0 += 1},
            'v' => {pos.0 -= 1},
            '>' => {pos.1 += 1},
            '<' => {pos.1 -= 1},
            _ => {}
        }

        table.insert(pos.clone(), 1);
    }

    println!("Visited houses: {:?}", table.len());
}
