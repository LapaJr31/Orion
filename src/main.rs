//use std::io::Read;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{terminal, event};
//use std::io;
use std::time::Duration;

struct CleanUp;

// Cleaning up for raw mode
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode()
            .expect("Could not disable raw mode")
    }
}

// reading the inputs
fn  main() {

    let _clean_up = CleanUp;

    terminal::enable_raw_mode()
        .expect("Could not turn on Raw mode");


    loop {
        if event::poll(Duration::from_secs(60))
            .expect("Error") {

         if let Event::Key(event) = event::read()
            .expect("Failed to read the line") {

                match event {
                    KeyEvent {

                        code: KeyCode::Char('q'), // exiting on press of crtl-q
                        modifiers: event::KeyModifiers::CONTROL, ..

                    } => break,

                    _ => {

                    //todo

                    }
            }
                println!("{:?}\r", event);
        };

    } else {
        println!("No input yet\r");
    }




    }


   // let mut buf = [0; 1];

    //while io::stdin()
    //    .read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {

    //    let charr = buf[0] as char;
    //    if charr.is_control(){
    //        println!("{}\r", charr as u8);
    //    } else {
    //        println!("{}\r", charr);
    //    }
    // }



    //terminal::disable_raw_mode();
}
