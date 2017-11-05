extern crate orbtk;
extern crate chrono;
extern crate orbclient;

use orbtk::{Rect, Window, Grid, Label, Button};
use orbtk::traits::{Place, Text, Border};
use chrono::prelude::*;
use orbclient::{Color};

fn main(){
    let cell_width = 90;
    let cell_height = 90;
    let cell_day_name_height = 16;
    let window_width = 7 * (cell_width + 8) + 8;
    let window_height = 5 * (cell_height + 8) + 16 + cell_day_name_height + 24;
    let mut window = Window::new(Rect::new(-1, -1, window_width, window_height), "Calendar");

    let date: DateTime<Local> = Local::now();

    {
        let string_date = date.format("%B %Y").to_string();
        let label_date = Label::new();
        let label_position = window_width / 2 - (string_date.len() * 8 /2) as u32;
        label_date.text(string_date)
            .size(300, 16)
            .position(label_position as i32, 8);
        window.add(&label_date);

        let offset = window_width / 2 - 100;
        let prev_month = Button::new();
        prev_month.text("<")
            .position(offset as i32, 6)
            .text_offset(5, 3)
            .size(20, 20);

        window.add(&prev_month);

        let next_month = Button::new();
        let offset = window_width / 2 + 80;
        next_month.text(">")
            .position(offset as i32, 6)
            .text_offset(5, 3)
            .size(20, 20);

        window.add(&next_month);
    }

    {

        let grid = Grid::new();
        grid.position(8, 8 + 16 + 8)
            .spacing(8, 8);

        // TODO(Bojan): Add start with Sunday option
        let day_names = &["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

        for (i, day) in day_names.iter().enumerate() {
            let label = Label::new();
            label.size(cell_width, cell_day_name_height).text(*day);
            grid.insert(i, 0, &label);
        }

        let mut day = 1;
        for idx in 0..5*7usize {

            if (idx as u32) < date.with_day(1).unwrap().weekday().number_from_monday() -1 {
                continue;
            }

            let d = date.with_day(day);
            match d {
                Some(x) => {

                    let cell = Label::new();
                    let text = format!("{}", x.day());
                    let text_offset = cell_width / 2 - (text.len() as u32 * 4);

                    cell.size(cell_width, cell_height)
                        .border(true)
                        .text(text)
                        .text_offset(text_offset as i32, (cell_width / 2 -8) as i32);
                    cell.bg.set(Color::rgb(255, 255, 255));

                    grid.insert(idx % 7, (idx / 7) + 1, &cell);
                },
                None => {}
            }
            day += 1;
        }

        window.add(&grid);
    }

    window.exec();
}
