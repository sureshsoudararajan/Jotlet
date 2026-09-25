use crate::models::note_color::NoteColor;

use adw::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

type ColorCallback = Box<dyn Fn(NoteColor)>;

#[derive(Clone)]
pub struct ColorPicker {
    popover: gtk::Popover,
    selected_callbacks: Rc<RefCell<Vec<ColorCallback>>>,
}

impl ColorPicker {
    pub fn new() -> Self {
        let popover = gtk::Popover::new();
        let grid = gtk::Grid::builder()
            .column_spacing(8)
            .row_spacing(8)
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .build();

        let selected_callbacks: Rc<RefCell<Vec<ColorCallback>>> =
            Rc::new(RefCell::new(Vec::new()));
        let colors = NoteColor::all();
        let cols = 3;

        for (i, color) in colors.iter().enumerate() {
            let row = (i / cols) as i32;
            let col = (i % cols) as i32;

            let swatch_hex = color.swatch_color().to_string();

            let drawing = gtk::DrawingArea::builder()
                .width_request(32)
                .height_request(32)
                .tooltip_text(color.display_name())
                .build();

            let hex = swatch_hex.clone();
            drawing.set_draw_func(move |_, cr, w, h| {
                if let Ok(rgba) = gtk::gdk::RGBA::parse(&hex) {
                    cr.set_source_rgba(
                        rgba.red() as f64,
                        rgba.green() as f64,
                        rgba.blue() as f64,
                        rgba.alpha() as f64,
                    );
                } else {
                    cr.set_source_rgba(0.9, 0.9, 0.9, 1.0);
                }
                cr.arc(
                    w as f64 / 2.0,
                    h as f64 / 2.0,
                    14.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                let _ = cr.fill();
            });

            let button = gtk::Button::builder()
                .child(&drawing)
                .has_frame(false)
                .tooltip_text(color.display_name())
                .build();
            button.add_css_class("circular");

            let color_val = *color;
            let cbs = selected_callbacks.clone();
            let pop = popover.clone();
            button.connect_clicked(move |_| {
                for cb in cbs.borrow().iter() {
                    cb(color_val);
                }
                pop.popdown();
            });

            grid.attach(&button, col, row, 1, 1);
        }

        popover.set_child(Some(&grid));

        Self {
            popover,
            selected_callbacks,
        }
    }

    pub fn popover(&self) -> &gtk::Popover {
        &self.popover
    }

    pub fn connect_color_selected<F: Fn(NoteColor) + 'static>(&self, f: F) {
        self.selected_callbacks.borrow_mut().push(Box::new(f));
    }
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self::new()
    }
}
