use fltk::{app, draw::*, enums::*, prelude::*, table::*};
use std::cell::RefCell;
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub enum ListViewContext {
    Cell,
    ColHeader,
    Other,
}

#[derive(Clone, Debug)]
pub struct ListView<const C: usize, const R: usize> {
    table: TableRow,
    headers: Rc<RefCell<[&'static str; C]>>,
    data: Rc<RefCell<[[&'static str; C]; R]>>,
}

impl<const C: usize, const R: usize> Default for ListView<C, R> {
    fn default() -> Self {
        ListView::<C, R>::new(0, 0, 0, 0, None)
    }
}

impl<const C: usize, const R: usize> ListView<C, R> {
    fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
        push_clip(x, y, w, h);
        draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
        set_draw_color(Color::Black);
        set_font(Font::Helvetica, 14);
        draw_text2(txt, x, y, w, h, Align::Center);
        pop_clip();
    }

    // The selected flag sets the color of the cell to a grayish color, otherwise white
    fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
        push_clip(x, y, w, h);
        if selected {
            set_draw_color(Color::from_u32(0xB4D5FE));
        } else {
            set_draw_color(Color::White);
        }
        draw_rectf(x, y, w, h);
        set_draw_color(Color::Gray0);
        set_font(Font::Helvetica, 14);
        draw_text2(txt, x, y, w, h, Align::Center);
        pop_clip();
    }

    pub fn new<L: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: L) -> Self {
        let mut table = TableRow::new(x, y, w, h, label);
        table.set_type(TableRowSelectMode::Single);
        table.set_scrollbar_size(-1);
        table.end();
        table.set_rows(R as i32);
        table.set_cols(C as i32);
        // rows
        table.set_row_header(false);
        table.set_row_resize(false);
        table.set_row_height_all((table.h() - 40) / table.rows());
        // columns
        table.set_col_header_height(40);
        table.set_col_header(true);
        table.set_col_width_all(table.w() / table.cols());
        table.set_col_resize(false);

        table.resize_callback(|t, _, _, _, _| {
            t.set_col_width_all(t.w() / t.cols());
            t.set_row_height_all((t.h() - 40) / t.rows() + 1);
        });

        let headers = Rc::new(RefCell::new([""; C]));
        let data = Rc::new(RefCell::new([[""; C]; R]));

        table.draw_cell({
            let data = data.clone();
            let headers = headers.clone();
            move |t, ctx, row, col, x, y, w, h| match ctx {
                TableContext::StartPage => set_font(Font::Helvetica, 14),
                TableContext::ColHeader => {
                    Self::draw_header(headers.borrow()[col as usize], x, y, w, h)
                }
                TableContext::Cell => {
                    Self::draw_data(
                        data.borrow()[row as usize][col as usize],
                        x,
                        y,
                        w,
                        h,
                        t.row_selected(row),
                    );
                }
                _ => (),
            }
        });

        Self {
            table,
            headers,
            data,
        }
    }

    pub fn default_fill() -> Self {
        ListView::<C, R>::default()
            .size_of_parent()
            .center_of_parent()
    }

