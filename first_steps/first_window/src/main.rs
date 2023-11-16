use fltk::{app, prelude::*, window::Window};

fn main() {
    // Create GUI Application
    // Note: Docs show that this is a Basic Container with very few methods
    // https://docs.rs/fltk/latest/fltk/window/type.Window.html#method.new
    // https://docs.rs/fltk/latest/fltk/app/struct.App.html
    let gui = app::App::default();

    // Create a Window
    // Note: Docs explain each param: https://docs.rs/fltk/latest/fltk/prelude/trait.WidgetBase.html#tymethod.new
    // This is from the WidgetBase Trait
    let mut window = Window::new(100, 100, 400, 300, "My Window");

    window.clone().center_screen();

    // State that you are done modifying the window
    // Note: docs state that this "ends a group"
    // https://docs.rs/fltk/latest/fltk/window/type.Window.html#method.end
    // GroupExt docs -- https://docs.rs/fltk/latest/fltk/prelude/trait.GroupExt.html
    window.end();

    // Place the window on the GUI Application
    // Docs say "shows widget" but there is not much here. No Trait and anything.
    // https://docs.rs/fltk/latest/fltk/window/type.Window.html#method.show
    window.show();

    // Start the event loop that will run your GUI Application
    // Docs -- https://docs.rs/fltk/latest/fltk/app/struct.App.html#method.run
    gui.run().unwrap();
}
