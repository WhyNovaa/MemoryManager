
use iced::{Sandbox, Settings};
use crate::core::app::App;
mod gui;
mod core;

fn main() -> iced::Result{
    App::run(Settings::default())
}