    pub fn set_callback<F: FnMut(&mut Self, ListViewContext) + 'static>(&mut self, mut cb: F) {
        let mut this = self.clone();
        self.table.set_callback(move |t| {
            let ctx = t.callback_context();
            if app::event() == Event::Push {
                cb(
                    &mut this,
                    match ctx {
                        TableContext::Cell => ListViewContext::Cell,
                        TableContext::ColHeader => ListViewContext::ColHeader,
                        _ => ListViewContext::Other,
                    },
                );
            }
        });
    }

    pub fn set_data(&mut self, headers: [&'static str; C], data: [[&'static str; C]; R]) {
        *self.headers.borrow_mut() = headers;
        *self.data.borrow_mut() = data;
    }

    /// Initialize to position x, y
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        let w = self.w();
        let h = self.h();
        self.resize(x, y, w, h);
        self
    }

    /// Initialize to size width, height
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        let x = self.x();
        let y = self.y();
        let w = self.width();
        let h = self.height();
        if w == 0 || h == 0 {
            self.widget_resize(x, y, width, height);
        } else {
            self.resize(x, y, width, height);
        }
        self
    }

    /// Initialize with a label
    pub fn with_label(mut self, title: &str) -> Self {
        self.set_label(title);
        self
    }

    /// Initialize center of another widget
    pub fn center_of<W: WidgetExt>(mut self, w: &W) -> Self {
        debug_assert!(
            w.width() != 0 && w.height() != 0,
            "center_of requires the size of the widget to be known!"
        );
        let sw = self.width() as f64;
        let sh = self.height() as f64;
        let ww = w.width() as f64;
        let wh = w.height() as f64;
        let sx = (ww - sw) / 2.0;
        let sy = (wh - sh) / 2.0;
        let wx = if w.as_window().is_some() { 0 } else { w.x() };
        let wy = if w.as_window().is_some() { 0 } else { w.y() };
        self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
        self.redraw();
        self
    }

    /// Initialize center of another widget on the x axis
    pub fn center_x<W: WidgetExt>(mut self, w: &W) -> Self {
        debug_assert!(
            w.width() != 0 && w.height() != 0,
            "center_of requires the size of the widget to be known!"
        );
        let sw = self.width() as f64;
        let sh = self.height() as f64;
        let ww = w.width() as f64;
        let sx = (ww - sw) / 2.0;
        let sy = self.y();
        let wx = if w.as_window().is_some() { 0 } else { w.x() };
        self.resize(sx as i32 + wx, sy, sw as i32, sh as i32);
        self.redraw();
        self
    }

    /// Initialize center of another widget on the y axis
    pub fn center_y<W: WidgetExt>(mut self, w: &W) -> Self {
        debug_assert!(
            w.width() != 0 && w.height() != 0,
            "center_of requires the size of the widget to be known!"
        );
        let sw = self.width() as f64;
        let sh = self.height() as f64;
        let wh = w.height() as f64;
        let sx = self.x();
        let sy = (wh - sh) / 2.0;
        let wy = if w.as_window().is_some() { 0 } else { w.y() };
        self.resize(sx, sy as i32 + wy, sw as i32, sh as i32);
        self.redraw();
        self
    }

    /// Initialize center of parent
    pub fn center_of_parent(mut self) -> Self {
        if let Some(w) = self.parent() {
            debug_assert!(
                w.width() != 0 && w.height() != 0,
                "center_of requires the size of the widget to be known!"
            );
            let sw = self.width() as f64;
            let sh = self.height() as f64;
            let ww = w.width() as f64;
            let wh = w.height() as f64;
            let sx = (ww - sw) / 2.0;
            let sy = (wh - sh) / 2.0;
            let wx = if w.as_window().is_some() { 0 } else { w.x() };
            let wy = if w.as_window().is_some() { 0 } else { w.y() };
            self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
            self.redraw();
        }
        self
    }

    /// Initialize to the size of another widget
    pub fn size_of<W: WidgetExt>(mut self, w: &W) -> Self {
        debug_assert!(
            w.width() != 0 && w.height() != 0,
            "size_of requires the size of the widget to be known!"
        );
        let x = self.x();
        let y = self.y();
        self.resize(x, y, w.width(), w.height());
        self
    }

    /// Initialize to the size of the parent
    pub fn size_of_parent(mut self) -> Self {
        if let Some(parent) = self.parent() {
            let w = parent.width();
            let h = parent.height();
            let x = self.x();
            let y = self.y();
            self.resize(x, y, w, h);
        }
        self
    }
}

impl<const C: usize, const R: usize> Deref for ListView<C, R> {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<const C: usize, const R: usize> DerefMut for ListView<C, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}
