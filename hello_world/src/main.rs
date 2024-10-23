extern crate crossterm;
mod ascii;
mod screen;

use crate::screen::Screen;
use crossterm::event::read;
use crossterm::{event, style, terminal};
use event::Event;
use std::io::{stdout};
use std::time::Instant;
use std::{thread, time};
use time::Duration;

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
    sprite: char,
}

fn clamp_screen((width, height) : (u16, u16)) -> (u16, u16) {
    let new_width = (width - 1).clamp(80, 200);
    let new_height = height.clamp(40, 70);
    (new_width, new_height)
}

fn main() {
    let (mut width, mut height) = clamp_screen(terminal::size().unwrap());
    let mut resize = true;

    let mut screen = Screen::new(stdout(), width, height);
    screen.init();

    let snow_flake_sprites = vec!['*', '·', '•'];
    let mut snow_flakes: Vec<Snowflake> = Vec::new();
    for _ in 0..100 {
        snow_flakes.push(Snowflake {
            x: (width as f64 * rand::random::<f64>()).floor(),
            y: ((height-1) as f64 * rand::random::<f64>()).floor(),
            speed: (rand::random::<f64>() * 0.5) + 0.4,
            sprite: snow_flake_sprites[(rand::random::<u16>() % snow_flake_sprites.len() as u16) as usize],
        });
    }

    let mut phase = 0.0;

    let mut dt;
    let mut current_time = Instant::now();
    let mut mouse_position = (0, 0);
    let mut mouse_down = false;

    loop {
        if resize {
            screen.resize(width, height);
            resize = false;
        }

        dt = delta_time(&mut current_time);
        phase += 1.0 * dt;

        screen.clear();

        draw_ascii(&mut screen, ascii::SANTA, 2, height - 20);
        draw_snow_flakes(&mut screen, width, height, phase, dt, &mut snow_flakes);
        draw_ascii(&mut screen, ascii::SYSTEK, width / 2 - 32, 1);
        draw_ground(&mut screen, width, height);
        draw_question(&mut screen, width, height, mouse_position, mouse_down);
        draw_debug_info(&mut screen, mouse_position, mouse_down, dt);

        screen.render();

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

            if let Event::Resize(w, h) = event {
                let (new_width, new_height) = clamp_screen((w, h));
                if new_width != width || new_height != height {
                    width = new_width;
                    height = new_height;

                    resize = true;
                }
            }
        }
    }

    screen.cleanup();
}

fn draw_debug_info(screen: &mut Screen, mouse_position: (u16, u16), mouse_down: bool, dt: f64) {
    let fps_str = format!("FPS: {:.2}", 1.0 / dt);
    for (i, c) in fps_str.chars().enumerate() {
        screen.set_cell(i as u16, 0, c, style::Color::White);
    }

    let mouse_pos_str = format!("Mouse: ({}, {})", mouse_position.0, mouse_position.1);
    for (i, c) in mouse_pos_str.chars().enumerate() {
        screen.set_cell(i as u16, 1, c, style::Color::White);
    }

    let mouse_down_str = format!("Mouse Down: {}", mouse_down);
    for (i, c) in mouse_down_str.chars().enumerate() {
        screen.set_cell(i as u16, 2, c, style::Color::White);
    }
}

// fn draw_sine_wave(screen_buffer: &mut Vec<Cell>, width: u16, height: u16, phase: f64) {
//     for i in 0..width {
//         let x = i as f64;
//         let y = (height as f64 / 2.0) + (phase + x / 10.0).sin() * 10.0;
//         let clamped_y = y as u16 % height;
//
//         let index = (clamped_y * width + x as u16) as usize;
//         screen_buffer[index].c = '█';
//     }
// }

fn draw_ground(screen: &mut Screen, width: u16, height: u16) {
    for i in 0..width {
        screen.set_cell(i, height - 1, '█', style::Color::White);
    }
}

