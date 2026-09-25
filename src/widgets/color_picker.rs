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
        let cols = 5;

        for (i, color) in colors.iter().enumerate() {
            let row = (i / cols) as i32;
            let col = (i % cols) as i32;

            let button = gtk::Button::builder()
                .has_frame(false)
                .tooltip_text(color.display_name())
                .width_request(32)
                .height_request(32)
                .build();
            button.add_css_class("circular");
            button.add_css_class("color-swatch-btn");
            button.add_css_class(&format!("swatch-color-{}", color));

            if *color == NoteColor::Wallpaper {
                let icon = gtk::Image::from_icon_name("preferences-desktop-wallpaper-symbolic");
                icon.set_pixel_size(16);
                button.set_child(Some(&icon));
            }

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
