use std::{cell::RefCell, ops::Range};

use ::crossterm::style::{Color, ContentStyle, Stylize};
use insults::DEFAULT_INSULTS;
use rand::{thread_rng, Rng};
use shrs::{anyhow::Result, prelude::*};
pub mod insults;
pub mod plugin;
pub mod state;

pub mod prelude {
    pub use crate::{insults, plugin, state};
}
