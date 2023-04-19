use std::cmp::Ordering;
use std::io;

use crate::{
    crossterm::{cursor, style, terminal},
    grapheme::Graphemes,
    selectbox::SelectBox,
    termutil::{self, Boundary},
    Output, Result,
};

/// Select specific state.
pub struct State {
    pub editor: SelectBox,
    pub prev: SelectBox,
    pub next: SelectBox,
    /// Title displayed on the initial line.
    pub title: Option<Graphemes>,
    pub title_color: Option<style::Color>,
    /// A symbol to emphasize the selected item (e.g. ">").
    pub label: Graphemes,
    pub label_color: style::Color,
    pub init_move_down_lines: u16,
    pub selected_cursor_position: u16,
    pub window: Option<u16>,
    pub suffix_after_trim: Graphemes,
}

impl Output for State {
    type Output = String;

    fn output(&self) -> Self::Output {
        self.editor.get().to_string()
    }
}

impl State {
    pub fn pre_render<W: io::Write>(&self, out: &mut W) -> Result<()> {
        // Move down with init_move_down_lines.
        if 0 < self.init_move_down_lines {
            crossterm::execute!(out, cursor::MoveToNextLine(self.init_move_down_lines))?;
        }

        // Render the title.
        if let Some(title) = &self.title {
            if let Some(color) = self.title_color {
                crossterm::execute!(out, style::SetForegroundColor(color))?;
            }
            crossterm::execute!(out, style::Print(title), cursor::MoveToNextLine(1))?;
            if self.title_color.is_some() {
                crossterm::execute!(out, style::SetForegroundColor(style::Color::Reset))?;
            }
        }

        // Return to the initial position.
        crossterm::execute!(out, cursor::MoveTo(0, 0))
    }

    pub fn render<W: io::Write>(&mut self, out: &mut W) -> Result<()> {
        let next = self.next.clone();
        if !next.data.is_empty() {
            crossterm::execute!(out, cursor::SavePosition)?;

            // Check to leave the space to render the data.
            let title_lines =
                termutil::num_lines(self.title.as_ref().unwrap_or(&Graphemes::default()))?;
            let used_space = self.init_move_down_lines + title_lines;
            if terminal::size()?.1 <= used_space {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Terminal does not leave the space to render.",
                ));
            }

            // Move down the lines already written.
            let move_down_lines = self.init_move_down_lines
                + termutil::num_lines(self.title.as_ref().unwrap_or(&Graphemes::default()))?;
            if 0 < move_down_lines {
                crossterm::execute!(out, cursor::MoveToNextLine(move_down_lines))?;
            }

            let selectbox_position = next.position();
            let from = selectbox_position - self.selected_cursor_position as usize;
            let to = selectbox_position
                + (self.selectbox_lines(&next)? - self.selected_cursor_position) as usize;

            for i in from..to {
                crossterm::execute!(out, terminal::Clear(terminal::ClearType::CurrentLine))?;
                if i == selectbox_position {
                    crossterm::execute!(out, style::SetForegroundColor(self.label_color))?;
                }
                crossterm::execute!(
                    out,
                    style::Print(&next.get_with_index(i).append_prefix_and_trim_suffix(
                        &if i == selectbox_position {
                            self.label.to_owned()
                        } else {
                            Graphemes::from(" ".repeat(self.label.width()))
                        },
                        &self.suffix_after_trim
                    )?)
                )?;
                if i == selectbox_position {
                    crossterm::execute!(out, style::SetForegroundColor(style::Color::Reset))?;
                }
                if termutil::compare_cursor_position(Boundary::Bottom)? == Ordering::Less {
                    crossterm::execute!(out, cursor::MoveToNextLine(1))?;
                }
            }

            // Return to the initial position.
            crossterm::execute!(out, cursor::RestorePosition)?;
        }
        Ok(())
    }
}

impl State {
    pub fn move_up(&mut self) -> Result<()> {
        if self.selected_cursor_position == 0 {
            self.selected_cursor_position = 0;
        } else {
            self.selected_cursor_position -= 1;
        }
        Ok(())
    }

    pub fn move_down(&mut self) -> Result<()> {
        if self.selectbox_lines(&self.editor)? > 0 {
            let limit = self.selectbox_lines(&self.editor)? - 1;
            if self.selected_cursor_position >= limit {
                self.selected_cursor_position = limit;
            } else {
                self.selected_cursor_position += 1;
            }
        }
        Ok(())
    }

    pub fn move_head(&mut self) -> Result<()> {
        self.selected_cursor_position = 0;
        Ok(())
    }

    pub fn move_tail(&mut self) -> Result<()> {
        self.selected_cursor_position = self.selectbox_lines(&self.editor)? - 1;
        Ok(())
    }

    pub fn selectbox_lines(&self, selectbox: &SelectBox) -> Result<u16> {
        let left_space = terminal::size()?.1
            - (self.init_move_down_lines
                + termutil::num_lines(self.title.as_ref().unwrap_or(&Graphemes::default()))?);
        Ok(*vec![
            left_space,
            self.window.unwrap_or(left_space),
            selectbox.data.len() as u16,
        ]
        .iter()
        .min()
        .unwrap_or(&left_space))
    }
}
