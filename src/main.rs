mod exiting;
mod plugin;
mod game;

use crate::exiting::ExitPlugin;
use crate::game::Game;

use macroquad::prelude::*;
use macroquad::Window;

fn main() {
    let mut game = Game::new();
    game.apply_plugins(&[ExitPlugin]);

    let screen_conf = Conf {
        fullscreen: true,
        window_title: "Macroquad Example".to_string(),
        ..Default::default()
    };

    Window::from_config(screen_conf, game_loop(game));
}

async fn game_loop(mut game: Game) {
    loop {
        // clear_background(BLACK);
        game.run();
        next_frame().await;
    }
}
