use fltk::{prelude::*, *};

fn main() {
    let gui = app::App::default();

    let mut win = window::Window::default().with_size(400, 300);
    let mut flex_1 = group::Flex::new(0, 0, 200, 150, None);
    let mut bnt_1 = button::Button::new(0, 0, 40, 50, "Btn 1");
    flex_1.end();

    let mut flex_2 = group::Flex::new(0, 0, 200, 150, None);
    let mut bnt_2 = button::Button::new(0, 0, 40, 50, "Btn 2");
    let mut bnt_3 = button::Button::new(0, 0, 40, 50, "Btn 3");
    flex_2.end();
    win.end();

    win.show();

    flex_2.clone().below_of(&flex_1, 10);

    // Button handlers
    bnt_1.handle(move |_, event| {
        println!("bnt 1: {:?}", event);
        false
    });
    bnt_2.handle(move |_, event| {
        println!("bnt 2: {:?}", event);
        false
    });
    bnt_3.handle(move |_, event| {
        println!("bnt 3: {:?}", event);
        true
    });

    // Flexbox handlers
    flex_1.handle(move |_, event| {
        println!("frame_1: {:?}", event);
        false
    });
    flex_2.handle(move |_, event| {
        println!("frame_2: {:?}", event);
        true
    });

    // Window handler
    win.handle(move |_, event| {
        println!("window: {:?}", event);
        true
    });

    gui.run().unwrap();
}
