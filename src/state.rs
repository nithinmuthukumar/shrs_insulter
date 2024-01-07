use std::cell::RefCell;

use crossterm::style::ContentStyle;
use rand::{thread_rng, Rng};

use crate::insults::DEFAULT_INSULTS;

pub struct InsulterState {
    pub insults: RefCell<Vec<String>>,
    pub freq: f32,
    pub style: ContentStyle,
}

impl InsulterState {
    pub fn new(
        insults: Vec<String>,
        freq: f32,
        include_default: bool,
        style: ContentStyle,
    ) -> Self {
        let mut insults_c = insults;
        if include_default {
            insults_c.append(&mut DEFAULT_INSULTS.iter().map(|i| i.to_string()).collect());
        }
        if insults_c.is_empty() {
            panic!("NO INSULTS U DUMMY")
        }
        Self {
            insults: RefCell::new(insults_c.iter().map(|i| i.to_string()).collect()),
            freq,
            style,
        }
    }

    pub(crate) fn should_insult(&self) -> bool {
        thread_rng().gen::<f32>() <= self.freq
    }
    pub(crate) fn rand_insult(&self) -> String {
        let i = self.insults.borrow_mut();

        i[thread_rng().gen_range(0..i.len())].clone()
    }
}
