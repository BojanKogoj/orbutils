extern crate orbtk;
extern crate chrono;
extern crate orbclient;

use orbtk::{Rect, Window, Grid, Label, Button};
use orbtk::traits::{Place, Text, Border, Click};
use chrono::prelude::*;
use chrono::Duration;
use orbclient::{Color};
use std::ops::{Sub, Add};
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::RefMut;


fn main(){
    let cell_width = 90;
    let cell_height = 90;
    let cell_day_name_height = 16;
    let window_width = 7 * (cell_width + 8) + 8;
    let window_height = 5 * (cell_height + 8) + 16 + cell_day_name_height + 24;
    let mut window = Window::new(Rect::new(-1, -1, window_width, window_height), "Calendar");

    // Setting to 5th day, so +- 31 days always lands on the next month
    let date: RefCell<DateTime<Local>> = RefCell::new(Local::now().with_day(5).unwrap());
    

    let grid = Grid::new();
    grid.position(8, 8 + 16 + 8)
        .spacing(8, 8);

    let label_date = Label::new();
    redraw_month_label(&label_date, *date.borrow(), window_width);
    window.add(&label_date);

    {
        let offset = window_width / 2 - 100;
        let prev_month = Button::new();

        let _grid = grid.clone();
        let _label_date = label_date.clone();
        let _date = date.clone();

        prev_month.text("<")
            .position(offset as i32, 6)
            .text_offset(5, 3)
            .size(20, 20)
            .on_click(move |_, _| {
                let mut date: RefMut<DateTime<chrono::Local>> = _date.borrow_mut();
                *date = date.sub(Duration::days(31));
                draw_grid(&_grid, *date, cell_width, cell_height, cell_day_name_height);
                redraw_month_label(&_label_date, *date, window_width);
            });
        window.add(&prev_month);
    }

    {

        let _grid = grid.clone();
        let _label_date = label_date.clone();

        let next_month = Button::new();
        let offset = window_width / 2 + 80;
        let _date = date.clone();

        next_month.text(">")
            .position(offset as i32, 6)
            .text_offset(5, 3)
            .size(20, 20)
            .on_click(move |_, _| {
                let mut date: RefMut<DateTime<chrono::Local>> = _date.borrow_mut();
                *date = date.add(Duration::days(31));
                draw_grid(&_grid, *date, cell_width, cell_height, cell_day_name_height);
                redraw_month_label(&_label_date, *date, window_width);
            });

        window.add(&next_month);
    }

    let mut _date: RefMut<DateTime<chrono::Local>> = date.borrow_mut();
    draw_grid(&grid, *_date, cell_width, cell_height, cell_day_name_height);

    window.add(&grid);

    window.exec();
}

fn draw_grid(grid: &Grid, date: DateTime<Local>, cell_width: u32, cell_height: u32, cell_day_name_height: u32) {

    grid.clear();

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
}

fn redraw_month_label(label: &Label, date: DateTime<Local>, window_width: u32) {

    let string_date = date.format("%B %Y").to_string();
    let label_position = window_width / 2 - (string_date.len() * 8 /2) as u32;
    label.text(string_date)
        .size(300, 16)
        .position(label_position as i32, 8);
}
