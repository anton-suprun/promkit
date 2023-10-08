use anyhow::Result;

use crate::{
    crossterm::style::ContentStyle,
    editor::{text_editor::TextEditor, Mode},
    history::History,
    suggest::Suggest,
    text_buffer::TextBuffer,
};

pub struct TextEditorBuilder {
    suggest: Suggest,
    label: String,
    label_style: ContentStyle,
    style: ContentStyle,
    cursor_style: ContentStyle,
    mode: Mode,
    mask: Option<char>,
}

impl Default for TextEditorBuilder {
    fn default() -> Self {
        Self {
            suggest: Suggest::default(),
            label: String::from("❯❯ "),
            label_style: ContentStyle::new(),
            style: ContentStyle::new(),
            cursor_style: ContentStyle::new(),
            mode: Mode::Insert,
            mask: None,
        }
    }
}

impl TextEditorBuilder {
    pub fn suggest(mut self, suggest: Suggest) -> Self {
        self.suggest = suggest;
        self
    }

    pub fn label<T: AsRef<str>>(mut self, label: T) -> Self {
        self.label = label.as_ref().to_string();
        self
    }

    pub fn label_style(mut self, style: ContentStyle) -> Self {
        self.label_style = style;
        self
    }

    pub fn style(mut self, style: ContentStyle) -> Self {
        self.style = style;
        self
    }

    pub fn cursor_style(mut self, style: ContentStyle) -> Self {
        self.cursor_style = style;
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn mask(mut self, mask: char) -> Self {
        self.mask = Some(mask);
        self
    }

    pub fn build(self) -> Result<Box<TextEditor>> {
        Ok(Box::new(TextEditor {
            textbuffer: TextBuffer::default(),
            history: History::default(),
            suggest: self.suggest,
            label: self.label,
            label_style: self.label_style,
            style: self.style,
            cursor_style: self.cursor_style,
            mode: self.mode,
            mask: self.mask,
        }))
    }
}
