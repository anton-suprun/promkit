use crate::{
    crossterm::{
        event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
        style::ContentStyle,
    },
    grapheme::{matrixify, Graphemes},
    history::History,
    pane::Pane,
    suggest::Suggest,
    text_buffer::TextBuffer,
};

use super::Editor;

/// Edit mode.
pub enum Mode {
    /// Insert a char at the current position.
    Insert,
    /// Overwrite a char at the current position.
    Overwrite,
}

pub struct TextEditor {
    pub textbuffer: TextBuffer,
    pub history: History,
    pub suggest: Suggest,

    pub label: String,
    pub label_style: ContentStyle,
    pub style: ContentStyle,
    pub cursor_style: ContentStyle,
    pub mode: Mode,
    pub mask: Option<char>,
}

impl Editor for TextEditor {
    fn gen_pane(&self, width: u16) -> Pane {
        let mut buf = Graphemes::default();
        buf.append(&mut Graphemes::new_with_style(
            &self.label,
            self.label_style,
        ));
        buf.append(
            &mut self
                .textbuffer
                .graphemes(self.style, self.cursor_style, self.mask),
        );

        Pane::new(
            matrixify(width as usize, buf),
            self.textbuffer.position / width as usize,
        )
    }

    /// Default key bindings for readline.
    ///
    /// | Key                    | Description
    /// | :--                    | :--
    /// | <kbd> Enter </kbd>     | Exit the event-loop
    /// | <kbd> CTRL + C </kbd>  | Exit the event-loop with an error
    /// | <kbd> ← </kbd>         | Move the cursor backward
    /// | <kbd> → </kbd>         | Move the cursor forward
    /// | <kbd> CTRL + A </kbd>  | Move the cursor to the beginning of the input buffer
    /// | <kbd> CTRL + E </kbd>  | Move the cursor to the end of the input buffer
    /// | <kbd> ↑ </kbd>         | Retrieve the previous input from history
    /// | <kbd> ↓ </kbd>         | Retrieve the next input from history
    /// | <kbd> Backspace </kbd> | Erase a character at the current cursor position
    /// | <kbd> CTRL + U </kbd>  | Erase all characters on the current line
    /// | <kbd> TAB </kbd>       | Perform tab completion by searching for suggestions
    fn handle_event(&mut self, event: &Event) {
        match event {
            // Before finishing on enter event.
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => {
                // Insert the result to history.
                self.history
                    .insert(self.textbuffer.to_string_without_cursor())
            }

            // Move cursor.
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.prev(),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.next(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.move_to_head(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.move_to_tail(),

            // Erase char(s).
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.erase(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('u'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.erase_all(),

            // Choose history
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => {
                if self.history.prev() {
                    self.textbuffer.replace(self.history.get())
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => {
                if self.history.next() {
                    self.textbuffer.replace(self.history.get())
                }
            }

            // Choose suggestion
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => {
                if let Some(new) = self
                    .suggest
                    .search(self.textbuffer.to_string_without_cursor())
                {
                    self.textbuffer.replace(new)
                }
            }

            // Input char.
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => match self.mode {
                Mode::Insert => self.textbuffer.insert(*ch),
                Mode::Overwrite => self.textbuffer.overwrite(*ch),
            },

            _ => (),
        };
    }

    fn reset(&mut self) {
        self.textbuffer = TextBuffer::default();
    }

    fn output(&self) -> String {
        self.textbuffer.to_string_without_cursor()
    }
}
