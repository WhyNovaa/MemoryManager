use iced::{theme, Color, Element, Length, Sandbox};
use iced::alignment::{Horizontal};
use iced::widget::{button, container, text, Column, Row, Scrollable, Space};

use crate::core::row::PackageRow;
use crate::gui::styles::{ButtonStyle, ConStyle, MainStyle};
use crate::gui::ui::allocation_input_field;

pub struct App {
    rows: Vec<PackageRow>,
    allocation_field: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Allocate,
    DeallocateChecked,
    ToggleCheck(i64),
    SelectAll(bool),
    AllocationFieldChanged(String),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            //rows: vec![PackageRow::new(17000), PackageRow::new(11)],
            rows: Vec::new(),
            allocation_field: String::new(),
        }
    }
    fn title(&self) -> String {
        String::from("Memory manager")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Allocate => {
                match self.allocation_field.parse::<usize>() {
                    Ok(size) => {
                        self.rows.push(PackageRow::new(size));
                    }
                    _ => {}
                }
            }
            Message::DeallocateChecked => {
                let mut vec = Vec::new();
                for row in self.rows.clone().iter_mut() {
                    if !row.checked {
                        vec.push(row.clone());
                    } else {
                        row.clear_memory();
                    }
                }
                self.rows = vec;
            }
            Message::ToggleCheck(addr) => {
                for r in &mut self.rows {
                    if r.addr == addr {
                        r.checked = !r.checked;
                    }
                }
            }
            Message::SelectAll(checked) => {
                for r in &mut self.rows {
                    r.checked = checked;
                }
            }
            Message::AllocationFieldChanged(size) => {
                self.allocation_field = size;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let but_col = Column::new()
            .push(button("Select all").on_press(Message::SelectAll(true))
                .style(theme::Button::Custom(Box::new(ButtonStyle::Standard))).width(Length::from(200)))
            .push(Space::with_height(5))
            .push(button("Unselect all").on_press(Message::SelectAll(false))
                .style(theme::Button::Custom(Box::new(ButtonStyle::Standard))).width(Length::from(200)))
            .push(Space::with_height(5))
            .push(button("Delete selected").on_press(Message::DeallocateChecked)
                .style(theme::Button::Custom(Box::new(ButtonStyle::DeleteButton))).width(Length::from(200)))
            .padding(10);

        let menu_row = Row::new()
            .push(button("Allocate").on_press(Message::Allocate)
                .style(theme::Button::Custom(Box::new(ButtonStyle::Standard))).width(Length::from(200)))
            .push(
                allocation_input_field("Input size...", &self.allocation_field)
                    .on_input(
                        |size| {
                            let a: String = size.chars().filter(|c| c.is_digit(10)).collect();
                            if a.len() < 7 {
                                Message::AllocationFieldChanged(a)
                            }
                            else {
                                unsafe { Message::AllocationFieldChanged(a.get_unchecked(0..7).to_string()) }
                            }
                        }
                    )
            )
            .push(text("bytes").size(20))
            .padding(10);


        if self.rows.is_empty() {
            let c1 = container(but_col)
                .style(theme::Container::Custom(Box::new(ConStyle)));
            let c2 = container(menu_row)
                .style(theme::Container::Custom(Box::new(ConStyle))).width(Length::Fill);

            let rows = Row::new()
                .push(c1)
                .push(c2);

            let res = Column::new()
                .push(rows)
                .push(text("No memories").style(Color::from_rgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0)).horizontal_alignment(Horizontal::Center).width(Length::Fill));

            container(res)
                .padding(10)
                .style(theme::Container::Custom(Box::new(MainStyle))).width(Length::Fill).height(Length::Fill).into()
        }
        else {

            let pack_rows = Scrollable::new(Column::new().extend(self.rows.iter().map(|r| r.view().into())).padding([0, 10, 0, 0]));

            let c1 = container(but_col)
                .style(theme::Container::Custom(Box::new(ConStyle)));
            let c2 = container(menu_row)
                .style(theme::Container::Custom(Box::new(ConStyle))).width(Length::Fill);
            let c3 = container(pack_rows)
                .style(theme::Container::Custom(Box::new(MainStyle))).width(Length::Fill).height(Length::Fill);
            let rows = Row::new()
                .push(c1)
                .push(c2);

            let res = Column::new()
                .push(rows)
                .push(c3);

            container(res)
                .padding(10)
                .style(theme::Container::Custom(Box::new(MainStyle))).width(Length::Fill).height(Length::Fill).into()
        }
    }
}