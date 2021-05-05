use super::*;

use core::fmt::Write;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    style::TextStyleBuilder,
};
use heapless::{consts::*, String};
use keyberon::layout::Event;

#[derive(Copy, Clone, Default)]
pub struct Info {
    usb_connected: bool,
    hand: Option<Hand>,
    last_matrix: Option<Event>,
}

impl super::State for Info {
    type Messages = Option<Message>;
    fn write_to_display<DI, DSIZE>(&self, display: &mut GraphicsMode<DI, DSIZE>)
    where
        DSIZE: DisplaySize,
        DI: WriteOnlyDataCommand,
    {
        display.clear();
        let font_6x8 = TextStyleBuilder::new(Font6x8)
            .text_color(BinaryColor::On)
            .build();

        Text::new("hand:", Point::zero())
            .into_styled(font_6x8)
            .draw(display)
            .unwrap();

        let hand = match self.hand {
            Some(Hand::Left) => "left",
            Some(Hand::Right) => "right",
            None => "",
        };

        Text::new(hand, Point::new(36, 0))
            .into_styled(font_6x8)
            .draw(display)
            .unwrap();

        Text::new("usb:", Point::new(0, 13))
            .into_styled(font_6x8)
            .draw(display)
            .unwrap();

        let usb = match self.usb_connected {
            true => "true",
            false => "false",
        };

        Text::new(usb, Point::new(30, 13))
            .into_styled(font_6x8)
            .draw(display)
            .unwrap();

        Text::new("last mtx:", Point::new(0, 26))
            .into_styled(font_6x8)
            .draw(display)
            .unwrap();

        let mut last_matrix: String<U10> = heapless::String::new();
        match self.last_matrix {
            Some(Event::Press(i, j)) => write!(last_matrix, "p ({}, {})", i, j).unwrap(),
            Some(Event::Release(i, j)) => write!(last_matrix, "r ({}, {})", i, j).unwrap(),
            None => (),
        };

        Text::new(last_matrix.as_str(), Point::new(0, 39))
            .into_styled(font_6x8)
            .draw(display)
            .unwrap();

        display.flush().unwrap();
    }

    fn handle_event(&mut self, message: Message) -> Self::Messages {
        match message {
            Message::YouArePrimary => {
                defmt::info!("I am primary");
                self.hand = Some(Hand::Left);
                None
            }
            Message::YouAreSecondary => {
                defmt::info!("I am secondary");
                self.hand = Some(Hand::Right);
                None
            }
            Message::UsbConnected(is_connected) => {
                defmt::info!("Usb is connected");
                self.usb_connected = is_connected;
                None
            }
            Message::MatrixKeyPress(i, j) => {
                self.last_matrix = Some(Event::Press(i, j));
                if !self.usb_connected {
                    Some(Message::SecondaryKeyPress(i, 13 - j))
                } else {
                    None
                }
            }
            Message::MatrixKeyRelease(i, j) => {
                self.last_matrix = Some(Event::Release(i, j));
                if !self.usb_connected {
                    Some(Message::SecondaryKeyRelease(i, 13 - j))
                } else {
                    None
                }
            }
            Message::Ping => Some(Message::Pong),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Hand {
    Left,
    Right,
}