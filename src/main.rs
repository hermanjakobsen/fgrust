extern crate crossterm;
mod ascii;
mod screen;
mod snowflakes;
mod days;
mod drawing;

use crate::screen::Screen;
use crossterm::event::read;
use crossterm::{event, terminal};
use event::Event;
use std::io::{stdout, Error};
use std::time::Instant;
use std::time;
use time::Duration;

use days::{create_quiz_day, CalendarDay, RunStatus};
use drawing::{
    draw_ascii, draw_calendar, draw_debug_info, draw_ground, 
};

fn delta_time(previous_time: &mut Instant) -> f64 {
    let new_time = Instant::now();
    let dt = new_time.duration_since(*previous_time).as_nanos() as f64 / 1_000_000_000.0;
    *previous_time = new_time;
    dt
}

struct Snowflake {
    x: f64,
    y: f64,
    speed: f64,
    sprite: char,
}

fn main() -> Result<(), Error> {
    let mut resize = true;

    let mut screen = Screen::new(stdout(), terminal::size()?);
    screen.init()?;

    let mut snow_flakes: Vec<Snowflake> = snowflakes::create(screen.width(), screen.height());

    let mut dt;
    let mut previous_time = Instant::now();

    let mut mouse_position = (0, 0);
    let mut mouse_down = false;
    let mut phase = 0.0;


    let days = [
        create_quiz_day(
            "What is the answer to life, the universe, and everything?",
            "42",
            &["24", "69"],
        ),
        create_quiz_day("This is question 2", "42", &["99", "100", "wrong", "no"]),
    ];
    let mut day_to_run: Option<usize> = None;
    let mut day_status: RunStatus = RunStatus::READY;

    loop {
        dt = delta_time(&mut previous_time);

        if resize {
            screen.resize(terminal::size()?);
            resize = false;

            snow_flakes = snowflakes::create(screen.width(), screen.height());
        }

        phase += dt;
        snowflakes::update(screen.width(), screen.height(), phase, dt, &mut snow_flakes);

        screen.clear();
        let screen_height = screen.height();
        let screen_width = screen.width();
        draw_ascii(&mut screen, ascii::SANTA, 2, screen_height - 20);
        snowflakes::draw(&mut screen, &snow_flakes);
        draw_ascii(&mut screen, ascii::SYSTEK, screen_width / 2 - 32, 1);
        draw_ground(&mut screen);


        if event::poll(Duration::from_millis(0))? {
            let raw = read();

            if raw.is_err() {
                continue;
            }

            let event = raw?;

            if let Event::Key(event) = event {
                if event.code == event::KeyCode::Char('q') {
                    println!("Exiting...");
                    break;
                }
            }
            if let Event::Mouse(event) = event {
                if event.kind == event::MouseEventKind::Down(event::MouseButton::Left) {
                    mouse_down = true;
                }
                if event.kind == event::MouseEventKind::Up(event::MouseButton::Left) {
                    mouse_down = false;
                }
                if event.kind == event::MouseEventKind::Moved {
                    mouse_position = (event.column, event.row);
                }
            }

            if let Event::Resize(_w, _h) = event {
                resize = true;
            }
        }
    }


        if day_to_run.is_none() || day_status == RunStatus::CORRECT {
            day_to_run = draw_calendar(
                &mut screen,
                mouse_position,
                mouse_down,
                &days,
            );
            day_status = RunStatus::READY;
        } else {
            day_status = days.get(day_to_run.unwrap()).unwrap().tick(
                &mut screen,
                mouse_position,
                mouse_down,
            );
        }

        draw_debug_info(&mut screen, mouse_position, mouse_down, dt, day_to_run, &day_status);

        screen.render();
        screen.cleanup()?;

        Ok(())
}
