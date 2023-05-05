//use std::io::Read;
use crossterm::event::*;
use crossterm::{terminal, event, execute, cursor, queue};
use crossterm::terminal::ClearType;
use std::io::stdout;
use std::time::Duration;
use std::io;
use std::io::Write;

const VERSION: &str = "0.0.1";


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

struct EditorContents{
    content: String,
}


impl EditorContents {
    fn new() -> Self {
        Self{
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    fn str_push(&mut self, string: &str) {
        self.content.push_str(string)
    }

}

impl io::Write for EditorContents {
    fn write (&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

struct Output {
    winsize: (usize, usize),
    editor_contents: EditorContents,
}

impl Output {
    fn new() -> Self {

        let winsize = terminal::size()
            .map(|(x,y)| (x as usize, y as usize))
            .unwrap();

        Self {
            winsize,
            editor_contents: EditorContents::new(),
        }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All));
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn rows(&mut self) {

        let scr_rows = self.winsize.1;
        let scr_columns = self.winsize.0;

        for i in 0..scr_rows {
            if i== scr_rows /3 {

                let mut welcome = format!("Orion Editor. Version {}", VERSION);
                if welcome.len() > scr_columns {
                    welcome.truncate(scr_columns)
                }

                let mut padding = (scr_columns - welcome.len()) /2;
                if padding != 0 {
                    self.editor_contents.push('$');
                    padding -= 1
                }

                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.str_push(&welcome);


            } else {

            self.editor_contents.push('$');
            }
            queue!(

            self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();

            if i < scr_rows - 1 {
                self.editor_contents.str_push("\r\n");
            }
        }
    }

    fn refresh_screen(&mut self) -> crossterm::Result<()> {

        queue!(self.editor_contents, terminal::Clear(ClearType::All), cursor:: MoveTo(0,0))?;

        self.rows();

        queue!(
            self.editor_contents,
            cursor::MoveTo(0,0), cursor::Show )?;
        self.editor_contents.flush()
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

    fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen();
        self.keypress()
    }
}



// reading the inputs
fn  main() -> crossterm::Result<()> {

    let _clean_up = CleanUp;

    terminal::enable_raw_mode()
        .expect("Could not turn on Raw mode");

    let mut editor = Editor::new();
    while editor.run()
        .expect("Error") {}

    Ok(())


}
