#![allow(dead_code, clippy::too_many_lines, clippy::module_name_repetitions)]
use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::Line, widgets::Widget};
use tui_widget_list::{WidgetItem, WidgetList};

use super::list::ListItem;

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

    pub fn expanded_parent(&self) -> Option<usize> {
        self.expanded_parent
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

#[derive(Clone)]
pub struct ItemWithChildren<'a> {
    element: ListItem<'a>,
    list: WidgetList<'a, ListItem<'a>>,
    height: usize,
}

impl<'a> ItemWithChildren<'a> {
    pub fn new<T, U>(element: T, list: U) -> Self
    where
        T: Into<Line<'a>>,
        U: Into<WidgetList<'a, ListItem<'a>>>,
    {
        ItemWithChildren {
            element: ListItem::new(element),
            list: list.into(),
            height: 1,
        }
    }

    // fn modify_selected(
    //     mut item: WidgetListItem<Self>,
    //     selected: Option<bool>,
    // ) -> WidgetListItem<Self> {
    //     if let Some(selected) = selected {
    //         if selected {
    //             let highlight_style = Style::default()
    //                 .bg(crate::theme::COL_LIST_HIGHLIGHTED_SERVICE_BG)
    //                 .fg(crate::theme::COL_LIST_HIGHLIGHTED_SERVICE_FG);
    //             item.content.element.style = highlight_style;
    //             item.height = 1_u16 + item.content.list.items.len() as u16;
    //             item.content.element = item.content.element.prefix(Some(">>"));
    //         } else {
    //             item.content.element = item.content.element.prefix(Some("  "));
    //         }
    //     }
    //     item
    // }
}

// impl<'a> From<ItemWithChildren<'a>> for WidgetListItem<ItemWithChildren<'a>> {
//     fn from(val: ItemWithChildren<'a>) -> Self {
//         Self::new(val, 1_u16).modify_fn(ItemWithChildren::modify_selected)
//     }
// }

impl WidgetItem for ItemWithChildren<'_> {
    fn height(&self) -> usize {
        self.height
    }

    fn highlighted(&self) -> Self {
        let mut item = self.clone();
        let highlight_style = Style::default()
            .bg(crate::theme::COL_LIST_HIGHLIGHTED_SERVICE_BG)
            .fg(crate::theme::COL_LIST_HIGHLIGHTED_SERVICE_FG);
        item.element.style = highlight_style;
        item.height = 1 + item.list.items.len();
        item
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.width < 1 || area.height < 1 {
            return;
        }
        // Render element
        let (x, y) = (area.left(), area.top());
        let elem_area = Rect::new(x, y, area.width, 1_u16);
        self.element.render(elem_area, buf);

        // Render list
        let mut list = self.list.clone();
        let height = list.items.len() as u16;
        let list_area = Rect::new(x, y + 1_u16, area.width, height);
        list.render(list_area, buf);
    }
}
