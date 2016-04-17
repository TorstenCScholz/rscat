use std::io::Read;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    for (i, arg) in args.iter().enumerate() {
        if i >= 1 {
            let mut file = File::open(arg).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            print!("{}", content);
        }
    }
}