fn draw_snow_flakes(screen: &mut Screen, width: u16, height: u16, phase: f64, dt: f64, snow_flakes: &mut Vec<Snowflake>) {
    for snow_flake in snow_flakes.iter_mut() {
        snow_flake.y += snow_flake.speed * dt;
        snow_flake.x += (phase + snow_flake.x / 10.0).sin() * 0.02;

        let x = (snow_flake.x as u16).clamp(0, width - 1);
        let y = (snow_flake.y as u16).clamp(0, height - 1);

        if y >= height - 1 {
            snow_flake.y = 0.0;
            snow_flake.x = (width - 1) as f64 * rand::random::<f64>();
        }

        screen.set_cell(x, y, snow_flake.sprite, style::Color::White);
    }
}

fn draw_ascii(screen: &mut Screen, ascii: &str, x: u16, y: u16) {
    let lines = ascii.lines();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            screen.set_cell(x + j as u16, y + i as u16, c, style::Color::White);
        }
    }
}

fn draw_question(screen: &mut Screen, width: u16, height: u16, mouse_position: (u16, u16), mouse_down: bool) {
    let question = "What is the answer to life, the universe, and everything?";
    draw_text_box(screen, width, height, question, 0, -5, (0, 0), false);

    let hover1 = draw_text_box(screen, width, height, "42", -20, 0, mouse_position, mouse_down);
    let hover2 = draw_text_box(screen, width, height, "24", 0, 0, mouse_position, mouse_down);
    let hover3 = draw_text_box(screen, width, height, "69", 20, 0, mouse_position, mouse_down);

    if mouse_down {
        let correct = hover1 && !hover2 && !hover3;
        if correct {
            draw_text_box(screen, width, height, "Correct!", 0, 5, (0, 0), false);
        } else {
            draw_text_box(screen, width, height, "Wrong!", 0, 5, (0, 0), false);
        }
    }
}

fn draw_text_box(screen: &mut Screen, width: u16, height: u16, q: &str, x_offset: i16, y_offset: i16, mouse_position: (u16, u16), mouse_down: bool) -> bool{
    let question = q;
    let x_origin = ((width as i16 - question.len() as i16) / 2 + x_offset) as u16;
    let y_origin = (height as i16 / 2 + y_offset) as u16;

    let fancy_top_border = "╭".to_string() + &"─".repeat(question.len() + 4) + "╮";
    let fancy_bottom_border = "╰".to_string() + &"─".repeat(question.len() + 4) + "╯";

    let mut color = style::Color::White;

    let mut is_hovered = false;
    if mouse_position.0 >= x_origin - 3 &&
        mouse_position.0 <= x_origin + question.len() as u16 + 2 &&
        mouse_position.1 >= y_origin - 1 &&
        mouse_position.1 <= y_origin + 1 {

        if mouse_down {
            color = style::Color::Rgb {
                r: 0,
                g: 255,
                b: 0,
            };
        } else {
            color = style::Color::Rgb {
                r: 255,
                g: 255,
                b: 0,
            };
        }
        is_hovered = true;
    }

    for i in 0..3 {
        let line = " ".repeat(question.len() + 4);
        for (j, c) in line.chars().enumerate() {
            screen.set_cell(x_origin - 2 + j as u16, y_origin + i as u16, c, style::Color::White);
        }
    }

    {
        let line = &fancy_top_border;
        for (j, c) in line.chars().enumerate() {
            screen.set_cell(x_origin - 3 + j as u16, y_origin - 1, c, color);
        }
    }

    {
        let line = &fancy_bottom_border;
        for (j, c) in line.chars().enumerate() {
            screen.set_cell(x_origin - 3 + j as u16, y_origin + 1, c, color);
        }
    }

    {
        screen.set_cell(x_origin - 3, y_origin, '│', color);
    }

    {
        screen.set_cell(x_origin + question.len() as u16 + 2, y_origin, '│', color);
    }

    for (i, c) in question.chars().enumerate() {
        screen.set_cell(x_origin + i as u16, y_origin, c, color);
    }

    is_hovered
}