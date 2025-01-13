use fltk::{app, draw::*, enums::*, prelude::*, table::*};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ListView {
    table: TableRow,
    data: Rc<RefCell<Vec<Vec<String>>>>,
}

impl Default for ListView {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl ListView {
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

        Self {
            table,
            data: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn set_table(&mut self, rows: i32, cols: i32) {
        self.table.set_rows(rows);
        self.table.set_cols(cols);
        // rows
        self.table.set_row_header(false);
        self.table.set_row_resize(false);
        self.table
            .set_row_height_all((self.table.h() - 40) / self.table.rows());
        // columns
        self.table.set_col_header_height(40);
        self.table.set_col_header(true);
        self.table
            .set_col_width_all(self.table.w() / self.table.cols());
        self.table.set_col_resize(false);

        self.table.resize_callback(|t, _, _, _, _| {
            t.set_col_width_all(t.w() / t.cols());
            t.set_row_height_all((t.h() - 40) / t.rows() + 1);
        });

        let data = self.data.clone();

        self.table
            .draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                TableContext::StartPage => set_font(Font::Helvetica, 14),
                TableContext::ColHeader => {
                    Self::draw_header(&data.borrow()[0][col as usize], x, y, w, h)
                }
                TableContext::Cell => {
                    Self::draw_data(
                        &data.borrow()[row as usize + 1][col as usize],
                        x,
                        y,
                        w,
                        h,
                        t.row_selected(row),
                    );
                }
                _ => (),
            });
    }

    pub fn default_fill() -> Self {
        Self::default().size_of_parent().center_of_parent()
    }

    pub fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, mut cb: F) {
        let mut this = self.clone();
        self.table.set_callback(move |t| {
            if t.callback_context() == TableContext::Cell && app::event() == Event::Push {
                cb(&mut this);
            }
        });
    }

    pub fn callback_row(&self) -> i32 {
        self.table.callback_row() + 1
    }

    pub fn set_data(&mut self, data: &[&[&str]]) {
        assert!(data[0].len() == self.table.cols() as usize);
        *self.data.borrow_mut() = data  
            .iter()
            .map(|s| s.iter().map(|l| l.to_string()).collect::<Vec<_>>())
            .collect();
    }
}
fltk::widget_extends!(ListView, TableRow, table);
