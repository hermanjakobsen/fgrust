// Unused code from original repo
// Can be removed, but want to keep it around just in case and as an example
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

fn draw_sine_wave(screen_buffer: &mut Vec<Cell>, width: u16, height: u16, phase: f64) {
    for i in 0..width {
        let x = i as f64;
        let y = (height as f64 / 2.0) + (phase + x / 10.0).sin() * 10.0;
        let clamped_y = y as u16 % height;

        let index = (clamped_y * width + x as u16) as usize;
        screen_buffer[index].c = '█';
    }
}
