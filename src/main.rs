extern crate getch;

use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    let stdin = getch::Getch::new();

    loop {
        let byte = stdin.getch().unwrap();

        write!(stdout, "0x{:x} ", byte).unwrap();
        stdout.flush().unwrap();
    }
}
