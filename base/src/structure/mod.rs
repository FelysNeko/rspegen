mod atom;
mod gram;
mod item;
mod alter;
mod rule;

pub use atom::Atom;
pub use gram::Grammar;
pub use item::Item;
pub use alter::Alter;
pub use rule::Rule;


pub trait Generate {
    fn generate(&self) -> String;
}
