use fltk::{group::*, prelude::*, *};

fn main() {
    let gui = app::App::default().with_scheme(app::Scheme::Gtk);

    // Note: They are initializing the Window with default values
    // And then setting height and width with the "with_size" method
    // Docs -- https://docs.rs/fltk/latest/fltk/window/type.Window.html#method.with_size
    // Note: This method is from the WidgetExt trait
    // Docs -- https://docs.rs/fltk/latest/fltk/prelude/trait.WidgetExt.html
    let mut win = window::Window::default().with_size(400, 300);

    // Makes the window make_resizable
    // Docs -- https://docs.rs/fltk/latest/fltk/group/struct.Flex.html#method.make_resizable
    win.make_resizable(true);

    // Flex new method comes from implementing the WidgetBase trait.
    // Docs -- https://docs.rs/fltk/latest/fltk/group/struct.Flex.html#method.new
    let mut flex = Flex::new(0, 0, 400, 300, None);

    // Sets the type of widget it will be
    // Docs -- https://docs.rs/fltk/latest/fltk/group/struct.Flex.html#method.new
    // The book says that there are only two types and we can verify that by going to the docs (https://docs.rs/fltk/latest/fltk/prelude/trait.WidgetType.html#impl-WidgetType-for-FlexType) and looking at the source code.
    flex.set_type(group::FlexType::Column);

    let expanding = button::Button::default().with_label("Expanding");
    let mut normal = button::Button::default().with_label("Normal");

    // Sets the size of the widget that is passed in.
    // Docs -- https://docs.rs/fltk/latest/fltk/group/struct.Flex.html#method.fixed
    flex.fixed(&mut normal, 30);

    // Flex gets the end method from implementing the GroupExt trait
    // Docs -- https://docs.rs/fltk/latest/fltk/group/struct.Flex.html#method.end
    flex.end();

    win.end();
    win.show();

    gui.run().unwrap();
}

