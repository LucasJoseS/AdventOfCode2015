use md5;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut fp = File::open("input").expect("Can't open the input file");

    let mut secret = String::new();
    fp.read_to_string(&mut secret).expect("Can't read the input file");
    secret.remove(secret.len()-1);

    let mut decimal = 0;
    loop {
        let test = format!("{}{}", secret, decimal);
        let message = md5::compute(test);

        if format!("{:?}", message).starts_with("00000") {
            print!("{}, {} -> {:?}\n", secret, decimal, message);
            break;
        }

        decimal += 1;
    }
}
