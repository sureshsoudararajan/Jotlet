use adw::prelude::*;
use gtk::gdk;
use gtk::gio;
use gtk::pango;
use std::cell::RefCell;
use std::rc::Rc;

/// Formatting actions that the editor supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatAction {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    BulletList,
    NumberedList,
    Checklist,
    Heading,
}

type ChangeCallback = Box<dyn Fn()>;

#[derive(Clone)]
pub struct NoteEditor {
    text_view: gtk::TextView,
    buffer: gtk::TextBuffer,
    changed_callbacks: Rc<RefCell<Vec<ChangeCallback>>>,
    font_provider: Rc<RefCell<Option<gtk::CssProvider>>>,
}

impl NoteEditor {
    pub fn new() -> Self {
        let buffer = gtk::TextBuffer::new(None::<&gtk::TextTagTable>);
        buffer.set_enable_undo(true);

        let text_view = gtk::TextView::builder()
            .buffer(&buffer)
            .wrap_mode(gtk::WrapMode::Word)
            .left_margin(12)
            .right_margin(12)
            .top_margin(8)
            .bottom_margin(8)
            .hexpand(true)
            .vexpand(true)
            .build();

        text_view.add_css_class("note-body");

        Self::create_tags(&buffer);

        let changed_callbacks: Rc<RefCell<Vec<ChangeCallback>>> =
            Rc::new(RefCell::new(Vec::new()));
        let cbs = changed_callbacks.clone();
        buffer.connect_changed(move |_| {
            for cb in cbs.borrow().iter() {
                cb();
            }
        });

        let font_provider = Rc::new(RefCell::new(None));

        let editor = Self {
            text_view,
            buffer,
            changed_callbacks,
            font_provider,
        };

        editor.setup_click_gesture();
        editor.setup_key_controller();

        editor
    }

    pub fn text_view(&self) -> &gtk::TextView {
        &self.text_view
    }

    pub fn buffer(&self) -> &gtk::TextBuffer {
        &self.buffer
    }

    pub fn set_content(&self, content: &str) {
        self.buffer.set_text(content);
        self.apply_initial_tags();
    }

    pub fn get_content(&self) -> String {
        let start = self.buffer.start_iter();
        let end = self.buffer.end_iter();
        self.buffer.text(&start, &end, true).to_string()
    }

