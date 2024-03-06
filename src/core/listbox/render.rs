use std::any::Any;

use crate::{
    crossterm::{event::Event, style::ContentStyle},
    grapheme::{trim, Graphemes, StyledGraphemes},
    keymap::KeymapManager,
    pane::Pane,
    AsAny, EventAction, Result,
};

use super::Listbox;

/// Represents a renderer for the `Listbox` component,
/// capable of visualizing a list of items in a pane.
/// It supports a custom symbol for the selected line,
/// styles for active and inactive items,
/// and a configurable number of lines for rendering.
#[derive(Clone)]
pub struct Renderer {
    /// The `Listbox` component to be rendered.
    pub listbox: Listbox,

    pub keymap: KeymapManager<Self>,

    /// Symbol for the selected line.
    pub cursor: String,

    /// Style for the selected line.
    pub active_item_style: ContentStyle,
    /// Style for un-selected lines.
    pub inactive_item_style: ContentStyle,

    /// Number of lines available for rendering.
    pub lines: Option<usize>,
}

impl crate::Renderer for Renderer {
    fn make_pane(&self, width: u16) -> Pane {
        let matrix = self
            .listbox
            .items()
            .iter()
            .enumerate()
            .map(|(i, item)| {
                if i == self.listbox.position() {
                    StyledGraphemes::from_str(
                        format!("{}{}", self.cursor, item),
                        self.active_item_style,
                    )
                } else {
                    StyledGraphemes::from_str(
                        format!(
                            "{}{}",
                            " ".repeat(Graphemes::from(self.cursor.clone()).widths()),
                            item
                        ),
                        self.inactive_item_style,
                    )
                }
            })
            .collect::<Vec<StyledGraphemes>>();

        let trimed = matrix.iter().map(|row| trim(width as usize, row)).collect();

        Pane::new(trimed, self.listbox.position(), self.lines)
    }

    fn handle_event(&mut self, event: &Event) -> Result<EventAction> {
        (self.keymap.get())(self, event)
    }

    fn postrun(&mut self) {
        self.listbox.move_to_head()
    }
}

impl AsAny for Renderer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
