use iced::{Background, Border, Color, Shadow, Theme, Vector};

use iced::border::Radius;
use iced::widget::{button, checkbox, container};
use iced::widget::checkbox::Appearance;

pub struct RowStyle;

impl container::StyleSheet for RowStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::from_rgb(194.0 / 255.0,157.0 / 255.0,137.0 / 255.0)),
            border: Border {
                color: Color::from_rgb(81.0 / 255.0,70.0 / 255.0,71.0 / 255.0),
                width: 3.0,
                radius: Radius::from(7),
            },
            background: Some(Background::Color(Color::from_rgb(0.141, 0.133, 0.149))),
            shadow: Shadow::default(),
        }
    }
}

pub struct ConStyle;

impl container::StyleSheet for ConStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::from_rgb(194.0 / 255.0,157.0 / 255.0,137.0 / 255.0)),
            border: Border {
                color: Color::from_rgb(125.0 / 255.0,109.0 / 255.0,118.0 / 255.0),
                width: 3.0,
                radius: Radius::from(7),
            },
            background: Some(Background::Color(Color::from_rgb(0.141, 0.133, 0.149))),
            shadow: Shadow::default(),
        }
    }
}
pub struct MainStyle;

impl container::StyleSheet for MainStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            border: Border::with_radius(0),
            background: Some(Background::Color(Color::from_rgb(0.141, 0.133, 0.149))),
            shadow: Shadow::default(),
        }
    }
}

pub enum ButtonStyle {
    Standard,
    DeleteButton,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: match self {
                Self::Standard => Some(Background::Color(Color::from_rgb(0.059, 0.463, 0.702))),
                Self::DeleteButton => Some(Background::Color(Color::from_rgb(0.9, 0.1, 0.3))),
            },
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::DeleteButton => Border::with_radius(5),
            },
            shadow_offset: match self {
                Self::Standard => Vector::new(0.0, 2.0),
                Self::DeleteButton => Vector::new(0.0, 2.0),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                Self::DeleteButton => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
            },
            text_color: {
                if theme == &Theme::Light {
                    match self {
                        Self::Standard => Color::WHITE,
                        Self::DeleteButton => Color::WHITE,
                    }
                } else {
                    match self {
                        Self::Standard => Color::WHITE,
                        Self::DeleteButton => Color::WHITE,
                    }
                }
            },
        }

    }
    fn hovered(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: match self {
                Self::Standard => Some(Background::Color(Color::from_rgb(0.4, 0.7, 1.0))),
                Self::DeleteButton => Some(Background::Color(Color::from_rgb(0.98, 0.6, 0.6))),
            },
            ..self.active(theme)
        }
    }
}

pub struct CheckBoxStyle;

impl checkbox::StyleSheet for CheckBoxStyle {
    type Style = Theme;

    fn active(&self, _: &Self::Style, is_checked: bool) -> Appearance {
        let background = if is_checked {
            Color::from_rgb(194.0 / 255.0, 157.0 / 255.0, 137.0 / 255.0) // цвет при включенном чекбоксе
        } else {
            Color::from_rgb(0.141, 0.133, 0.149) // цвет при выключенном чекбоксе
        };

        Appearance {
            background: Background::Color(background),
            icon_color: Color::from_rgb(1.0, 1.0, 1.0), // белая иконка галочки
            border: Border {
                color: Color::from_rgb(81.0 / 255.0, 70.0 / 255.0, 71.0 / 255.0), // цвет границы
                width: 3.0,
                radius: Radius::from(7),
            },
            text_color: Some(Color::from_rgb(194.0 / 255.0, 157.0 / 255.0, 137.0 / 255.0)), // цвет текста
        }
    }

    fn hovered(&self, _: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let background = if is_checked {
            Color::from_rgb(194.0 / 255.0, 157.0 / 255.0, 137.0 / 255.0) // цвет при наведении на включенный чекбокс
        } else {
            Color::from_rgb(0.2, 0.2, 0.2) // цвет при наведении на выключенный чекбокс
        };

        Appearance {
            background: Background::Color(background),
            icon_color: Color::from_rgb(1.0, 1.0, 1.0),
            border: Border {
                color: Color::from_rgb(81.0 / 255.0, 70.0 / 255.0, 71.0 / 255.0),
                width: 3.0,
                radius: Radius::from(7),
            },
            text_color: Some(Color::from_rgb(194.0 / 255.0, 157.0 / 255.0, 137.0 / 255.0)),
        }
    }
}
