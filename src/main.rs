use gtk::{prelude::*, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.e.HelloWorld";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Hello")
        .margin_top(10)
        .margin_end(10)
        .margin_start(10)
        .margin_bottom(10)
        .build();

    button.connect_clicked(move |button| {
        // button.set_label("Hello World!");
        button.set_label(match button.label().unwrap().as_str() {
            "Hello" => "World",
            "World" => "Hello",
            _ => panic!("What..?"),
        });
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}
