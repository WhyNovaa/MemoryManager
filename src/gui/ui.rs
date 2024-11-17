use iced::{Length, Padding};
use iced::widget::TextInput;

use crate::core::app::Message;


pub fn allocation_input_field(_placeholder: &str, _value: &str, ) -> TextInput<'static, Message> {
    TextInput::new(_placeholder, _value)
        .width(Length::Fixed(100.0))
        .padding(Padding::from(10))
}