use crate::game::Game;

pub trait Plugin {
    fn apply(&self, game: &mut Game);
}