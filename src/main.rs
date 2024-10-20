mod ascii;
mod drawing;

extern crate crossterm;

use crossterm::event::{read, DisableMouseCapture, EnableMouseCapture};
use crossterm::style::{SetForegroundColor, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    cursor,
    cursor::MoveTo,
    event, execute, queue, style, terminal,
    terminal::{Clear, ClearType},
};
use drawing::{
    clamp_screen, draw_ascii, draw_debug_info, draw_ground, draw_question, draw_snow_flakes,
    reset_screen_buffer, Cell, Snowflake,
};
use event::Event;
use std::io::{stdout, Write};
use std::time::Instant;
use std::{thread, time};
use time::Duration;

fn delta_time(current_time: &mut Instant) -> f64 {
    let new_time = Instant::now();
    let dt = new_time.duration_since(*current_time).as_secs_f64();
    *current_time = new_time;
    dt
}

fn main() {
    let (width, height) = clamp_screen(terminal::size().unwrap());
    let mut resize = true;

    let mut stdout = stdout();
    let mut screen_buffer: Vec<Cell> = Vec::new();

    enable_raw_mode().unwrap();

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        EnableMouseCapture,
        Clear(ClearType::All)
    )
    .unwrap();

    let snow_flake_sprites = vec!['*', '·', '•'];
    let mut snow_flakes: Vec<Snowflake> = Vec::new();
    for _ in 0..100 {
        snow_flakes.push(Snowflake {
            x: (width as f64 * rand::random::<f64>()).floor(),
            y: ((height - 1) as f64 * rand::random::<f64>()).floor(),
            speed: (rand::random::<f64>() * 0.5) + 2.8,
            sprite: snow_flake_sprites
                [(rand::random::<u16>() % snow_flake_sprites.len() as u16) as usize],
        });
    }

    let mut phase = 0.0;

    let mut dt;
    let mut current_time = Instant::now();
    let mut mouse_position = (0, 0);
    let mut mouse_down = false;

    loop {
        if resize {
            reset_screen_buffer(&mut screen_buffer, width, height);
            resize = false;
        }

        dt = delta_time(&mut current_time);

        phase += 1.0 * dt;

        // clear screen buffer
        for i in 0..screen_buffer.len() {
            screen_buffer[i].c = ' ';
        }

        draw_ascii(
            &mut screen_buffer,
            ascii::SANTA,
            width,
            height,
            2,
            height - 20,
        );
        draw_snow_flakes(
            &mut screen_buffer,
            width,
            height,
            phase,
            dt,
            &mut snow_flakes,
        );
        draw_ascii(
            &mut screen_buffer,
            ascii::SYSTEK,
            width,
            height,
            width / 2 - 32,
            1,
        );
        draw_question(
            &mut screen_buffer,
            width,
            height,
            mouse_position,
            mouse_down,
        );
        draw_ground(&mut screen_buffer, width, height);
        draw_debug_info(
            &mut screen_buffer,
            width,
            height,
            mouse_position,
            mouse_down,
            dt,
        );

        // render screen buffer cells
        for cell in screen_buffer.iter() {
            queue!(
                stdout,
                MoveTo(cell.x, cell.y),
                SetForegroundColor(cell.color),
                style::PrintStyledContent(cell.c.stylize())
            )
            .unwrap();
        }

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(16));

        if event::poll(Duration::from_millis(0)).unwrap() {
            let raw = read();

            if raw.is_err() {
                continue;
            }

            let event = raw.unwrap();

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
        }
    }

    disable_raw_mode().unwrap();
    execute!(
        stdout,
        cursor::Show,
        terminal::LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
}
