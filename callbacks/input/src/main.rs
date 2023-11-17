use fltk::{prelude::*, *};

fn main() {
    let gui = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default()
        .with_size(100, 100)
        .column()
        .center_of_parent();

    // Changed the example to set the label value in button callback.
    // That is why this is now mutable.
    let mut label = frame::Frame::default().with_label("Enter age");

    // IntInput only accepts integer input
    // Docs -- https://docs.rs/fltk/latest/fltk/input/struct.IntInput.html#
    let input = input::IntInput::default();
    let mut btn = button::Button::default().with_label("Submit");
    flex.end();
    win.end();
    win.show();

    // Printing out the button trigger to see what it is set to.
    // Note: trigger is apart of the WidgetExt.
    println!("{:?}", btn.trigger());

    // Example of setting a new trigger
    // Uncomment to test this out.
    //btn.set_trigger(enums::CallbackTrigger::Never);

    btn.set_callback(move |_| {
        // The input features come from the InputExt trait
        // Docs -- https://docs.rs/fltk/latest/fltk/input/struct.Input.html#impl-InputExt-for-Input
        label.set_label(&format!("Age: {}", input.value()));
    });

    gui.run().unwrap();
}