    pub fn connect_changed<F: Fn() + 'static>(&self, f: F) {
        self.changed_callbacks.borrow_mut().push(Box::new(f));
    }

    #[allow(deprecated)]
    pub fn set_font_size(&self, size: u32) {
        if let Some(ref old_provider) = *self.font_provider.borrow() {
            self.text_view.style_context().remove_provider(old_provider);
        }
        let provider = gtk::CssProvider::new();
        provider.load_from_string(&format!("textview.note-body {{ font-size: {}px; }}", size));
        self.text_view
            .style_context()
            .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
        *self.font_provider.borrow_mut() = Some(provider);
    }

    pub fn apply_format(&self, action: FormatAction) {
        match action {
            FormatAction::Bold => self.toggle_tag("bold"),
            FormatAction::Italic => self.toggle_tag("italic"),
            FormatAction::Underline => self.toggle_tag("underline"),
            FormatAction::Strikethrough => self.toggle_tag("strikethrough"),
            FormatAction::Heading => self.toggle_line_tag("heading"),
            FormatAction::BulletList => self.insert_bullet_item(),
            FormatAction::NumberedList => self.insert_numbered_item(),
            FormatAction::Checklist => self.insert_checklist_item(),
        }
    }

    fn create_tags(buffer: &gtk::TextBuffer) {
        let tag_table = buffer.tag_table();

        let bold_tag = gtk::TextTag::builder()
            .name("bold")
            .weight(700)
            .build();
        tag_table.add(&bold_tag);

        let italic_tag = gtk::TextTag::builder()
            .name("italic")
            .style(pango::Style::Italic)
            .build();
        tag_table.add(&italic_tag);

        let underline_tag = gtk::TextTag::builder()
            .name("underline")
            .underline(pango::Underline::Single)
            .build();
        tag_table.add(&underline_tag);

        let strikethrough_tag = gtk::TextTag::builder()
            .name("strikethrough")
            .strikethrough(true)
            .build();
        tag_table.add(&strikethrough_tag);

        let heading_tag = gtk::TextTag::builder()
            .name("heading")
            .weight(700)
            .scale(1.4)
            .pixels_below_lines(4)
            .build();
        tag_table.add(&heading_tag);

        let link_tag = gtk::TextTag::builder()
            .name("link")
            .underline(pango::Underline::Single)
            .foreground("rgb(53,132,228)")
            .build();
        tag_table.add(&link_tag);

        let checkbox_unchecked_tag = gtk::TextTag::builder()
            .name("checkbox-unchecked")
            .weight(700)
            .scale(1.15)
            .foreground("rgb(119, 118, 123)")
            .build();
        tag_table.add(&checkbox_unchecked_tag);

        let checkbox_checked_tag = gtk::TextTag::builder()
            .name("checkbox-checked")
            .weight(700)
            .scale(1.15)
            .foreground("rgb(46, 194, 126)")
            .build();
        tag_table.add(&checkbox_checked_tag);

        let checked_text_tag = gtk::TextTag::builder()
            .name("checked-text")
            .strikethrough(true)
            .foreground("rgba(130, 130, 130, 0.55)")
            .build();
        tag_table.add(&checked_text_tag);

        let bullet_tag = gtk::TextTag::builder()
            .name("bullet-list")
            .left_margin(24)
            .build();
        tag_table.add(&bullet_tag);

        let number_tag = gtk::TextTag::builder()
            .name("numbered-list")
            .left_margin(24)
            .build();
        tag_table.add(&number_tag);
    }

    /// Sets up click gesture for interactive checkboxes and links.
    fn setup_click_gesture(&self) {
        let gesture = gtk::GestureClick::new();
        let buf = self.buffer.clone();
        let tv = self.text_view.clone();
        let editor = self.clone();

        gesture.connect_pressed(move |_, _n_press, x, y| {
            let (buf_x, buf_y) =
                tv.window_to_buffer_coords(gtk::TextWindowType::Widget, x as i32, y as i32);
            let (iter, _) = tv.line_at_y(buf_y);

            let mut line_start = iter;
            line_start.set_line_offset(0);
            let mut line_end = iter;
            if !line_end.ends_line() {
                line_end.forward_to_line_end();
            }

            let line_text = buf.text(&line_start, &line_end, true).to_string();

            // Check if clicking near the checkbox (first 50 pixels or first 2 characters)
            if line_text.starts_with("☐ ") || line_text.starts_with("☑ ") {
                let at_box = if let Some((click_iter, _)) = tv.iter_at_position(buf_x, buf_y) {
                    x < 50.0 || click_iter.line_offset() <= 1
                } else {
                    x < 50.0
                };
                if at_box {
                    editor.toggle_checkbox_at_line(iter.line());
                    return;
                }
            }

            // Check if clicked on a link
            if let Some(link_tag) = buf.tag_table().lookup("link") {
                if iter.has_tag(&link_tag) {
                    let mut url_start = iter;
                    while !url_start.is_start() && url_start.has_tag(&link_tag) {
                        url_start.backward_char();
                    }
                    if !url_start.has_tag(&link_tag) {
                        url_start.forward_char();
                    }
                    let mut url_end = iter;
                    while !url_end.is_end() && url_end.has_tag(&link_tag) {
                        url_end.forward_char();
                    }

                    let url = buf.text(&url_start, &url_end, false).to_string();
                    let url_clean = url.trim();
                    if url_clean.starts_with("http://") || url_clean.starts_with("https://") {
                        let _ = gio::AppInfo::launch_default_for_uri(
                            url_clean,
                            None::<&gio::AppLaunchContext>,
                        );
                    }
                }
            }
        });

        self.text_view.add_controller(gesture);
    }

    /// Toggles a checkbox on a specific line and applies professional styling & strikethrough.
    pub fn toggle_checkbox_at_line(&self, line_num: i32) {
        let Some(line_start) = self.buffer.iter_at_line(line_num) else { return };
        let mut line_end = line_start;
        if !line_end.ends_line() {
            line_end.forward_to_line_end();
        }
        let line_text = self.buffer.text(&line_start, &line_end, true).to_string();

        if line_text.starts_with("☐ ") {
            let mut s = line_start;
            let mut e = line_start;
            e.forward_chars(1);
            self.buffer.delete(&mut s, &mut e);
            self.buffer.insert(&mut s, "☑");

            // Re-acquire fresh iterators after buffer mutation
            if let Some(l_start) = self.buffer.iter_at_line(line_num) {
                let mut box_end = l_start;
                box_end.forward_chars(1);
                self.buffer.remove_tag_by_name("checkbox-unchecked", &l_start, &box_end);
                self.buffer.apply_tag_by_name("checkbox-checked", &l_start, &box_end);

                let mut text_start = l_start;
                text_start.forward_chars(2);
                let mut l_end = text_start;
                if !l_end.ends_line() {
                    l_end.forward_to_line_end();
                }
                self.buffer.apply_tag_by_name("checked-text", &text_start, &l_end);
            }
        } else if line_text.starts_with("☑ ") {
            let mut s = line_start;
            let mut e = line_start;
            e.forward_chars(1);
            self.buffer.delete(&mut s, &mut e);
            self.buffer.insert(&mut s, "☐");

            // Re-acquire fresh iterators after buffer mutation
            if let Some(l_start) = self.buffer.iter_at_line(line_num) {
                let mut box_end = l_start;
                box_end.forward_chars(1);
                self.buffer.remove_tag_by_name("checkbox-checked", &l_start, &box_end);
                self.buffer.apply_tag_by_name("checkbox-unchecked", &l_start, &box_end);

                let mut text_start = l_start;
                text_start.forward_chars(2);
                let mut l_end = text_start;
                if !l_end.ends_line() {
                    l_end.forward_to_line_end();
                }
                self.buffer.remove_tag_by_name("checked-text", &text_start, &l_end);
            }
        }
    }

    /// Sets up keyboard shortcuts for rich formatting.
    fn setup_key_controller(&self) {
        let key_controller = gtk::EventControllerKey::new();
        let editor = self.clone();

        key_controller.connect_key_pressed(move |_, keyval, _keycode, state| {
            let ctrl = state.contains(gdk::ModifierType::CONTROL_MASK);
            let shift = state.contains(gdk::ModifierType::SHIFT_MASK);

            if ctrl && !shift {
                match keyval {
                    gdk::Key::b | gdk::Key::B => {
                        editor.apply_format(FormatAction::Bold);
                        return glib::Propagation::Stop;
                    }
                    gdk::Key::i | gdk::Key::I => {
                        editor.apply_format(FormatAction::Italic);
                        return glib::Propagation::Stop;
                    }
                    gdk::Key::u | gdk::Key::U => {
                        editor.apply_format(FormatAction::Underline);
                        return glib::Propagation::Stop;
                    }
                    _ => {}
                }
            } else if ctrl && shift {
                match keyval {
                    gdk::Key::_7 | gdk::Key::ampersand => {
                        editor.apply_format(FormatAction::NumberedList);
                        return glib::Propagation::Stop;
                    }
                    gdk::Key::_8 | gdk::Key::asterisk => {
                        editor.apply_format(FormatAction::BulletList);
                        return glib::Propagation::Stop;
                    }
                    gdk::Key::_9 | gdk::Key::parenleft => {
                        editor.apply_format(FormatAction::Checklist);
                        return glib::Propagation::Stop;
                    }
                    _ => {}
                }
            } else if !ctrl
                && !shift
                && !state.contains(gdk::ModifierType::ALT_MASK)
                && (keyval == gdk::Key::Return || keyval == gdk::Key::KP_Enter)
                && editor.handle_enter_key()
            {
                return glib::Propagation::Stop;
            }

            glib::Propagation::Proceed
        });

        self.text_view.add_controller(key_controller);
    }

    /// Handles Enter key press for continuing or exiting lists and checklists.
    fn handle_enter_key(&self) -> bool {
        let mark = self.buffer.get_insert();
        let iter = self.buffer.iter_at_mark(&mark);

        // If cursor is at start of line, let default Enter behavior insert line before it
        if iter.line_offset() == 0 {
            return false;
        }

        let mut line_start = iter;
        line_start.set_line_offset(0);
        let mut line_end = iter;
        if !line_end.ends_line() {
            line_end.forward_to_line_end();
        }

        let line_text = self.buffer.text(&line_start, &line_end, true).to_string();

        // 1. Checklist items (☐ or ☑)
        if line_text.starts_with("☐ ") || line_text.starts_with("☑ ") {
            let content = &line_text[2..];
            if content.trim().is_empty() {
                // Empty item: delete prefix to exit list mode
                let mut s = line_start;
                let mut e = line_end;
                self.buffer.delete(&mut s, &mut e);
                return true;
            }

            if let Some((mut s, mut e)) = self.buffer.selection_bounds() {
                self.buffer.delete(&mut s, &mut e);
            }

            self.buffer.insert_at_cursor("\n☐ ");

            // Apply unchecked style to the new box and ensure no strikethrough
            let mark = self.buffer.get_insert();
            let cur = self.buffer.iter_at_mark(&mark);
            let line_num = cur.line();
            if let Some(l_start) = self.buffer.iter_at_line(line_num) {
                let mut box_end = l_start;
                box_end.forward_chars(1);
                self.buffer.remove_tag_by_name("checkbox-checked", &l_start, &box_end);
                self.buffer.remove_tag_by_name("checked-text", &l_start, &box_end);
                self.buffer.apply_tag_by_name("checkbox-unchecked", &l_start, &box_end);

                let mut l_end = l_start;
                if !l_end.ends_line() {
                    l_end.forward_to_line_end();
                }
                self.buffer.remove_tag_by_name("checked-text", &l_start, &l_end);
            }

            self.text_view.scroll_mark_onscreen(&mark);
            return true;
        }

        // 2. Bullet list items (• )
        if line_text.starts_with("• ") {
            let content = &line_text[2..];
            if content.trim().is_empty() {
                // Empty item: delete bullet to exit list mode
                let mut s = line_start;
                let mut e = line_end;
                self.buffer.delete(&mut s, &mut e);
                return true;
            }

            if let Some((mut s, mut e)) = self.buffer.selection_bounds() {
                self.buffer.delete(&mut s, &mut e);
            }

            self.buffer.insert_at_cursor("\n• ");
            let mark = self.buffer.get_insert();
            self.text_view.scroll_mark_onscreen(&mark);
            return true;
        }

        // 3. Numbered list items (<num>. )
        if let Some(dot_pos) = line_text.find(". ") {
            let prefix = &line_text[..dot_pos];
            if !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_digit()) {
                let content = &line_text[dot_pos + 2..];
                if content.trim().is_empty() {
                    // Empty item: delete prefix to exit list mode
                    let mut s = line_start;
                    let mut e = line_end;
                    self.buffer.delete(&mut s, &mut e);
                    return true;
                }

                if let Ok(num) = prefix.parse::<u64>() {
                    if let Some((mut s, mut e)) = self.buffer.selection_bounds() {
                        self.buffer.delete(&mut s, &mut e);
                    }

                    self.buffer.insert_at_cursor(&format!("\n{}. ", num + 1));
                    let mark = self.buffer.get_insert();
                    self.text_view.scroll_mark_onscreen(&mark);
                    return true;
                }
            }
        }

        false
    }

    /// Applies initial formatting tags based on content (e.g. checked items, URLs).
    fn apply_initial_tags(&self) {
        let start = self.buffer.start_iter();
        let mut iter = start;

        // Apply checked / unchecked styling to any checklist line
        while !iter.is_end() {
            let mut line_end = iter;
            if !line_end.ends_line() {
                line_end.forward_to_line_end();
            }

            let line_text = self.buffer.text(&iter, &line_end, true).to_string();
            if line_text.starts_with("☑ ") {
                let mut box_end = iter;
                box_end.forward_chars(1);
                self.buffer.apply_tag_by_name("checkbox-checked", &iter, &box_end);

                let mut text_start = iter;
                text_start.forward_chars(2);
                self.buffer.apply_tag_by_name("checked-text", &text_start, &line_end);
            } else if line_text.starts_with("☐ ") {
                let mut box_end = iter;
                box_end.forward_chars(1);
                self.buffer.apply_tag_by_name("checkbox-unchecked", &iter, &box_end);
            }

            if !iter.forward_line() {
                break;
            }
        }

        // Apply link tags to URLs
        let full_text = self.get_content();
        let url_regex = regex::Regex::new(r"https?://[^\s<]+").ok();
        if let Some(re) = url_regex {
            for mat in re.find_iter(&full_text) {
                let s_offset = mat.start() as i32;
                let e_offset = mat.end() as i32;
                let s_iter = self.buffer.iter_at_offset(s_offset);
                let e_iter = self.buffer.iter_at_offset(e_offset);
                self.buffer.apply_tag_by_name("link", &s_iter, &e_iter);
            }
        }
    }

    fn toggle_tag(&self, tag_name: &str) {
        if let Some((start, end)) = self.buffer.selection_bounds() {
            if let Some(tag) = self.buffer.tag_table().lookup(tag_name) {
                if start.has_tag(&tag) {
                    self.buffer.remove_tag_by_name(tag_name, &start, &end);
                } else {
                    self.buffer.apply_tag_by_name(tag_name, &start, &end);
                }
            }
        }
    }

    fn toggle_line_tag(&self, tag_name: &str) {
        let mark = self.buffer.get_insert();
        let iter = self.buffer.iter_at_mark(&mark);

        let mut line_start = iter;
        line_start.set_line_offset(0);
        let mut line_end = iter;
        if !line_end.ends_line() {
            line_end.forward_to_line_end();
        }

        if let Some(tag) = self.buffer.tag_table().lookup(tag_name) {
            if line_start.has_tag(&tag) {
                self.buffer
                    .remove_tag_by_name(tag_name, &line_start, &line_end);
            } else {
                self.buffer
                    .apply_tag_by_name(tag_name, &line_start, &line_end);
            }
        }
    }

    fn insert_bullet_item(&self) {
        let mark = self.buffer.get_insert();
        let mut iter = self.buffer.iter_at_mark(&mark);
        iter.set_line_offset(0);

        let line_text = self.get_line_text(&iter);
        if line_text.starts_with("• ") {
            let mut end = iter;
            end.set_line_offset(2);
            self.buffer.delete(&mut iter, &mut end);
        } else {
            self.buffer.insert(&mut iter, "• ");
        }
    }

    fn insert_numbered_item(&self) {
        let mark = self.buffer.get_insert();
        let mut iter = self.buffer.iter_at_mark(&mark);
        iter.set_line_offset(0);

        let line_text = self.get_line_text(&iter);
        if let Some(dot_pos) = line_text.find(". ") {
            let prefix = &line_text[..dot_pos];
            if prefix.chars().all(|c| c.is_ascii_digit()) {
                let mut end = iter;
                end.set_line_offset((dot_pos + 2) as i32);
                self.buffer.delete(&mut iter, &mut end);
                return;
            }
        }

        let line_num = iter.line();
        let mut count = 1;
        if line_num > 0 {
            let mut prev = self.buffer.iter_at_line(line_num - 1).unwrap_or(iter);
            prev.set_line_offset(0);
            let prev_text = self.get_line_text(&prev);
            if let Some(n) = prev_text
                .split('.')
                .next()
                .and_then(|s| s.trim().parse::<i32>().ok())
            {
                count = n + 1;
            }
        }
        self.buffer.insert(&mut iter, &format!("{}. ", count));
    }

    fn insert_checklist_item(&self) {
        let mark = self.buffer.get_insert();
        let mut iter = self.buffer.iter_at_mark(&mark);
        let line_num = iter.line();
        iter.set_line_offset(0);

        let line_text = self.get_line_text(&iter);
        if line_text.starts_with("☐ ") || line_text.starts_with("☑ ") {
            let mut end = iter;
            end.set_line_offset(2);
            self.buffer.delete(&mut iter, &mut end);
            if let Some(l_start) = self.buffer.iter_at_line(line_num) {
                let mut l_end = l_start;
                if !l_end.ends_line() {
                    l_end.forward_to_line_end();
                }
                self.buffer.remove_tag_by_name("checkbox-unchecked", &l_start, &l_end);
                self.buffer.remove_tag_by_name("checkbox-checked", &l_start, &l_end);
                self.buffer.remove_tag_by_name("checked-text", &l_start, &l_end);
            }
        } else {
            self.buffer.insert(&mut iter, "☐ ");
            if let Some(l_start) = self.buffer.iter_at_line(line_num) {
                let mut box_end = l_start;
                box_end.forward_chars(1);
                self.buffer.apply_tag_by_name("checkbox-unchecked", &l_start, &box_end);
            }
        }
    }

    fn get_line_text(&self, iter: &gtk::TextIter) -> String {
        let mut start = *iter;
        start.set_line_offset(0);
        let mut end = *iter;
        if !end.ends_line() {
            end.forward_to_line_end();
        }
        self.buffer.text(&start, &end, true).to_string()
    }
}

impl Default for NoteEditor {
    fn default() -> Self {
        Self::new()
    }
}
