use std::io::{Stdout, Write};
use crossterm::{cursor, queue, style, terminal};
use crossterm::cursor::MoveTo;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::style::{SetForegroundColor, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Cell {
    pub rune: char,
    pub color: style::Color,
}

impl Cell {
    fn new(c: char, color: style::Color) -> Cell {
        Cell {rune: c, color }
    }

    fn clear(&mut self) {
        self.set(' ', style::Color::White);
    }

    fn set(&mut self, c: char, color: style::Color) {
        self.rune = c;
        self.color = color;
    }
}

pub struct Screen {
    stdout: Stdout,
    width: u16,
    height: u16,
    buffer: Vec<Cell>,
}

impl Screen {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Screen {
        let mut screen = Screen {
            stdout,
            width,
            height,
            buffer: Vec::new(),
        };

        screen.resize(width, height);
        screen
    }

    pub fn init(&mut self) {
        enable_raw_mode().unwrap();

        queue!(
            self.stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            EnableMouseCapture,
            Clear(ClearType::All)
        ).unwrap();
    }

    pub fn cleanup(&mut self) {
        disable_raw_mode().unwrap();

        queue!(
            self.stdout,
            cursor::Show,
            terminal::LeaveAlternateScreen,
            DisableMouseCapture
        ).unwrap();

        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i].clear();
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.buffer.clear();
        for _ in 0..(width * height) {
            self.buffer.push(Cell::new(' ', style::Color::White));
        }
        self.clear();
    }

    pub fn set_cell(&mut self, x: u16, y: u16, c: char, color: style::Color) {
        let index = self.xy_to_index(x, y);

        if x >= self.width || y >= self.height {
            panic!("Attempted to set cell outside of screen bounds (x: {}, y: {}, i: {}, len: {})", x, y, index, self.buffer.len());
        }

        self.buffer[index].set(c, color);
    }

    pub fn xy_to_index(&self, x: u16, y: u16) -> usize {
        (y * self.width + x) as usize
    }

    pub fn index_to_xy(&self, index: usize) -> (u16, u16) {
        let x = index as u16 % self.width;
        let y = index as u16 / self.width;
        (x, y)
    }

    pub fn render(&mut self) {
        for (i, cell) in self.buffer.iter().enumerate() {
            let (x, y) = self.index_to_xy(i);
            queue!(
                self.stdout,
                MoveTo(x, y),
                SetForegroundColor(cell.color),
                style::PrintStyledContent(cell.rune.stylize())
            ).unwrap();
        }
        self.stdout.flush().unwrap();
    }
}