use rand::{thread_rng, Rng};

use crate::{drawing::draw_question, screen::Screen};

pub trait CalendarDay {
    // Return `true` if task succeeded, false if not
    fn tick(
        &self,
        screen: &mut Screen,
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
    let number_of_answers = wrong_answers.len() + 1;
    QuizDay {
        question,
        correct_answer,
        wrong_answers,
        correct_answer_position: thread_rng().gen_range(0..number_of_answers),
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
        screen: &mut Screen,
        mouse_position: (u16, u16),
        mouse_down: bool,
    ) -> RunStatus {
        let mut status = RunStatus::RUNNING;
        draw_question(
            screen,
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
