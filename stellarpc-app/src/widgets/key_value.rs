#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions
)]
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};
use tui_widget_list::WidgetItem;

use crate::{commons::editor::TextEditor, theme::COL_WINDOW_BORDER_HIGHLIGHTED_FG};

#[derive(Clone)]
pub struct Tile<'a> {
    editor: TextEditor<'a>,
    style: Style,
    block: Block<'a>,
}

impl<'a> Tile<'a> {
    pub fn new(text: &str) -> Self {
        Self {
            editor: TextEditor::from_str(text),
            style: Style::default(),
            block: Block::default(),
        }
    }

    // pub fn set_text<T: Into<String>>(&mut self, text: T) {
    //     self.editor.set_text_raw(&text.into());
    // }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }

    pub fn into_editor(mut self) -> TextEditor<'a> {
        self.editor.update_style();
        self.editor.set_block(self.block);
        self.editor.set_style(self.style);
        self.editor
    }
}

#[derive(Clone)]
pub struct KeyValue<'a> {
    key: Tile<'a>,
    val: Tile<'a>,
    style: Style,
    block: Option<Block<'a>>,
    selected: usize,
}

impl<'a> Default for KeyValue<'a> {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl<'a> KeyValue<'a> {
    pub fn new(key: &str, val: &str) -> Self {
        Self {
            key: Tile::new(key),
            val: Tile::new(val),
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

    // pub fn insert_key<T: Into<String>>(&mut self, text: T) {
    //     self.key.set_text(text);
    // }
    //
    // pub fn insert_val<T: Into<String>>(&mut self, text: T) {
    //     self.val.set_text(text);
    // }
    //
    // pub fn block(mut self, block: Block<'a>) -> Self {
    //     self.block = Some(block);
    //     self
    // }
}

impl<'a> WidgetItem for KeyValue<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Set the base style
        let area = match self.block.as_ref() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.clone().render(area, buf);
                inner_area
            }
            None => area,
        };
        buf.set_style(area, self.style);

        // Render key
        let (x, y) = (area.left(), area.top());
        let width = (f64::from(area.width - 2_u16) * 0.5) as u16;
        let height = area.height;
        let area = Rect::new(x, y, width, height);
        self.key.clone().into_editor().render(area, buf);

        // Render value
        let x = area.right() + 2_u16;
        let area = Rect::new(x, y, width, height);
        self.val.clone().into_editor().render(area, buf);
    }

    fn height(&self) -> usize {
        if self.block.is_some() {
            5
        } else {
            3
        }
    }

    fn highlighted(&self) -> Self {
        let mut item: KeyValue = self.clone();
        let highlighted = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(Style::default().fg(COL_WINDOW_BORDER_HIGHLIGHTED_FG));
        if item.is_key_selected() {
            item = item.block_key(highlighted.clone().title("Key"));
        }
        if item.is_val_selected() {
            item = item.block_val(highlighted.title("Value"));
        }
        // Makes sure that the editor changes in insert mode
        item.key.editor.update_style();
        item.val.editor.update_style();
        item
    }
}
