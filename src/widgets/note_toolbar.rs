use crate::widgets::note_editor::FormatAction;

use adw::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

const FONT_SIZES: &[&str] = &["14", "16", "18", "20", "22", "24", "26", "28", "32", "36", "48"];
const DEFAULT_FONT_SIZE_INDEX: u32 = 6; // 26px

type FormatCallback = Box<dyn Fn(FormatAction)>;
type FontSizeCallback = Box<dyn Fn(u32)>;

#[derive(Clone)]
pub struct NoteToolbar {
    scrolled: gtk::ScrolledWindow,
    container: gtk::Box,
    format_callbacks: Rc<RefCell<Vec<FormatCallback>>>,
    font_size_callbacks: Rc<RefCell<Vec<FontSizeCallback>>>,
}

impl NoteToolbar {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        container.set_margin_start(4);
        container.set_margin_end(4);
        container.set_margin_top(2);
        container.set_margin_bottom(2);

        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Never)
            .propagate_natural_width(false)
            .propagate_natural_height(true)
            .has_frame(false)
            .child(&container)
            .build();

        scrolled.add_css_class("formatting-toolbar");

        let format_callbacks = Rc::new(RefCell::new(Vec::new()));
        let font_size_callbacks = Rc::new(RefCell::new(Vec::new()));

        let mut toolbar = Self {
            scrolled,
            container,
            format_callbacks,
            font_size_callbacks,
        };

        toolbar.build_toolbar();
        toolbar
    }

    pub fn widget(&self) -> &gtk::ScrolledWindow {
        &self.scrolled
    }

    fn build_toolbar(&mut self) {
        self.add_format_button("format-text-bold-symbolic", "Bold (Ctrl+B)", FormatAction::Bold);
        self.add_format_button("format-text-italic-symbolic", "Italic (Ctrl+I)", FormatAction::Italic);
        self.add_format_button("format-text-underline-symbolic", "Underline (Ctrl+U)", FormatAction::Underline);
        self.add_format_button("format-text-strikethrough-symbolic", "Strikethrough", FormatAction::Strikethrough);

        let sep = gtk::Separator::new(gtk::Orientation::Vertical);
        sep.set_margin_start(4);
        sep.set_margin_end(4);
        self.container.append(&sep);

        self.add_format_button("view-list-symbolic", "Bullet List", FormatAction::BulletList);
        self.add_format_button("view-list-ordered-symbolic", "Numbered List", FormatAction::NumberedList);
        self.add_format_button("object-select-symbolic", "Checklist", FormatAction::Checklist);

        let sep2 = gtk::Separator::new(gtk::Orientation::Vertical);
        sep2.set_margin_start(4);
        sep2.set_margin_end(4);
        self.container.append(&sep2);

        self.add_heading_button();

        let sep3 = gtk::Separator::new(gtk::Orientation::Vertical);
        sep3.set_margin_start(4);
        sep3.set_margin_end(4);
        self.container.append(&sep3);

        self.add_font_size_dropdown();
    }

    fn add_format_button(&self, icon_name: &str, tooltip: &str, action: FormatAction) {
        let button = gtk::Button::builder()
            .icon_name(icon_name)
            .tooltip_text(tooltip)
            .has_frame(false)
            .build();

        let callbacks = self.format_callbacks.clone();
        button.connect_clicked(move |_| {
            for cb in callbacks.borrow().iter() {
                cb(action);
            }
        });

        self.container.append(&button);
    }

    fn add_heading_button(&self) {
        let button = gtk::Button::builder()
            .label("H")
            .tooltip_text("Heading")
            .has_frame(false)
            .build();
        button.add_css_class("toolbar-heading-btn");

        let callbacks = self.format_callbacks.clone();
        button.connect_clicked(move |_| {
            for cb in callbacks.borrow().iter() {
                cb(FormatAction::Heading);
            }
        });

        self.container.append(&button);
    }

    fn add_font_size_dropdown(&self) {
        let string_list = gtk::StringList::new(FONT_SIZES);
        let dropdown = gtk::DropDown::new(Some(string_list), gtk::Expression::NONE);
        dropdown.set_selected(DEFAULT_FONT_SIZE_INDEX);
        dropdown.add_css_class("font-size-dropdown");
        dropdown.set_tooltip_text(Some("Font Size"));

        let fs_cbs = self.font_size_callbacks.clone();
        dropdown.connect_selected_notify(move |dd| {
            let idx = dd.selected() as usize;
            if let Some(&size_str) = FONT_SIZES.get(idx) {
                if let Ok(size) = size_str.parse::<u32>() {
                    for cb in fs_cbs.borrow().iter() {
                        cb(size);
                    }
                }
            }
        });

        self.container.append(&dropdown);
    }

    pub fn connect_format_action<F: Fn(FormatAction) + 'static>(&self, f: F) {
        self.format_callbacks.borrow_mut().push(Box::new(f));
    }

    pub fn connect_font_size_changed<F: Fn(u32) + 'static>(&self, f: F) {
        self.font_size_callbacks.borrow_mut().push(Box::new(f));
    }
}

impl Default for NoteToolbar {
    fn default() -> Self {
        Self::new()
    }
}

