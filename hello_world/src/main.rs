extern crate crossterm;

use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, cursor::MoveTo, execute, style, terminal, terminal::{Clear, ClearType}, QueueableCommand};
use std::io::{stdout, Write};

fn main() {
    let mut system = ParticleSystem::new();
    system.add_particle(Particle {
        x: 0.0,
        y: 0.0,
        speed: 1.0,
        direction: 0.0,
    });

    // enter alt screen
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

        system.update(dt);
        // system.render();

        stdout.queue(MoveTo(0, 0)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        //
        // for i in 0..height {
        //     for j in 0..width {
        //         stdout.queue(MoveTo(j, i)).unwrap();
        //         stdout.queue(style::PrintStyledContent("·".grey())).unwrap();
        //     }
        // }

        // update x and y, wrapping it around the screen
        // use delta to make the movement framerate-independent
        // move in a sine wave pattern
        phase += 1.0 * dt;
        x = (width as f64 / 2.0) + (-(phase * 2.0).sin() * (width as f64 / 2.1));
        y = (height as f64 / 2.0) + ((phase * 2.0).cos() * (height as f64 / 2.1));

        // stdout.queue(MoveTo(clamped_x, clamped_y)).unwrap();
        // stdout.queue(style::PrintStyledContent(x.to_string().magenta())).unwrap();

        // move cursor to x, y
        stdout.queue(MoveTo(x as u16 % width, y as u16 % height)).unwrap();
        stdout.queue(style::PrintStyledContent("◯".green())).unwrap();

        // render a sine wave pattern across "width" number of columns
        for i in 0..width {
            let x = i as f64;
            let y = (height as f64 / 2.0) + (phase + x / 10.0).sin() * 10.0;
            let clamped_y = y as u16 % height;
            stdout.queue(MoveTo(x as u16, clamped_y)).unwrap();
            stdout.queue(style::PrintStyledContent("█".magenta())).unwrap();
        }

        // move cursor to 0, 0
        stdout.queue(MoveTo(0, 0)).unwrap();

        // flush
        stdout.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));

        // Exit the program when the user presses the 'q' key
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

struct Particle {
    x: f64,
    y: f64,
    speed: f64,
    direction: f64,
}

struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new() -> Self {
        ParticleSystem {
            particles: Vec::new(),
        }
    }

    fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    fn update(&mut self, dt: f64) {
        for particle in &mut self.particles {
            particle.x += particle.direction.cos() * particle.speed * dt;
            particle.y += particle.direction.sin() * particle.speed * dt;
        }
    }

    fn render(&self) {
        let mut stdout = stdout();

        // Clear the screen
        execute!(stdout, Clear(ClearType::All)).unwrap();

        // Define the size of the console grid
        let width = 80;
        let height = 24;

        // Create a 2D grid filled with spaces
        let mut grid = vec![vec![' '; width]; height];

        // Place particles on the grid
        for particle in &self.particles {
            let x = particle.x as usize % width;
            let y = particle.y as usize % height;
            grid[y][x] = '*';
        }

        // Print the grid to the console
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '*' {
                    execute!(stdout, MoveTo(x as u16, y as u16)).unwrap();
                    print!("{}", cell);
                }
            }
        }

        stdout.flush().unwrap();
    }
}