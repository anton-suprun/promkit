use std::{fmt, io::Write};

use anyhow::Result;

use crate::crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType, ScrollUp},
};

#[derive(Clone)]
pub struct Engine<W: Write> {
    out: W,
}

impl<W: Write> Engine<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        execute!(self.out, Clear(ClearType::All), MoveTo(0, 0))
    }

    pub fn write<D: fmt::Display>(&mut self, string: D) -> Result<(), std::io::Error> {
        execute!(self.out, Print(format!("{}", string)))
    }

    pub fn move_to(&mut self, pos: (u16, u16)) -> Result<(), std::io::Error> {
        execute!(self.out, MoveTo(pos.0, pos.1))
    }

    pub fn is_bottom() -> Result<bool> {
        Ok(cursor::position()?.1 + 1 == terminal::size()?.1)
    }

    pub fn move_to_next_line(&mut self, scroll_up: bool) -> Result<()> {
        crossterm::execute!(self.out, cursor::MoveToNextLine(1))?;
        if scroll_up {
            execute!(self.out, ScrollUp(1))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod clear {
        use super::super::*;

        #[test]
        fn test() {
            let out = vec![];
            let mut engine = Engine::new(out);
            assert!(engine.clear().is_ok());
            assert_eq!(
                String::from_utf8(strip_ansi_escapes::strip(engine.out)).unwrap(),
                ""
            );
        }
    }

    mod write {
        use super::super::*;

        #[test]
        fn test() {
            let out = vec![];
            let mut engine = Engine::new(out);
            assert!(engine.write("abcde").is_ok());
            assert_eq!(
                String::from_utf8(strip_ansi_escapes::strip(engine.out)).unwrap(),
                "abcde"
            );
        }
    }
}
