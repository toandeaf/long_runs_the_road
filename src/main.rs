mod circle;
mod exiting;
mod game;
mod player;
mod plugin;

use crate::exiting::ExitPlugin;
use crate::game::Game;
use bevy_ecs::prelude::Resource;

use crate::circle::CirclePlugin;
use crate::player::{add_player_sprite, PlayerPlugin};
use macroquad::prelude::*;
use macroquad::Window;

fn main() {
    let mut game = Game::new();
    
    game.add_plugin(ExitPlugin);
    game.add_plugin(CirclePlugin);
    game.add_plugin(PlayerPlugin);

    let screen_conf = Conf {
        fullscreen: true,
        window_title: "LRTR".to_string(),
        ..Default::default()
    };

    Window::from_config(screen_conf, game_loop(game));
}

async fn game_loop(mut game: Game) {
    add_player_sprite(&mut game).await;

    loop {
        clear_background(WHITE);
        game.run();
        next_frame().await;
    }
}
