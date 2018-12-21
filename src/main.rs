extern crate azul;

use azul::prelude::*;
use azul::widgets::{button::Button, label::Label};

struct MyApplication {
    count: i32,
}

impl Layout for MyApplication {
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("Count: {}", self.count)).dom();

        let increase_button = Button::with_label("Increase")
            .dom()
            .with_callback(On::MouseDown, Callback(increase_count));

        let decrease_button = Button::with_label("Decrease")
            .dom()
            .with_callback(On::MouseDown, Callback(decrease_count));

        let button_container = Dom::new(NodeType::Div)
            .with_child(increase_button)
            .with_child(decrease_button);

        Dom::new(NodeType::Div)
            .with_child(label)
            .with_child(button_container)
    }
}

fn increase_count(
    state: &mut AppState<MyApplication>,
    _event: WindowEvent<MyApplication>,
) -> UpdateScreen {
    state.data.modify(|state| state.count += 1);
    UpdateScreen::Redraw
}

fn decrease_count(
    state: &mut AppState<MyApplication>,
    _event: WindowEvent<MyApplication>,
) -> UpdateScreen {
    state.data.modify(|state| state.count -= 1);
    UpdateScreen::Redraw
}

fn main() {
    let window = Window::new(WindowCreateOptions::default(), css::native()).unwrap();

    App::new(MyApplication { count: 0 }, AppConfig::default())
        .run(window)
        .unwrap();
}
