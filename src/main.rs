mod stereotype;

use adw::{ActionRow, Application, ApplicationWindow, Clamp, HeaderBar, ToolbarView};
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use gtk::{Align, Orientation};
use gtk::{Box, Button, FileDialog, ListBox, ScrolledWindow};
use stereotype::{file_stereotype::FileStereotype, stereotyper::stereotype_file};

const APP_ID: &str = "dev.mateusrodcosta.Stereotyping";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_intial_screen);

    app.run()
}

fn build_intial_screen(app: &Application) {
    let header_bar = HeaderBar::builder().build();

    let button = Button::builder().label("Choose File...").build();
    button.add_css_class("suggested-action");
    button.add_css_class("pill");
    button.set_vexpand(false);
    button.set_hexpand(false);
    button.set_can_shrink(true);

    let content = Box::builder()
        .margin_top(32)
        .margin_bottom(32)
        .margin_start(32)
        .margin_end(32)
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .build();
    content.set_spacing(16);
    content.append(&button);

    let toolbar_view = ToolbarView::builder().content(&content).build();
    toolbar_view.add_top_bar(&header_bar);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Stereotyping")
        .content(&toolbar_view)
        .build();
    window.set_default_size(640, 480);

    button.connect_clicked(clone!(
        #[weak]
        window,
        #[weak]
        toolbar_view,
        move |_| {
            let file_dialog = FileDialog::new();
            file_dialog.open(
                Some(&window),
                None::<&gio::Cancellable>,
                clone!(
                    #[weak]
                    window,
                    move |r| {
                        match r {
                            Ok(file) => fetch_file_data(file, &window, &toolbar_view),
                            _ => (),
                        }
                    }
                ),
            );
        }
    ));

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
    let listbox = ListBox::builder().build();
    listbox.add_css_class("boxed-list");
    for item in list_items.iter() {
        listbox.append(item);
    }

    let content = Box::builder()
        .margin_top(32)
        .margin_bottom(32)
        .margin_start(32)
        .margin_end(32)
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .build();
    content.set_spacing(16);
    content.append(&listbox);

    let clamp = Clamp::builder().child(&content).build();
    let scrolled_window = ScrolledWindow::builder().child(&clamp).build();

    scrolled_window
}

fn stereotype_to_rows(stereotype: &FileStereotype) -> [ActionRow; 5] {
    let item_filename = ActionRow::builder()
        .title("Filename")
        .subtitle(stereotype.get_filename())
        .build();
    item_filename.add_css_class("property");

    let item_description = ActionRow::builder()
        .title("Description")
        .subtitle(stereotype.get_description())
        .build();
    item_description.add_css_class("property");

    let item_mime_type = ActionRow::builder()
        .title("MIME Type")
        .subtitle(stereotype.get_mime_type())
        .build();
    item_mime_type.add_css_class("property");

    let item_mime_encoding = ActionRow::builder()
        .title("MIME Encoding")
        .subtitle(stereotype.get_mime_encoding())
        .build();
    item_mime_encoding.add_css_class("property");

    let item_extension = ActionRow::builder()
        .title("Extension")
        .subtitle(stereotype.get_entension())
        .build();
    item_extension.add_css_class("property");

    [
        item_filename,
        item_description,
        item_mime_type,
        item_mime_encoding,
        item_extension,
    ]
}
