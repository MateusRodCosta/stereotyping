mod config;
mod stereotype;

use adw::{ActionRow, Application, ApplicationWindow, Clamp, HeaderBar, ToolbarView};
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use gtk::{Box, Button, Label, ListBox};
use gtk::{Orientation, Align};
use gtk::FileDialog;
use stereotype::{stereotyper::stereotype_file, file_stereotype::FileStereotype};
use config::APP_ID;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_intial_screen);

    app.run()
}

fn build_intial_screen(app: &Application) {
    let header_bar = HeaderBar::builder().build();

    let button = Button::builder()
        .label("Choose File...")
        .css_classes(["pill", "suggested-action"])
        .build();

    let content = Box::builder()
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(12)
        .margin_end(12)
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    content.set_spacing(24);
    content.append(&button);

    let toolbar_view = ToolbarView::builder().content(&content).build();
    toolbar_view.add_top_bar(&header_bar);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Stereotyping")
        .content(&toolbar_view)
        .build();
    window.set_default_size(640, 480);

    if APP_ID.ends_with(".Devel") {
        window.add_css_class("devel");
    }

    button.connect_clicked(clone!(@weak window, @weak toolbar_view => move |_| {
        let file_dialog = FileDialog::new();
        file_dialog.open(Some(&window), None::<&gio::Cancellable>, clone!(@weak window => move |r| {
            match r {
                Ok(file) => fetch_file_data(file, &window, &toolbar_view),
                _ => (),
            }
        }));
    }));

    window.present();
}

fn fetch_file_data(file: gio::File, window: &ApplicationWindow, toolbar_view: &ToolbarView) {
    let path = match file.path() {
        Some(p) => match p.into_os_string().into_string() {
            Ok(r) => Some(r),
            _ => None,
        },
        _ => None,
    };

    let stereotype = match path {
        Some(p) => stereotype_file(&p),
        _ => None,
    };

    match stereotype {
        Some(s) => {
            let content = build_details_screen(&s);
            let filename = &s.get_filename();
            window.set_title(Some(&format!("Stereotyping - {}", filename)));
            toolbar_view.set_content(Some(&content));
        }
        _ => (),
    };
}

fn build_details_screen(stereotype: &FileStereotype) -> ScrolledWindow {
    let list_items = stereotype_to_rows(stereotype);
    let listbox = ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    for item in list_items.iter() {
        listbox.append(item);
    }

    let content = Box::builder()
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(12)
        .margin_end(12)
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    content.set_spacing(24);
    content.append(&listbox);

    let clamp = Clamp::builder().child(&content).build();
    let scrolled_window = ScrolledWindow::builder().child(&clamp).build();

    scrolled_window
}

fn stereotype_to_rows(stereotype: &FileStereotype) -> [ActionRow; 5] {
    let item_filename = ActionRow::builder()
        .title("Filename")
        .subtitle(stereotype.get_filename())
        .css_classes(["property"])
        .build();

    let item_description = ActionRow::builder()
        .title("Description")
        .subtitle(stereotype.get_description())
        .css_classes(["property"])
        .build();

    let item_mime_type = ActionRow::builder()
        .title("MIME Type")
        .subtitle(stereotype.get_mime_type())
        .css_classes(["property"])
        .build();

    let item_mime_encoding = ActionRow::builder()
        .title("MIME Encoding")
        .subtitle(stereotype.get_mime_encoding())
        .css_classes(["property"])
        .build();

    let item_extension = ActionRow::builder()
        .title("Extension")
        .subtitle(stereotype.get_entension())
        .css_classes(["property"])
        .build();

    [
        item_filename,
        item_description,
        item_mime_type,
        item_mime_encoding,
        item_extension,
    ]
}
