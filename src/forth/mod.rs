mod error;
mod native_words;
mod state;

pub use self::error::Error;
pub use self::state::State;

pub type Number = i32;

type ValueStack = Vec<Number>;
