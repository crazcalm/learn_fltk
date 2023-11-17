use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use std::time::SystemTime;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);

    // Frame is an ordinary widget. Nothing special here.
    // Docs -- https://docs.rs/fltk/latest/fltk/frame/struct.Frame.html#
    let mut frame = Frame::default().with_size(200, 100).center_of(&wind);

    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();

    // set_callback is a method from the WidgetExt Trait
    // Docs - https://docs.rs/fltk/latest/fltk/prelude/trait.WidgetExt.html#tymethod.set_callback
    but.set_callback(move |btn| {
        // set_lable is a method from the Widget Trait
        // Docs - https://docs.rs/fltk/latest/fltk/prelude/trait.WidgetExt.html#tymethod.set_label
        // Note: I added a systemtime string here so that you can visually note a difference
        // between button click responses.
        frame.set_label(&format!(
            "Hello world - {:?}",
            SystemTime::now().elapsed().unwrap()
        ));
        btn.set_label("Clicked");
    });

    app.run().unwrap();
}
