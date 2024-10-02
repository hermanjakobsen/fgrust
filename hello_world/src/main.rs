mod santa;

extern crate crossterm;

use crossterm::style::{SetForegroundColor, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, cursor::MoveTo, event, execute, queue, style, terminal, terminal::{Clear, ClearType}, QueueableCommand};
use std::io::{stdout, Stdout, Write};
use std::{thread, time};
use std::time::Instant;
use time::Duration;
use crossterm::event::{read, DisableMouseCapture, EnableMouseCapture};
use event::Event;
use crate::santa::SANTA_ASCII;

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

struct Cell {
    x: u16,
    y: u16,
    c: char,
    color: style::Color,
}

impl Cell {
    fn new(x: u16, y: u16, c: char, color: style::Color) -> Cell {
        Cell { x, y, c, color }
    }
}

fn main() {
    let (width, height) = terminal::size().unwrap();

    let width = (width - 1).clamp(0, 200);
    let height = 30;

    let mut stdout = stdout();
    let mut screen_buffer: Vec<Cell> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            screen_buffer.push(Cell::new(j, i, ' ', style::Color::Rgb { r: 255, g: 255, b: 255 }));
        }
    }

    enable_raw_mode().unwrap();

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        EnableMouseCapture,
        Clear(ClearType::All)
    ).unwrap();

    let snow_flake_sprites = vec!['*', '·', '•'];
    let mut snow_flakes: Vec<Snowflake> = Vec::new();
    for _ in 0..100 {
        snow_flakes.push(Snowflake {
            x: (width as f64 * rand::random::<f64>()).floor(),
            y: ((height-1) as f64 * rand::random::<f64>()).floor(),
            speed: (rand::random::<f64>() * 0.5) + 2.8,
            sprite: snow_flake_sprites[(rand::random::<u16>() % snow_flake_sprites.len() as u16) as usize],
        });
    }

    let mut phase = 0.0;

    let mut dt;
    let mut current_time = Instant::now();
    let mut mouse_position = (0, 0);
    let mut mouse_down = false;

    loop {
        dt = delta_time(&mut current_time);

        phase += 1.0 * dt;

        // clear screen buffer
        for i in 0..screen_buffer.len() {
            screen_buffer[i].c = ' ';
        }

        // draw_sine_wave(&mut screen_buffer, width, height, phase);
        draw_santa(&mut screen_buffer, width, height);
        draw_snow_flakes(&mut screen_buffer, width, height, phase, dt, &mut snow_flakes);
        draw_question(&mut screen_buffer, width, height, mouse_position, mouse_down);
        draw_ground(&mut screen_buffer, width, height);
        draw_debug_info(&mut screen_buffer, width, height, mouse_position, mouse_down, dt);

        // render screen buffer cells
        for cell in screen_buffer.iter() {
            queue!(
                stdout,
                MoveTo(cell.x, cell.y),
                SetForegroundColor(cell.color),
                style::PrintStyledContent(cell.c.stylize())
            ).unwrap();
        }

        // for i in 0..height {
        //     let start = (i * width) as usize;
        //     let end = ((i + 1) * width) as usize;
        //     let line: String = screen_buffer[start..end].iter().map(|cell| cell.c).collect();
        //     queue!(
        //         stdout,
        //         MoveTo(0, i),
        //         style::PrintStyledContent(line.stylize())
        //     ).unwrap();
        // }

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

fn draw_debug_info(screen_buffer: &mut Vec<Cell>, width: u16, height: u16, mouse_position: (u16, u16), mouse_down: bool, dt: f64) {
    let fps_str = format!("FPS: {:.2}", 1.0 / dt);
    for (i, c) in fps_str.chars().enumerate() {
        let index = (0 * width + i as u16) as usize;
        screen_buffer[index].c = c;
    }

    let mouse_pos_str = format!("Mouse: ({}, {})", mouse_position.0, mouse_position.1);
    for (i, c) in mouse_pos_str.chars().enumerate() {
        let index = (1 * width + i as u16) as usize;
        screen_buffer[index].c = c;
    }

    let mouse_down_str = format!("Mouse Down: {}", mouse_down);
    for (i, c) in mouse_down_str.chars().enumerate() {
        let index = (2 * width + i as u16) as usize;
        screen_buffer[index].c = c;
    }
}

fn draw_sine_wave(screen_buffer: &mut Vec<Cell>, width: u16, height: u16, phase: f64) {
    for i in 0..width {
        let x = i as f64;
        let y = (height as f64 / 2.0) + (phase + x / 10.0).sin() * 10.0;
        let clamped_y = y as u16 % height;

        let index = (clamped_y * width + x as u16) as usize;
        screen_buffer[index].c = '█';
    }
}

fn draw_ground(screen_buffer: &mut Vec<Cell>, width: u16, height: u16) {
    let ground = width * (height - 1);

    for i in 0..width {
        screen_buffer[(ground + i) as usize].c = '█';
    }
}

fn draw_snow_flakes(screen_buffer: &mut Vec<Cell>, width: u16, height: u16, phase: f64, dt: f64, snow_flakes: &mut Vec<Snowflake>) {
    for snow_flake in snow_flakes.iter_mut() {
        snow_flake.y += snow_flake.speed * dt;
        snow_flake.x += (phase + snow_flake.x / 10.0).sin() * 0.02;

        let x = (snow_flake.x as u16).clamp(0, width-1);
        let y = snow_flake.y as u16;

        if y >= height-2 {
            snow_flake.y = 0.0;
            snow_flake.x = (width - 1) as f64 * rand::random::<f64>();
        }

        let index = ((y * width) + x) as usize;
        screen_buffer[index].c = snow_flake.sprite;
    }
}

fn draw_santa(screen_buffer: &mut Vec<Cell>, width: u16, height: u16) {
    let lines = SANTA_ASCII.lines();
    let x = 2;
    let offset = height - 1 - lines.clone().count() as u16;

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            let index = ((offset + i as u16) * width + x + j as u16) as usize;
            screen_buffer[index].c = c;
        }
    }
}

