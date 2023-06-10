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
    cursor_controller: CursContr,
}

impl Output {
    fn new() -> Self {

        let winsize = terminal::size()
            .map(|(x,y)| (x as usize, y as usize))
            .unwrap();

        Self {
            winsize,
            editor_contents: EditorContents::new(),
            cursor_controller: CursContr::new(winsize),
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

        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;

        queue!(
        self.editor_contents,
        cursor::MoveTo(cursor_x as u16, cursor_y as u16),
        cursor::Show
        )?;

        self.editor_contents.flush()
    }

    fn move_curs(&mut self,direction: KeyCode) {
        self.cursor_controller.move_curs(direction);
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

    fn keypress(&mut self) -> crossterm::Result<bool> {

        match self.reader.read_key()? {

            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL, ..
            } => return Ok(false),

            KeyEvent {
                code: direction @ (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End
                ),
                modifiers: KeyModifiers::NONE, ..
            } => self.output.move_curs(direction),

            KeyEvent {
                code: val @
                    (KeyCode::PageUp
                    |KeyCode::PageDown
                    ),
                modifiers: KeyModifiers::NONE, ..

            } => (0..self.output.winsize.1).for_each(|_| {
                    self.output.move_curs(if matches!(val, KeyCode::PageUp) {
                        KeyCode::Up
                    }else{
                        KeyCode::Down
                    });
                }),
        _ => {}
        }
        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.keypress()
    }
}


struct CursContr {
    cursor_x: usize,
    cursor_y: usize,
    scr_columns: usize,
    scr_rows: usize,
}

impl CursContr {
    fn new(winsize: (usize, usize)) -> CursContr {

        Self {
            cursor_x: 0,
            cursor_y: 0,
            scr_columns: winsize.0,
            scr_rows: winsize.1,
        }

    }


    fn move_curs(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y != self.scr_rows - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x != self.scr_columns - 1 {
                    self.cursor_x += 1;
                }
            }

            KeyCode::End => self.cursor_x = self.scr_columns -1,
            KeyCode::Home => self.cursor_x = 0,
            _ => unimplemented!(),
        }
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

