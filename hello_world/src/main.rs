extern crate crossterm;

use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, cursor::MoveTo, event, execute, queue, style, terminal, terminal::{Clear, ClearType}, QueueableCommand};
use std::io::{stdout, Stdout, Write};
use std::{thread, time};
use std::time::Instant;
use time::Duration;
use event::Event;

fn delta_time(current_time: &mut Instant) -> f64 {
    let new_time = Instant::now();
    let dt = new_time.duration_since(*current_time).as_secs_f64();
    *current_time = new_time;
    dt
}

struct Snowflake {
    x: f64,
    y: f64,
    speed: f64,
}

fn main() {
    let width = 200;
    let height = 30;
    let mut stdout = stdout();

    enable_raw_mode().unwrap();

    queue!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide
    ).unwrap();
    stdout.flush().unwrap();

    let mut snow_flakes: Vec<Snowflake> = Vec::new();
    for _ in 0..100 {
        snow_flakes.push(Snowflake {
            x: (width as f64 * rand::random::<f64>()).floor(),
            y: (height as f64 * rand::random::<f64>()).floor(),
            speed: (rand::random::<f64>() * 0.5) + 2.8,
        });
    }

    let mut phase = 0.0;

    let mut dt;
    let mut current_time = Instant::now();

    loop {
        dt = delta_time(&mut current_time);

        queue!(
            stdout,
            MoveTo(0, 0),
            Clear(ClearType::All)
        ).unwrap();

        phase += 1.0 * dt;

        draw_dots(&mut stdout, width, height);
        draw_sine_wave(&mut stdout, width, height, phase);
        draw_snow_flakes(&mut stdout, width, height, phase, dt, &mut snow_flakes);

        stdout.queue(MoveTo(0, 0)).unwrap();
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(100));

        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                if event.code == event::KeyCode::Char('q') {
                    println!("Exiting...");
                    break;
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(
        stdout,
        cursor::Show,
        terminal::LeaveAlternateScreen
    ).unwrap();
}

fn draw_dots(stdout: &mut Stdout, width: u16, height: u16) {
    for i in 0..=height {
        for j in 0..=width {
            queue!(
                stdout,
                MoveTo(j, i),
                style::PrintStyledContent("·".grey())
            ).unwrap();
        }
    }
}

fn draw_sine_wave(stdout: &mut Stdout, width: u16, height: u16, phase: f64) {
    for i in 0..width {
        let x = i as f64;
        let y = (height as f64 / 2.0) + (phase + x / 10.0).sin() * 10.0;
        let clamped_y = y as u16 % height;

        queue!(
            stdout,
            MoveTo(x as u16, clamped_y),
            style::PrintStyledContent("█".magenta())
        ).unwrap();
    }
}

fn draw_snow_flakes(stdout: &mut Stdout, width: u16, height: u16, phase: f64, dt: f64, snow_flakes: &mut Vec<Snowflake>) {
    for snow_flake in snow_flakes.iter_mut() {
        snow_flake.y += snow_flake.speed * dt;
        snow_flake.x += (phase + snow_flake.x / 10.0).sin() * 0.3;

        let x = snow_flake.x as u16;
        let mut y = snow_flake.y as u16;

        if y > height {
            snow_flake.y = 0.0;
            snow_flake.x = (width as f64 * rand::random::<f64>()).floor();
            y = 0;
        }

        queue!(
            stdout,
            MoveTo(x, y),
            style::PrintStyledContent("*".white())
        ).unwrap();
    }
}