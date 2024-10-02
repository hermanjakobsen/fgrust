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
    sprite: String,
}

const SANTA: &str = r#"
                    _...
              o_.-"`    `\
       .--.  _ `'-._.-'""-;     _
     .'    \`_\_  {_.-a"a-}  _ / \
   _/     .-'  '. {c-._o_.){\|`  |
  (@`-._ /       \{    ^  } \\ _/
   `~\  '-._      /'.     }  \}  .-.
     |>:<   '-.__/   '._,} \_/  / ())
     |     >:<   `'---. ____'-.|(`"`
     \            >:<  \\_\\_\ | ;
      \                 \\-{}-\/  \
       \                 '._\\'   /)
        '.                       /(
          `-._ _____ _ _____ __.'\ \
            / \     / \     / \   \ \
         _.'/^\'._.'/^\'._.'/^\'.__) \
     ,=='  `---`   '---'   '---'      )
     `"""""""""""""""""""""""""""""""`
"#;

fn main() {
    let (width, height) = terminal::size().unwrap();

    let width = (width - 1).clamp(0, 200);
    let height = 30;
    let mut stdout = stdout();

    enable_raw_mode().unwrap();

    queue!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        EnableMouseCapture,
        Clear(ClearType::All)
    ).unwrap();
    stdout.flush().unwrap();

    let snow_flake_sprites = vec!["*", "·", "•"];
    let mut snow_flakes: Vec<Snowflake> = Vec::new();
    for _ in 0..100 {
        snow_flakes.push(Snowflake {
            x: (width as f64 * rand::random::<f64>()).floor(),
            y: (height as f64 * rand::random::<f64>()).floor(),
            speed: (rand::random::<f64>() * 0.5) + 2.8,
            sprite: snow_flake_sprites[(rand::random::<u16>() % snow_flake_sprites.len() as u16) as usize].to_string(),
        });
    }

    let mut phase = 0.0;

    let mut dt;
    let mut current_time = Instant::now();
    let mut mouse_position = (0, 0);

    loop {
        dt = delta_time(&mut current_time);

        queue!(
            stdout,
            MoveTo(0, 0),
            Clear(ClearType::All)
        ).unwrap();

        phase += 1.0 * dt;

        // draw_sine_wave(&mut stdout, width, height, phase);

        draw_santa(&mut stdout, width, height);
        draw_snow_flakes(&mut stdout, width, height, phase, dt, &mut snow_flakes);
        draw_ground(&mut stdout, width, height);
        draw_question(&mut stdout, width, height, mouse_position);

        queue!(
            stdout,
            MoveTo(0, 0),
            style::PrintStyledContent(format!("Mouse: ({}, {})", mouse_position.0, mouse_position.1).stylize())
        ).unwrap();

        queue!(
            stdout,
            MoveTo(0, 1),
            style::PrintStyledContent(format!("FPS: {:.2}", 1.0 / dt).stylize())
        ).unwrap();

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(100));

        if event::poll(Duration::from_millis(0)).unwrap() {
            let raw = read();

            if raw.is_err() {
                println!("Error reading event: {:?}", raw.err());
                break;
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
                    mouse_position = (event.column, event.row);
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

fn draw_ground(stdout: &mut Stdout, width: u16, height: u16) {
    for i in 0..width {
        queue!(
            stdout,
            MoveTo(i, height),
            style::PrintStyledContent("█".white())
        ).unwrap();
    }
}

fn draw_snow_flakes(stdout: &mut Stdout, width: u16, height: u16, phase: f64, dt: f64, snow_flakes: &mut Vec<Snowflake>) {
    for snow_flake in snow_flakes.iter_mut() {
        snow_flake.y += snow_flake.speed * dt;
        snow_flake.x += (phase + snow_flake.x / 10.0).sin() * 0.3;

        let mut x = snow_flake.x as u16;
        let y = snow_flake.y as u16;

        if x > width-1 {
            x = width-1;
        }

        if y >= height {
            snow_flake.y = 0.0;
            snow_flake.x = (width as f64 * rand::random::<f64>()).floor();
        }

        queue!(
            stdout,
            MoveTo(x, y),
            style::PrintStyledContent(snow_flake.sprite.clone().white())
        ).unwrap();
    }
}

fn draw_santa(stdout: &mut Stdout, width: u16, height: u16) {
    let lines = SANTA.lines();
    let offset = height - lines.clone().count() as u16;

    for (i, line) in lines.enumerate() {
        if line == " " {
            continue;
        }

        queue!(
            stdout,
            MoveTo(5, offset + i as u16),
            style::PrintStyledContent(line.white())
        ).unwrap();
    }
}

fn draw_question(stdout: &mut Stdout, width: u16, height: u16, mouse_position: (u16, u16)) {
    let question = "What is the answer to life, the universe, and everything?";
    draw_text_box(stdout, width, height, question, 0, -5, (0, 0));

    draw_text_box(stdout, width, height, "42", -20, 0, mouse_position);
    draw_text_box(stdout, width, height, "24", 0, 0, mouse_position);
    draw_text_box(stdout, width, height, "69", 20, 0, mouse_position);
}

fn draw_text_box(stdout: &mut Stdout, width: u16, height: u16, q: &str, x_offset: i16, y_offset: i16, mouse_position: (u16, u16)) {
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

    // set color if mouse_position is within the text box
    if mouse_position.0 >= x_origin - 3 &&
        mouse_position.0 <= x_origin + question.len() as u16 + 2 &&
        mouse_position.1 >= y_origin - 1 &&
        mouse_position.1 <= y_origin + 1 {
        color = style::Color::Rgb {
            r: 255,
            g: 0,
            b: 0,
        };
    }

    queue!(
        stdout,
        SetForegroundColor(color),
        MoveTo(x_origin - 3, y_origin - 1),
        style::PrintStyledContent(fancy_top_border.stylize())
    ).unwrap();

    for i in 0..1 {
        queue!(
            stdout,
            MoveTo(x_origin - 2 - 1, y_origin + i),
            style::PrintStyledContent("│".stylize())
        ).unwrap();

        queue!(
            stdout,
            MoveTo(x_origin + question.len() as u16 + 2, y_origin + i),
            style::PrintStyledContent("│".stylize())
        ).unwrap();
    }

    queue!(
        stdout,
        MoveTo(x_origin - 3, y_origin + 1),
        style::PrintStyledContent(fancy_bottom_border.stylize())
    ).unwrap();

    queue!(
        stdout,
        MoveTo(x_origin, y_origin),
        style::PrintStyledContent(question.stylize())
    ).unwrap();

    queue!(
        stdout,
        MoveTo(x_origin, y_origin),
        style::PrintStyledContent(question.stylize()),
        SetForegroundColor(style::Color::Reset)
    ).unwrap();
}