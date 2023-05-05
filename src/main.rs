//use std::io::Read;
use crossterm::event::*;
use crossterm::{terminal, event, execute, cursor};
use crossterm::terminal::ClearType;
use std::io::stdout;
use std::time::Duration;

struct CleanUp;

// Cleaning up for raw mode
impl Drop for CleanUp {
    fn drop(&mut self) {

        terminal::disable_raw_mode()
            .expect("Could not disable raw mode");

        Output::clear_screen()
            .expect("Error");
    }
}


struct Output {
    winsize: (usize, usize),
}

impl Output {
    fn new() -> Self {

        let winsize = terminal::size()
            .map(|(x,y)| (x as usize, y as usize))
            .unwrap();

        Self { winsize }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All));
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn rows(&self) {
        let scr_rows = self.winsize.1;
        for _ in 0..scr_rows {
            println!("$\r");
        }
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()?;

        self.rows();
        execute!(stdout(), cursor::MoveTo(0,0))
    }
}


struct Reader;

impl Reader {

    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}


struct Editor {

    reader: Reader,
    output: Output,
}

impl Editor {

  fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    fn keypress(&self) -> crossterm::Result<bool> {

        match self.reader.read_key()? {

            KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: event::KeyModifiers::CONTROL, ..
        } => return Ok(false),
        _ => {}
        }
        Ok(true)
    }

    fn run(&self) -> crossterm::Result<bool> {
        self.output.refresh_screen();
        self.keypress()
    }
}



// reading the inputs
fn  main() -> crossterm::Result<()> {

    let _clean_up = CleanUp;

    terminal::enable_raw_mode()
        .expect("Could not turn on Raw mode");

    let editor = Editor::new();
    while editor.run()
        .expect("Error") {}

    Ok(())


}
