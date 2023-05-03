use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};
use tui_widget_list::WidgetListItem;

use crate::{commons::editor::TextEditor, theme::COL_WINDOW_BORDER_HIGHLIGHTED_FG};

#[derive(Clone)]
pub struct Tile<'a> {
    editor: TextEditor<'a>,
    style: Style,
    block: Block<'a>,
}

impl<'a> Tile<'a> {
    pub fn new() -> Self {
        Self {
            editor: TextEditor::new(),
            style: Style::default(),
            block: Block::default(),
        }
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.editor.set_text_raw(&text.into());
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }

    pub fn into_editor(mut self) -> TextEditor<'a> {
        self.editor.set_style_default();
        self.editor.set_block(self.block);
        self.editor.set_style(self.style);
        self.editor
    }
}

#[derive(Clone)]
pub struct KeyValueWidget<'a> {
    key: Tile<'a>,
    val: Tile<'a>,
    style: Style,
    block: Option<Block<'a>>,
    selected: usize,
}

impl<'a> Default for KeyValueWidget<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> KeyValueWidget<'a> {
    pub fn new() -> Self {
        Self {
            key: Tile::new(),
            val: Tile::new(),
            style: Style::default(),
            block: None,
            selected: 0,
        }
        .set_default_style()
    }

    fn set_default_style(self) -> Self {
        self.block_key(Block::default().borders(Borders::ALL).title("Key"))
            .block_val(Block::default().borders(Borders::ALL).title("Value"))
            .style_key(Style::default().bg(Color::Black).fg(Color::White))
            .style_val(Style::default().bg(Color::Black).fg(Color::White))
    }

    pub fn get_key(&self) -> &'_ TextEditor<'a> {
        &self.key.editor
    }

    pub fn get_val(&self) -> &'_ TextEditor<'a> {
        &self.val.editor
    }

    pub fn get_key_mut(&mut self) -> &'_ mut TextEditor<'a> {
        &mut self.key.editor
    }

    pub fn get_val_mut(&mut self) -> &'_ mut TextEditor<'a> {
        &mut self.val.editor
    }

    pub fn get_selected(&self) -> &'_ TextEditor<'a> {
        if self.is_key_selected() {
            self.get_key()
        } else {
            self.get_val()
        }
    }

    pub fn get_selected_mut(&mut self) -> &'_ mut TextEditor<'a> {
        if self.is_key_selected() {
            self.get_key_mut()
        } else {
            self.get_val_mut()
        }
    }

    pub fn select_key(&mut self) {
        self.selected = 0;
    }

    pub fn select_val(&mut self) {
        self.selected = 1;
    }

    pub fn is_key_selected(&self) -> bool {
        self.selected == 0
    }

    pub fn is_val_selected(&self) -> bool {
        self.selected == 1
    }

    pub fn style_key(mut self, style: Style) -> Self {
        self.key = self.key.style(style);
        self
    }

    pub fn style_val(mut self, style: Style) -> Self {
        self.val = self.val.style(style);
        self
    }

    pub fn block_key(mut self, block: Block<'a>) -> Self {
        self.key = self.key.block(block);
        self
    }

    pub fn block_val(mut self, block: Block<'a>) -> Self {
        self.val = self.val.block(block);
        self
    }

    pub fn insert_key<T: Into<String>>(&mut self, text: T) {
        self.key.set_text(text);
    }

    pub fn insert_val<T: Into<String>>(&mut self, text: T) {
        self.val.set_text(text);
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    // Render the item differently depending on the selection state
    fn modify_fn(mut item: WidgetListItem<Self>, selected: Option<bool>) -> WidgetListItem<Self> {
        if let Some(selected) = selected {
            if selected {
                let highlighted = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .style(Style::default().fg(COL_WINDOW_BORDER_HIGHLIGHTED_FG));
                if item.content.is_key_selected() {
                    item.content = item.content.block_key(highlighted.clone().title("Key"));
                }
                if item.content.is_val_selected() {
                    item.content = item.content.block_val(highlighted.title("Value"));
                }
            }
        }
        item
    }
}

impl<'a> From<KeyValueWidget<'a>> for WidgetListItem<KeyValueWidget<'a>> {
    fn from(val: KeyValueWidget<'a>) -> Self {
        let height = if val.block.is_some() { 5_u16 } else { 3_u16 };
        Self::new(val, height).modify_fn(KeyValueWidget::modify_fn)
    }
}

impl<'a> Widget for KeyValueWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Set the base style
        let area = match self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        buf.set_style(area, self.style);

        // Render key
        let (x, y) = (area.left(), area.top());
        let width = ((area.width - 2_u16) as f64 * 0.5) as u16;
        let height = area.height;
        let area = Rect::new(x, y, width, height);
        self.key.into_editor().widget().render(area, buf);

        // Render value
        let x = area.right() + 2_u16;
        let area = Rect::new(x, y, width, height);
        self.val.into_editor().widget().render(area, buf);
    }
}
