use std::{cell::Cell, fmt};

pub struct Person<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub birth_year: u16,
    pub birth_month: u8,
}

impl<'a> Person<'a> {
    pub fn print_data(&self) {
        println!(
            "Name: {} {}, Birth Year: {}, Birth Month: {}",
            self.first_name, self.last_name, self.birth_year, self.birth_month
        );
    }
}

pub fn new_person<'a>(
    first_name: &'a str,
    last_name: &'a str,
    birth_year: u16,
    birth_month: u8,
) -> Person<'a> {
    Person {
        first_name,
        last_name,
        birth_year,
        birth_month,
    }
}

#[derive(Debug)]
pub enum VehicleColor {
    Red,
    Blue,
    Green,
    Black,
    White,
}

impl fmt::Display for VehicleColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            VehicleColor::Red => "Red",
            VehicleColor::Blue => "Blue",
            VehicleColor::Green => "Green",
            VehicleColor::Black => "Black",
            VehicleColor::White => "White",
        };
        write!(f, "{}", color)
    }
}

pub struct Vehicle<'a> {
    name: &'a str,
    year: u16,
    pub nick_name: Cell<String>,
    color: VehicleColor,
}

impl<'a> Vehicle<'a> {
    pub fn print_data(&self) {
        println!(
            "Name: {}, Year: {}, Nickname: {}, color: {}",
            self.name,
            self.year,
            self.nick_name.take(),
            format!("{}", self.color)
        );
    }

    pub fn paint(&mut self, color: VehicleColor) {
        self.color = color;
    }

    pub fn create_vehicle<'b>(
        name: &'b str,
        year: u16,
        nick_name: &'b str,
        color: VehicleColor,
    ) -> Vehicle<'b> {
        Vehicle {
            name,
            year,
            nick_name: Cell::new(nick_name.to_string()),
            color,
        }
    }
}
