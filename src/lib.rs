pub mod insults;
pub mod plugin;
pub mod state;

pub mod prelude {
    pub use crate::{insults::DEFAULT_INSULTS, plugin::InsulterPlugin, state::InsulterState};
}
