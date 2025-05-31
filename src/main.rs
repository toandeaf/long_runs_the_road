mod exiting;
mod plugin;
mod game;
mod circle;

use crate::exiting::ExitPlugin;
use crate::game::Game;

use crate::circle::CirclePlugin;
use macroquad::prelude::*;
use macroquad::Window;

fn main() {
    let mut game = Game::new();
    game.add_plugin(ExitPlugin);
    game.add_plugin(CirclePlugin);

    let screen_conf = Conf {
        fullscreen: true,
        window_title: "Macroquad Example".to_string(),
        ..Default::default()
    };

    Window::from_config(screen_conf, game_loop(game));
}

async fn game_loop(mut game: Game) {

    let camera = Camera2D::from_display_rect(Rect::new(200.0, 200.0, 200.0, 200.0));

    loop {
        set_camera(&camera);
        clear_background(WHITE);
        game.run();
        next_frame().await;
    }
}
