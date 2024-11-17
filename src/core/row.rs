use iced::{theme, Length};
use iced::Alignment::Center;
use iced::widget::{container, text, Checkbox, Container, Space};

use crate::core::app::Message;
use crate::gui::styles::{CheckBoxStyle, RowStyle};

#[derive(Clone, Debug)]
pub struct PackageRow {
    pub addr: i64,
    pub mem: Box<[u8]>,
    pub checked: bool,
}

impl PackageRow {
    pub fn new(size: usize) -> Self {
        let mem = vec![0_u8; size].into_boxed_slice();
        let addr = mem.as_ptr() as i64;
        Self {
            mem,
            addr,
            checked: false,
        }
    }

    pub fn get_size(&self) -> usize {
        self.mem.len() * size_of::<u8>()
    }

    pub fn clear_memory(&mut self) {
        self.mem = Vec::new().into_boxed_slice();
    }

    pub fn get_addr(&self) -> String {
        format!["Address: {};", self.addr]
    }

    pub fn get_mem_size_str(&self) -> String {
        let size = self.get_size();

        if size as f64 / 1_000_000.0 >= 1.0 {
            format!["Size: {} MB", size as f64 / 1_000_000.0]
        }
        else if size as f64 / 1000.0 >= 1.0 {
            format!["Size: {} KB", size as f64 / 1_000.0]
        }
        else {
            format!["Size: {} Bytes", size]
        }
    }

    pub fn view(&self) -> Container<'static, Message> {
        let addr_clone = self.addr.clone();

        let row = iced::widget::Row::new()
            .push(Space::with_width(20))
            .push(Checkbox::new("", self.checked).on_toggle(move |_| Message::ToggleCheck(addr_clone)).style(theme::Checkbox::Custom(Box::new(CheckBoxStyle))))
            .push(Space::with_width(10))
            .push(text(self.get_addr()).size(20))
            .push(Space::with_width(10))
            .push(text(self.get_mem_size_str()).size(20))
            .push(Space::with_width(Length::Fill))
            .height(60)
            .align_items(Center);

        container(row)
            .style(theme::Container::Custom(Box::new(RowStyle)))
    }
}