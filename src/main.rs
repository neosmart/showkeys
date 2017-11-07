extern crate termion;

use std::io::Write;
use std::ops::Deref;
use termion::input::TermReadEventsAndRaw;
use termion::event::{Event, Key};
use termion::raw::IntoRawMode;

fn main() {
    println!("showkeys {} by NeoSmart Technologies", env!("CARGO_PKG_VERSION"));
    let mut stdout = std::io::stdout();

    let tty = termion::get_tty()
        .expect("Could not open handle to tty!")
        .into_raw_mode()
        .unwrap();
    let raw_events = TermReadEventsAndRaw::events_and_raw(tty.deref());
    let mut ctrl_c = false;

    for (event, codes) in raw_events.map(|e| e.unwrap()) {
        match event {
            Event::Key(key) => {
                //abort on 
                if key == Key::Ctrl('c') && ctrl_c {
                    break;
                } else if key == Key::Ctrl('c') {
                    ctrl_c = true;
                } else {
                    ctrl_c = false;
                }

                for c in codes {
                    write!(stdout, "0x{:x},", c).unwrap();
                }
                //replace the last comma with a space
                stdout.write(&[b'\x08', b' ']).unwrap();
                stdout.flush().unwrap();
            }
            //ignore non-keyboard events
            _ => {}
        };

    }
}
