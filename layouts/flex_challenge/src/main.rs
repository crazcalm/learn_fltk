use fltk::{group::*, prelude::*, *};

fn main() {
    let gui = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(300, 400);

    win.make_resizable(true);

    let mut flex1 = Flex::new(0, 0, 300, 100, None);
    flex1.set_type(group::FlexType::Column);
    let _btn1 = button::Button::default().with_label("top");
    flex1.end();

    let flex2 = Flex::new(0, 0, 300, 200, None).row();

    let _btn2 = button::Button::default().with_label("left");
    let _btn3 = button::Button::default().with_label("mid");
    let _btn4 = button::Button::default().with_label("right");
    flex2.end();

    // Below of is an example of relative positioning
    // Docs -- https://docs.rs/fltk/latest/fltk/group/struct.Flex.html#method.below_of
    flex2.clone().below_of(&flex1, 0);

    let flex3 = Flex::new(0, 0, 300, 100, None).column();
    let _btn5 = button::Button::default().with_label("bottom");

    flex3.end();

    flex3.below_of(&flex2, 0);

    win.end();
    win.show();

    gui.run().unwrap();
}
