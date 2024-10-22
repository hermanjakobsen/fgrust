use rand::{thread_rng, Rng};

use crate::drawing::{draw_question, Cell};

pub trait CalendarDay {
    // Return `true` if task succeeded, false if not
    fn tick(
        &self,
        screen_buffer: &mut Vec<Cell>,
        width: u16,
        height: u16,
        mouse_position: (u16, u16),
        mouse_down: bool,
    ) -> RunStatus;
}

#[derive(PartialEq, Eq, Debug)]
pub enum RunStatus {
    RUNNING,
    CORRECT,
    READY,
}

pub fn create_quiz_day<'a>(
    question: &'a str,
    correct_answer: &'a str,
    wrong_answers: &'a [&'a str],
) -> QuizDay<'a> {
    QuizDay {
        question,
        correct_answer,
        wrong_answers,
        correct_answer_position: thread_rng().gen_range(0..3),
    }
}

pub struct QuizDay<'a> {
    question: &'a str,
    correct_answer: &'a str,
    wrong_answers: &'a [&'a str],

    correct_answer_position: usize,
}

impl CalendarDay for QuizDay<'_> {
    fn tick(
        &self,
        screen_buffer: &mut Vec<Cell>,
        width: u16,
        height: u16,
        mouse_position: (u16, u16),
        mouse_down: bool,
    ) -> RunStatus {
        let mut status = RunStatus::RUNNING;
        draw_question(
            screen_buffer,
            width,
            height,
            mouse_position,
            mouse_down,
            self.question,
            self.correct_answer,
            self.wrong_answers,
            self.correct_answer_position,
            &mut || status = RunStatus::CORRECT,
        );
        status
    }
}
