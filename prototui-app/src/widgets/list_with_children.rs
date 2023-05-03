#![allow(
    dead_code,
    clippy::cast_possible_truncation,
    clippy::too_many_lines,
    clippy::module_name_repetitions
)]
use ratatui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::Style,
    text::Text,
    widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

/// The local state for the list with children widget.
/// Holds the index of selected parent and child. And
/// whether the child list of a service should be
/// expanded.
#[derive(Debug, Clone, Default)]
pub struct ListWithChildrenState {
    offset: usize,
    selected_parent: Option<usize>,
    selected_child: Option<usize>,
    expanded_parent: Option<usize>,
    selection_level: SelectionLevel,
}

/// Whether we are currently selecting in the parent
/// list or in the children list.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum SelectionLevel {
    #[default]
    Parent,
    Children,
}

impl ListWithChildrenState {
    pub fn selected_parent(&self) -> Option<usize> {
        self.selected_parent
    }

    pub fn selected_child(&self) -> Option<usize> {
        self.selected_child
    }

    pub fn select_parent(&mut self, index: Option<usize>) {
        self.selected_parent = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    pub fn select_child(&mut self, index: Option<usize>) {
        self.selected_child = index;
    }

    pub fn expand_parent(&mut self) {
        self.expanded_parent = self.selected_parent();
        self.selection_level = SelectionLevel::Children;
    }

    pub fn collapse_children(&mut self) {
        self.expanded_parent = None;
        self.selected_child = None;
        self.selection_level = SelectionLevel::Parent;
    }

    pub fn selection_level(&self) -> SelectionLevel {
        self.selection_level.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListWithChildrenItem<'a> {
    parent: ListItem<'a>,
    children: Vec<ListItem<'a>>,
}

impl<'a> ListWithChildrenItem<'a> {
    pub fn new<T, U>(content: T, children: U) -> ListWithChildrenItem<'a>
    where
        T: Into<Text<'a>>,
        U: Into<Vec<ListItem<'a>>>,
    {
        ListWithChildrenItem {
            parent: ListItem::new(content),
            children: children.into(),
        }
    }
}

/// A widget to display several items among which one can be selected (optional)
#[derive(Debug, Clone)]
pub struct ListWithChildren<'a> {
    block: Option<Block<'a>>,
    items: Vec<ListWithChildrenItem<'a>>,
    /// Style used as a base style for the widget
    style: Style,
    start_corner: Corner,
    /// Style used to render selected item
    highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
    /// Whether to repeat the highlight symbol for each line of the selected item
    repeat_highlight_symbol: bool,
    /// Style used to render selected sub item
    highlight_sub_style: Style,
}

impl<'a> ListWithChildren<'a> {
    pub fn new<T>(items: T) -> ListWithChildren<'a>
    where
        T: Into<Vec<ListWithChildrenItem<'a>>>,
    {
        ListWithChildren {
            block: None,
            style: Style::default(),
            items: items.into(),
            start_corner: Corner::TopLeft,
            highlight_style: Style::default(),
            highlight_symbol: None,
            repeat_highlight_symbol: false,
            highlight_sub_style: Style::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> ListWithChildren<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> ListWithChildren<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> ListWithChildren<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> ListWithChildren<'a> {
        self.highlight_style = style;
        self
    }

    pub fn highlight_sub_style(mut self, style: Style) -> ListWithChildren<'a> {
        self.highlight_sub_style = style;
        self
    }

    pub fn repeat_highlight_symbol(mut self, repeat: bool) -> ListWithChildren<'a> {
        self.repeat_highlight_symbol = repeat;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> ListWithChildren<'a> {
        self.start_corner = corner;
        self
    }
}

impl<'a> StatefulWidget for ListWithChildren<'a> {
    type State = ListWithChildrenState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.items.is_empty() {
            return;
        }

        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;
        let has_selection = state.selected_parent.is_some();
        for (i, item) in self.items.iter_mut().enumerate() {
            let (x, y) = if self.start_corner == Corner::BottomLeft {
                current_height += item.parent.height() as u16;
                (list_area.left(), list_area.bottom() - current_height)
            } else {
                let pos = (list_area.left(), list_area.top() + current_height);
                current_height += item.parent.height() as u16;
                pos
            };
            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: item.parent.height() as u16,
            };
            let item_style = self.style.patch(item.parent.style);
            buf.set_style(area, item_style);

            let is_selected = state.selected_parent.map_or(false, |s| s == i);
            for (j, line) in item.parent.content.lines.iter().enumerate() {
                // if the item is selected, we need to display the highlight symbol:
                // - either for the first line of the item only,
                // - or for each line of the item if the appropriate option is set
                let symbol = if is_selected && (j == 0 || self.repeat_highlight_symbol) {
                    highlight_symbol
                } else {
                    &blank_symbol
                };
                let (elem_x, max_element_width) = if has_selection {
                    let (elem_x, _) = buf.set_stringn(
                        x,
                        y + j as u16,
                        symbol,
                        list_area.width as usize,
                        item_style,
                    );
                    (elem_x, (list_area.width - (elem_x - x)))
                } else {
                    (x, list_area.width)
                };
                buf.set_spans(elem_x, y + j as u16, line, max_element_width);

                let is_expanded = state.expanded_parent.map_or(false, |s| s == i);

                if is_expanded {
                    for (k, child) in item.children.iter_mut().enumerate() {
                        let (x, y) = if self.start_corner == Corner::BottomLeft {
                            current_height += child.height() as u16;
                            (list_area.left(), list_area.bottom() - current_height)
                        } else {
                            let pos = (list_area.left(), list_area.top() + current_height);
                            current_height += child.height() as u16;
                            pos
                        };
                        let area = Rect {
                            x,
                            y,
                            width: list_area.width,
                            height: child.height() as u16,
                        };
                        let item_style = self.style.patch(item.parent.style);
                        buf.set_style(area, item_style);

                        let is_sub_selected = state.selected_child.map_or(false, |s| s == k);
                        for (m, sub_line) in child.content.lines.iter().enumerate() {
                            // if the item is selected, we need to display the highlight symbol:
                            // - either for the first line of the item only,
                            // - or for each line of the item if the appropriate option is set
                            let symbol =
                                if is_sub_selected && (m == 0 || self.repeat_highlight_symbol) {
                                    highlight_symbol
                                } else {
                                    &blank_symbol
                                };
                            let (elem_x, max_element_width) = if has_selection {
                                let (elem_x, _) = buf.set_stringn(
                                    x,
                                    y + j as u16,
                                    blank_symbol.clone() + symbol,
                                    list_area.width as usize,
                                    item_style,
                                );
                                (elem_x, (list_area.width - (elem_x - x)))
                            } else {
                                (x, list_area.width)
                            };
                            buf.set_spans(elem_x, y + j as u16, sub_line, max_element_width);
                        }

                        // Highlight selected child
                        if is_sub_selected {
                            buf.set_style(area, self.highlight_sub_style);
                        }
                    }
                }
            }

            // Highlight selected parent
            if is_selected {
                buf.set_style(area, self.highlight_style);
            }
        }
    }
}

impl<'a> Widget for ListWithChildren<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListWithChildrenState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    pub content: Text<'a>,
    pub style: Style,
}

impl<'a> ListItem<'a> {
    pub fn new<T>(content: T) -> ListItem<'a>
    where
        T: Into<Text<'a>>,
    {
        ListItem {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> ListItem<'a> {
        self.style = style;
        self
    }

    pub fn height(&self) -> usize {
        self.content.height()
    }

    pub fn width(&self) -> usize {
        self.content.width()
    }
}
