use crate::widgets::note_editor::FormatAction;

use adw::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

const FONT_SIZES: &[&str] = &["14", "16", "18", "20", "22", "24", "26", "28", "32", "36", "48"];
const DEFAULT_FONT_SIZE_INDEX: u32 = 6; // 26px

pub const BASE_FONT_FAMILIES: &[&str] = &[
    "Default",
    "Adwaita Sans",
    "Adwaita Mono",
    "JetBrainsMono Nerd Font",
    "DejaVu Sans",
    "DejaVu Serif",
    "Liberation Sans",
    "Liberation Serif",
    "Noto Sans",
    "Noto Serif",
];

type FormatCallback = Box<dyn Fn(FormatAction)>;
type FontSizeCallback = Box<dyn Fn(u32)>;
type FontFamilyCallback = Box<dyn Fn(String)>;

#[derive(Clone)]
pub struct NoteToolbar {
    scrolled: gtk::ScrolledWindow,
    container: gtk::Box,
    format_callbacks: Rc<RefCell<Vec<FormatCallback>>>,
    font_size_callbacks: Rc<RefCell<Vec<FontSizeCallback>>>,
    font_family_callbacks: Rc<RefCell<Vec<FontFamilyCallback>>>,
    family_dropdown: gtk::DropDown,
    family_string_list: gtk::StringList,
    size_dropdown: gtk::DropDown,
    font_dialog_button: gtk::FontDialogButton,
    is_updating: Rc<RefCell<bool>>,
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
        let font_family_callbacks = Rc::new(RefCell::new(Vec::new()));
        let is_updating = Rc::new(RefCell::new(false));

        let family_string_list = gtk::StringList::new(BASE_FONT_FAMILIES);
        let family_dropdown =
            gtk::DropDown::new(Some(family_string_list.clone()), gtk::Expression::NONE);
        family_dropdown.set_selected(0);
        family_dropdown.add_css_class("font-family-dropdown");
        family_dropdown.set_tooltip_text(Some("Font Family"));

        let font_dialog = gtk::FontDialog::new();
        let font_dialog_button = gtk::FontDialogButton::new(Some(font_dialog));
        font_dialog_button.set_use_font(true);
        font_dialog_button.set_use_size(false);
        font_dialog_button.add_css_class("flat");
        font_dialog_button.add_css_class("font-picker-btn");
        font_dialog_button.set_tooltip_text(Some("Browse all system fonts"));

        let size_string_list = gtk::StringList::new(FONT_SIZES);
        let size_dropdown =
            gtk::DropDown::new(Some(size_string_list), gtk::Expression::NONE);
        size_dropdown.set_selected(DEFAULT_FONT_SIZE_INDEX);
        size_dropdown.add_css_class("font-size-dropdown");
        size_dropdown.set_tooltip_text(Some("Font Size"));

        let mut toolbar = Self {
            scrolled,
            container,
            format_callbacks,
            font_size_callbacks,
            font_family_callbacks,
            family_dropdown,
            family_string_list,
            size_dropdown,
            font_dialog_button,
            is_updating,
        };

        toolbar.build_toolbar();
        toolbar.wire_font_controls();
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

        self.container.append(&self.family_dropdown);
        self.container.append(&self.font_dialog_button);
        self.container.append(&self.size_dropdown);
    }

    fn wire_font_controls(&self) {
        // Size dropdown notify
        let fs_cbs = self.font_size_callbacks.clone();
        let updating = self.is_updating.clone();
        self.size_dropdown.connect_selected_notify(move |dd| {
            if *updating.borrow() {
                return;
            }
            let idx = dd.selected() as usize;
            if let Some(&size_str) = FONT_SIZES.get(idx) {
                if let Ok(size) = size_str.parse::<u32>() {
                    for cb in fs_cbs.borrow().iter() {
                        cb(size);
                    }
                }
            }
        });

        // Family dropdown notify
        let fam_cbs = self.font_family_callbacks.clone();
        let sl_clone = self.family_string_list.clone();
        let updating_fam = self.is_updating.clone();
        let btn_clone = self.font_dialog_button.clone();
        self.family_dropdown.connect_selected_notify(move |dd| {
            if *updating_fam.borrow() {
                return;
            }
            let idx = dd.selected();
            if let Some(item) = sl_clone.string(idx) {
                let family_name = item.to_string();
                if family_name != "Default" {
                    let desc = gtk::pango::FontDescription::from_string(&family_name);
                    btn_clone.set_font_desc(&desc);
                }
                for cb in fam_cbs.borrow().iter() {
                    cb(family_name.clone());
                }
            }
        });

        // Font dialog button notify
        let fam_cbs2 = self.font_family_callbacks.clone();
        let sl_clone2 = self.family_string_list.clone();
        let dd_clone2 = self.family_dropdown.clone();
        let updating_btn = self.is_updating.clone();
        self.font_dialog_button.connect_font_desc_notify(move |btn| {
            if *updating_btn.borrow() {
                return;
            }
            if let Some(desc) = btn.font_desc() {
                if let Some(family) = desc.family() {
                    let family_str = family.to_string();
                    *updating_btn.borrow_mut() = true;
                    let mut found_idx = None;
                    for i in 0..sl_clone2.n_items() {
                        if let Some(s) = sl_clone2.string(i) {
                            if s.as_str() == family_str.as_str() {
                                found_idx = Some(i);
                                break;
                            }
                        }
                    }
                    let idx = match found_idx {
                        Some(i) => i,
                        None => {
                            sl_clone2.append(&family_str);
                            sl_clone2.n_items() - 1
                        }
                    };
                    dd_clone2.set_selected(idx);
                    *updating_btn.borrow_mut() = false;

                    for cb in fam_cbs2.borrow().iter() {
                        cb(family_str.clone());
                    }
                }
            }
        });
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

    pub fn set_font_family(&self, family: &str) {
        *self.is_updating.borrow_mut() = true;
        let target = if family.is_empty() { "Default" } else { family };
        let mut found_idx = None;
        for i in 0..self.family_string_list.n_items() {
            if let Some(s) = self.family_string_list.string(i) {
                if s.as_str() == target {
                    found_idx = Some(i);
                    break;
                }
            }
        }
        let idx = match found_idx {
            Some(i) => i,
            None => {
                self.family_string_list.append(target);
                self.family_string_list.n_items() - 1
            }
        };
        self.family_dropdown.set_selected(idx);
        if target != "Default" {
            let desc = gtk::pango::FontDescription::from_string(target);
            self.font_dialog_button.set_font_desc(&desc);
        }
        *self.is_updating.borrow_mut() = false;
    }

    pub fn set_font_size(&self, size: u32) {
        *self.is_updating.borrow_mut() = true;
        let size_str = size.to_string();
        for (i, &s) in FONT_SIZES.iter().enumerate() {
            if s == size_str {
                self.size_dropdown.set_selected(i as u32);
                break;
            }
        }
        *self.is_updating.borrow_mut() = false;
    }

    pub fn connect_format_action<F: Fn(FormatAction) + 'static>(&self, f: F) {
        self.format_callbacks.borrow_mut().push(Box::new(f));
    }

    pub fn connect_font_size_changed<F: Fn(u32) + 'static>(&self, f: F) {
        self.font_size_callbacks.borrow_mut().push(Box::new(f));
    }

    pub fn connect_font_family_changed<F: Fn(String) + 'static>(&self, f: F) {
        self.font_family_callbacks.borrow_mut().push(Box::new(f));
    }
}

impl Default for NoteToolbar {
    fn default() -> Self {
        Self::new()
    }
}

