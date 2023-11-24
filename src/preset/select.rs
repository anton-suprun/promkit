use std::fmt::Display;

use crate::{
    error::Result,
    theme::select::Theme,
    view::{ItemPicker, ItemPickerBuilder, State, TextBuilder, Viewable},
    Prompt,
};

pub struct Select {
    title: TextBuilder,
    item_picker: ItemPickerBuilder,
}

impl Select {
    pub fn new<T: Display, I: IntoIterator<Item = T>>(items: I) -> Self {
        Self {
            title: Default::default(),
            item_picker: ItemPickerBuilder::new(items),
        }
        .theme(Theme::default())
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.title = self.title.style(theme.title_style);
        self.item_picker = self
            .item_picker
            .cursor(theme.cursor)
            .style(theme.item_style)
            .cursor_style(theme.cursor_style);
        self
    }

    pub fn title<T: AsRef<str>>(mut self, text: T) -> Self {
        self.title = self.title.text(text);
        self
    }

    pub fn lines(mut self, lines: usize) -> Self {
        self.item_picker = self.item_picker.lines(lines);
        self
    }

    pub fn prompt(self) -> Result<Prompt<String>> {
        Prompt::try_new(
            vec![self.title.build_state()?, self.item_picker.build_state()?],
            |_, _| Ok(true),
            |viewables: &Vec<Box<dyn Viewable + 'static>>| -> Result<String> {
                Ok(viewables[1]
                    .as_any()
                    .downcast_ref::<State<ItemPicker>>()
                    .unwrap()
                    .after
                    .borrow()
                    .itembox
                    .get())
            },
        )
    }
}
