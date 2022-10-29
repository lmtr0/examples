// use iced::widget::{button, Column, Text};
use iced::{Alignment, Element, Sandbox, Settings, Button, Row, Text, };

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        // let mut state = button::State::new();
        // let button = Button::new(&mut state, Text::new("Press me!")).on_press(Message::DecrementPressed);

        Row::with_children(vec![
            // Button::new(&state, Text::new("Press me!")),
            Text::new("Hello world").size(50).into(),
            // Button::new("Decrement").on_press(Message::DecrementPressed)
        ])
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
