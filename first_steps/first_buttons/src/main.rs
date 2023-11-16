use fltk::{app, button::Button, prelude::*, window::Window};

fn main() {
    let gui = app::App::default();
    let mut window = Window::new(100, 100, 400, 300, "My Window");

    // Button Implements WidgetBase -- https://docs.rs/fltk/latest/fltk/prelude/trait.WidgetBase.html
    // WidgetBase defines methods implemented by all widgets.
    // WidgetBase has a new method for initializing a widget.
    let _btn = Button::new(50, 50, 100, 100, "Button");

    window.end();
    window.show();

    gui.run().unwrap();
}
