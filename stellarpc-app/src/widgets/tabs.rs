use ratatui::prelude::Buffer;
use ratatui::prelude::Rect;
use ratatui::widgets::Widget;
use ratatui::{
    style::{Style, Styled},
    symbols,
    text::{Line, Span},
    widgets::Block,
};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ActivatableTabs<'a> {
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: Vec<Line<'a>>,
    /// Whether a tab should be displayed as diabled.
    active: Option<Vec<bool>>,
    /// The index of the selected tabs
    selected: usize,
    /// The style of inactive tabs
    style: Style,
    /// The style of inactive and disabled tabs
    active_style: Style,
    /// Style to apply to the selected item
    highlight_style: Style,
    /// Tab divider
    divider: Span<'a>,
}

impl<'a> ActivatableTabs<'a> {
    pub fn new<T>(titles: Vec<T>) -> ActivatableTabs<'a>
    where
        T: Into<Line<'a>>,
    {
        ActivatableTabs {
            block: None,
            titles: titles.into_iter().map(Into::into).collect(),
            active: None,
            selected: 0,
            style: Style::default(),
            active_style: Style::default(),
            highlight_style: Style::default(),
            divider: Span::raw(symbols::line::VERTICAL),
        }
    }

    /// Surrounds the `Tabs` with a [`Block`].
    pub fn block(mut self, block: Block<'a>) -> ActivatableTabs<'a> {
        self.block = Some(block);
        self
    }

    /// Defines which elements are active.
    pub fn active(mut self, active: Vec<bool>) -> ActivatableTabs<'a> {
        assert!(active.len() == self.titles.len());
        self.active = Some(active);
        self
    }

    /// Sets the selected tab.
    ///
    /// The first tab has index 0 (this is also the default index).  
    /// The selected tab can have a different style with [`Tabs::highlight_style`].
    pub fn select(mut self, selected: usize) -> ActivatableTabs<'a> {
        self.selected = selected;
        self
    }

    /// Sets the style of the tabs.
    ///
    /// This will set the given style on the entire render area.
    /// More precise style can be applied to the titles by styling the ones given to [`Tabs::new`].
    /// The selected tab can be styled differently using [`Tabs::highlight_style`].
    pub fn style(mut self, style: Style) -> ActivatableTabs<'a> {
        self.style = style;
        self
    }

    /// Sets the style for the highlighted tab.
    ///
    /// Highlighted tab can be selected with [`Tabs::select`].
    pub fn highlight_style(mut self, style: Style) -> ActivatableTabs<'a> {
        self.highlight_style = style;
        self
    }

    /// Sets the style for the actice but not hightlighted tab.
    ///
    /// Highlighted tab can be selected with [`Tabs::select`].
    pub fn active_style(mut self, style: Style) -> ActivatableTabs<'a> {
        self.active_style = style;
        self
    }

    /// Sets the string to use as tab divider.
    ///
    /// By default, the divider is a pipe (`|`).
    ///
    /// # Examples
    ///
    /// Use a dot (`•`) as separator.
    /// ```
    /// # use ratatui::{prelude::*, widgets::Tabs, symbols};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).divider(symbols::DOT);
    /// ```
    /// Use dash (`-`) as separator.
    /// ```
    /// # use ratatui::{prelude::*, widgets::Tabs, symbols};
    /// let tabs = Tabs::new(vec!["Tab 1", "Tab 2"]).divider("-");
    /// ```
    pub fn divider<T>(mut self, divider: T) -> ActivatableTabs<'a>
    where
        T: Into<Span<'a>>,
    {
        self.divider = divider.into();
        self
    }
}

impl<'a> Styled for ActivatableTabs<'a> {
    type Item = ActivatableTabs<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

impl<'a> Widget for ActivatableTabs<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let tabs_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if tabs_area.height < 1 {
            return;
        }

        let mut x = tabs_area.left();
        let titles_length = self.titles.len();
        for (i, title) in self.titles.into_iter().enumerate() {
            let last_title = titles_length - 1 == i;
            x = x.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 {
                break;
            }
            let pos = buf.set_line(x, tabs_area.top(), &title, remaining_width);
            let tab_area = Rect {
                x,
                y: tabs_area.top(),
                width: pos.0.saturating_sub(x),
                height: 1,
            };
            if let Some(ref active) = self.active {
                if active[i] {
                    buf.set_style(tab_area, self.active_style);
                }
            }
            if i == self.selected {
                buf.set_style(tab_area, self.highlight_style);
            }
            x = pos.0.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 || last_title {
                break;
            }
            let pos = buf.set_span(x, tabs_area.top(), &self.divider, remaining_width);
            x = pos.0;
        }
    }
}
