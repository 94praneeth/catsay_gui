extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;


fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("layout.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder
        .object("applicationwindow1").unwrap();
    
    window.set_application(Some(app));

    // Load the input outputs after setting a the application.

    // Inputs.
    let message_input: gtk::Entry = builder.object("message_input").unwrap();

    // Submit button.
    let button: gtk::Button = builder.object("generate_button").unwrap();
    let is_dead_switch: gtk::Switch = builder.object("is_dead_switch").unwrap();

    // Outputs.
    let message_output: gtk::Label = builder.object("message_output").unwrap();
    let image_output: gtk::Image = builder.object("image_output") .unwrap();
    let image_output_clone = image_output.clone();

    button.connect_clicked(move |_| {
        message_output.set_text(&format!(
            "{}\n     \\\n    \\",
            message_input.to_string().as_str()
        ));

        let is_dead = is_dead_switch.is_active();

        if is_dead {
            image_output_clone.set_from_file(Some("./images/cat_dead.png"));
        } else {
            image_output_clone.set_from_file(Some("./images/cat.png"));
        }

        image_output_clone.show();
    });

    window.show_all();
    image_output.hide();

}

fn main() {

    let application = gtk::Application::new(
        Some("pcr.catsay-gui"),
        Default::default()
    );

    application.connect_activate(|app|{
        build_ui(app);
    });

    application.run();


    // let app = Application::new(Some("pcr.catsay-gui"), gio::ApplicationFlags::empty());

    // // Callback when the app starts
    // app.connect_startup(|app| {
    //     let window = ApplicationWindow::new(app);
    //     window.set_title("Catsay");
    //     window.set_default_size(350, 70);

    //     window.connect_delete_event(|win, _| {
    //         unsafe{
    //             win.destroy();
    //             Inhibit(false)
    //         }
    //     });
    //     window.show_all();
    // });

    // app.connect_activate(|_|{});
    // app.run();
}
