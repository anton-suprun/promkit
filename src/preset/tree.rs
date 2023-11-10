use crate::{
    components::{Component, State, TextBuilder, TreeViewer, TreeViewerBuilder},
    error::Result,
    tree::Node,
    Prompt,
};

pub struct Tree {
    title: TextBuilder,
    tree_viewer: TreeViewerBuilder,
}

impl Tree {
    pub fn new(root: Node) -> Self {
        Self {
            title: Default::default(),
            tree_viewer: TreeViewerBuilder::new(root),
        }
        // .theme(Theme::default())
    }

    // pub fn theme(mut self, theme: Theme) -> Self {
    //     self.title = self.title.style(theme.title_style);
    //     self.item_picker = self
    //         .item_picker
    //         .cursor(theme.cursor)
    //         .style(theme.item_style)
    //         .cursor_style(theme.cursor_style);
    //     self
    // }

    pub fn title<T: AsRef<str>>(mut self, text: T) -> Self {
        self.title = self.title.text(text);
        self
    }

    pub fn lines(mut self, lines: usize) -> Self {
        self.tree_viewer = self.tree_viewer.lines(lines);
        self
    }

    pub fn prompt(self) -> Result<Prompt<String>> {
        Prompt::try_new(
            vec![self.title.build_state()?, self.tree_viewer.build_state()?],
            |_, _| Ok(true),
            |components: &Vec<Box<dyn Component + 'static>>| -> Result<String> {
                Ok(components[1]
                    .as_any()
                    .downcast_ref::<State<TreeViewer>>()
                    .unwrap()
                    .after
                    .borrow()
                    .tree
                    .get())
            },
        )
    }
}
