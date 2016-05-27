use std::io::Read;
use std::fs::File;
use std::env;

fn get_file_content(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}

fn main() {
    let args: Vec<_> = env::args().collect();

    for (i, arg) in args.iter().enumerate() {
        if i >= 1 {
            print!("{}", get_file_content(arg));
        }
    }
}

#[test]
fn test_get_file_content() {
    let content = get_file_content("test/test.txt");
    assert_eq!("This is a test\n", content);
}