fn draw_question(screen_buffer: &mut Vec<Cell>, width: u16, height: u16, mouse_position: (u16, u16), mouse_down: bool) { {
    let question = "What is the answer to life, the universe, and everything?";
    draw_text_box(screen_buffer, width, height, question, 0, -5, (0, 0), false);

    let hover1 = draw_text_box(screen_buffer, width, height, "42", -20, 0, mouse_position, mouse_down);
    let hover2 = draw_text_box(screen_buffer, width, height, "24", 0, 0, mouse_position, mouse_down);
    let hover3 = draw_text_box(screen_buffer, width, height, "69", 20, 0, mouse_position, mouse_down);

    let correct = hover1 && mouse_down;
    let wrong1 = hover2 && mouse_down;
    let wrong2 = hover3 && mouse_down;

    if correct {
        draw_text_box(screen_buffer, width, height, "Correct!", 0, 5, (0, 0), false);
    } else if wrong1 || wrong2 {
        draw_text_box(screen_buffer, width, height, "Wrong!", 0, 5, (0, 0), false);
    }
}

fn draw_text_box(screen_buffer: &mut Vec<Cell>, width: u16, height: u16, q: &str, x_offset: i16, y_offset: i16, mouse_position: (u16, u16), mouse_down: bool) -> bool{
    let question = q;
    let x_origin = ((width as i16 - question.len() as i16) / 2 + x_offset) as u16;
    let y_origin = (height as i16 / 2 + y_offset) as u16;

    let fancy_top_border = "╭".to_string() + &"─".repeat(question.len() + 4) + "╮";
    let fancy_bottom_border = "╰".to_string() + &"─".repeat(question.len() + 4) + "╯";

    let mut color = style::Color::Rgb {
        r: 255,
        g: 255,
        b: 255,
    };

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
        let index = ((y_origin + i) * width + x_origin - 2) as usize;
        let line = " ".repeat(question.len() + 4);
        for (j, c) in line.chars().enumerate() {
            screen_buffer[index + j].c = c;
        }
    }

    {
        let index = ((y_origin - 1) * width + x_origin - 3) as usize;
        let line = &fancy_top_border;
        for (j, c) in line.chars().enumerate() {
            screen_buffer[index + j].c = c;
            screen_buffer[index + j].color = color;
        }
    }

    {
        let index = ((y_origin + 1) * width + x_origin - 3) as usize;
        let line = &fancy_bottom_border;
        for (j, c) in line.chars().enumerate() {
            screen_buffer[index + j].c = c;
            screen_buffer[index + j].color = color;
        }
    }

    {
        let index = ((y_origin) * width + x_origin - 3) as usize;
        screen_buffer[index].c = '│';
        screen_buffer[index].color = color;
    }

    {
        let index = ((y_origin) * width + x_origin + question.len() as u16 + 2) as usize;
        screen_buffer[index].c = '│';
        screen_buffer[index].color = color;
    }


    for (i, c) in question.chars().enumerate() {
        let index = ((y_origin) * width + x_origin + i as u16) as usize;
        screen_buffer[index].c = c;
        screen_buffer[index].color = color;
    }

    is_hovered
}
}