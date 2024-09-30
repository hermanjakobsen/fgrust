extern crate crossterm;

use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, cursor::MoveTo, execute, style, terminal, terminal::{Clear, ClearType}, QueueableCommand};
use std::io::{stdout, Write};

fn main() {
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap();

    enable_raw_mode().unwrap();
    execute!(stdout(), cursor::Hide).unwrap();

    let mut stdout = stdout();
    let mut x = 0.0;
    let mut y = 0.0;
    let mut phase = 0.0;

    let width = 80;
    let height = 30;

    let mut dt = 1.0 / 60.0;
    let mut current_time = std::time::Instant::now();

    loop {
        let new_time = std::time::Instant::now();
        dt = new_time.duration_since(current_time).as_secs_f64();
        current_time = new_time;

        stdout.queue(MoveTo(0, 0)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();

        // for i in 0..height {
        //     for j in 0..width {
        //         stdout.queue(MoveTo(j, i)).unwrap();
        //         stdout.queue(style::PrintStyledContent("·".grey())).unwrap();
        //     }
        // }

        phase += 1.0 * dt;
        x = (width as f64 / 2.0) + (-(phase * 2.0).sin() * (width as f64 / 2.1));
        y = (height as f64 / 2.0) + ((phase * 2.0).cos() * (height as f64 / 2.1));

        // stdout.queue(MoveTo(clamped_x, clamped_y)).unwrap();
        // stdout.queue(style::PrintStyledContent(x.to_string().magenta())).unwrap();

        stdout.queue(MoveTo(x as u16 % width, y as u16 % height)).unwrap();
        stdout.queue(style::PrintStyledContent("◯".green())).unwrap();

        for i in 0..width {
            let x = i as f64;
            let y = (height as f64 / 2.0) + (phase + x / 10.0).sin() * 10.0;
            let clamped_y = y as u16 % height;
            stdout.queue(MoveTo(x as u16, clamped_y)).unwrap();
            stdout.queue(style::PrintStyledContent("█".magenta())).unwrap();
        }

        stdout.queue(MoveTo(0, 0)).unwrap();
        stdout.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));

        if crossterm::event::poll(std::time::Duration::from_millis(0)).unwrap() {
            if let crossterm::event::Event::Key(event) = crossterm::event::read().unwrap() {
                if event.code == crossterm::event::KeyCode::Char('q') {
                    println!("Exiting...");
                    break;
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show).unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
}