extern crate crossterm;
use crossterm::style;

pub struct Cell {
    pub x: u16,
    pub y: u16,
    pub c: char,
    pub color: style::Color,
}

impl Cell {
    fn new(x: u16, y: u16, c: char, color: style::Color) -> Cell {
        Cell { x, y, c, color }
    }
}

pub struct Snowflake {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub sprite: char,
}

pub fn clamp_screen((width, height): (u16, u16)) -> (u16, u16) {
    let new_width = (width - 1).clamp(80, 200);
    let new_height = height.clamp(40, 70);
    (new_width, new_height)
}

pub fn reset_screen_buffer(screen_buffer: &mut Vec<Cell>, width: u16, height: u16) {
    screen_buffer.clear();
    for i in 0..height {
        for j in 0..width {
            screen_buffer.push(Cell::new(
                j,
                i,
                ' ',
                style::Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            ));
        }
    }
}

pub fn draw_debug_info(
    screen_buffer: &mut Vec<Cell>,
    width: u16,
    _height: u16,
    mouse_position: (u16, u16),
    mouse_down: bool,
    dt: f64,
) {
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

pub fn draw_ground(screen_buffer: &mut Vec<Cell>, width: u16, height: u16) {
    let ground = width * (height - 1);

    for i in 0..width {
        screen_buffer[(ground + i) as usize].c = '█';
    }
}

pub fn draw_snow_flakes(
    screen_buffer: &mut Vec<Cell>,
    width: u16,
    height: u16,
    phase: f64,
    dt: f64,
    snow_flakes: &mut Vec<Snowflake>,
) {
    for snow_flake in snow_flakes.iter_mut() {
        snow_flake.y += snow_flake.speed * dt;
        snow_flake.x += (phase + snow_flake.x / 10.0).sin() * 0.02;

        let x = (snow_flake.x as u16).clamp(0, width - 1);
        let y = snow_flake.y as u16;

        if y >= height - 1 {
            snow_flake.y = 0.0;
            snow_flake.x = (width - 1) as f64 * rand::random::<f64>();
        }

        let index = ((y * width) + x) as usize;
        screen_buffer[index].c = snow_flake.sprite;
    }
}

pub fn draw_ascii(
    screen_buffer: &mut Vec<Cell>,
    ascii: &str,
    width: u16,
    _height: u16,
    x: u16,
    y: u16,
) {
    let lines = ascii.lines();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            let index = ((y + i as u16) * width + x + j as u16) as usize;
            if index < screen_buffer.len() {
                screen_buffer[index].c = c;
            }
        }
    }
}

pub fn draw_question(
    screen_buffer: &mut Vec<Cell>,
    width: u16,
    height: u16,
    mouse_position: (u16, u16),
    mouse_down: bool,
    question: &str,
    correct_answer: &str,
    wrong_answers: &[&str],
    correct_answer_position: usize,
) {
    {
        draw_text_box(screen_buffer, width, height, question, 0, -5, (0, 0), false);
        let total_answers = wrong_answers.len() + 1;

        let mut incorrect_is_hovered: Vec<bool> = vec![];
        let mut correct_answer_hovered: bool = false;

        let delta_offset: i16 = 20;
        let minimum_offset: i16 = (((total_answers - 1) as f32 / 2.0) * -delta_offset as f32) as i16;

        let mut wrong_answer_iterator: usize = 0;

        for i in 0..total_answers {
            let x_offset = minimum_offset + delta_offset * i as i16;
            if i == correct_answer_position {
                correct_answer_hovered = draw_text_box(
                    screen_buffer,
                    width,
                    height,
                    correct_answer,
                    x_offset,
                    0,
                    mouse_position,
                    mouse_down,
                );
            } else {
                incorrect_is_hovered.push(draw_text_box(
                    screen_buffer,
                    width,
                    height,
                    wrong_answers[wrong_answer_iterator],
                    x_offset,
                    0,
                    mouse_position,
                    mouse_down,
                ));
                wrong_answer_iterator += 1;
            }
        }

        if mouse_down {
            let any_incorrect_is_hovered = incorrect_is_hovered.into_iter().any(|x| x);
            if correct_answer_hovered && !any_incorrect_is_hovered {
                draw_text_box(
                    screen_buffer,
                    width,
                    height,
                    "Correct!",
                    0,
                    5,
                    (0, 0),
                    false,
                );
            } else if any_incorrect_is_hovered {
                draw_text_box(screen_buffer, width, height, "Wrong!", 0, 5, (0, 0), false);
            }
        }
    }

    fn draw_text_box(
        screen_buffer: &mut Vec<Cell>,
        width: u16,
        height: u16,
        q: &str,
        x_offset: i16,
        y_offset: i16,
        mouse_position: (u16, u16),
        mouse_down: bool,
    ) -> bool {
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
        if mouse_position.0 >= x_origin - 3
            && mouse_position.0 <= x_origin + question.len() as u16 + 2
            && mouse_position.1 >= y_origin - 1
            && mouse_position.1 <= y_origin + 1
        {
            if mouse_down {
                color = style::Color::Rgb { r: 0, g: 255, b: 0 };
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
