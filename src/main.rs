use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Image, Label, Orientation};

fn main() {
    let app = Application::builder()
        .application_id("org.PCR.catsay")
        .build();

    app.connect_activate(|app| {
        // Creates the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Catsay")
            .build();

        // Here Box is not the std Rust Box type. This is a container
        // from GTK library. It works as a layout that contains and align
        // widgets.
        let layout_box = Box::new(Orientation::Vertical, 0);
        
        // Creating a label.
        let label = Label::new(Some("Meow!\n    \\\n      \\"));
        // Adding the lable to layout.
        layout_box.add(&label);

        // Getting the image into the program.
        let cat_image = Image::from_file("./images/cat.png");
        // Adding image into layout
        layout_box.add(&cat_image);

        // After all we have to add the layout onto the window.
        win.add(&layout_box);

        // Making everything created visible.
        win.show_all();

    });

    app.run();
}
