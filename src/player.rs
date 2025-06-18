use crate::game::{DeltaTime, Game, Stage};
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use macroquad::prelude::animation::*;
use macroquad::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn apply(&self, game: &mut Game) {
        game.register_event::<MovementEvent>();

        game.add_systems(Stage::Producers, movement_input);
        game.add_systems(Stage::Consumers, movement_applier);
        game.add_systems(Stage::Renders, (update_camera_target, render_player));
    }
}

#[derive(Resource)]
struct PlayerSprite {
    sprite: AnimatedSprite,
    texture: Texture2D,
    transform: Vec2,
}

#[derive(Event)]
struct MovementEvent {
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

const PLAYER_SPEED: f32 = 100.0;

fn movement_input(mut writer: EventWriter<MovementEvent>) {
    if is_key_down(KeyCode::W) {
        writer.write(MovementEvent {
            direction: Direction::Up,
        });
    } else if is_key_down(KeyCode::S) {
        writer.write(MovementEvent {
            direction: Direction::Down,
        });
    } else if is_key_down(KeyCode::A) {
        writer.write(MovementEvent {
            direction: Direction::Left,
        });
    } else if is_key_down(KeyCode::D) {
        writer.write(MovementEvent {
            direction: Direction::Right,
        });
    } else {
        writer.write(MovementEvent {
            direction: Direction::None, // Default to down if no key is pressed
        });
    }

    if is_key_released(KeyCode::A) || is_key_released(KeyCode::D) || is_key_released(KeyCode::W) || is_key_released(KeyCode::S) {
        writer.write(MovementEvent {
            direction: Direction::None, // Reset to None when any key is released
        });
    }
}

fn movement_applier(
    mut player_sprite: ResMut<PlayerSprite>,
    mut reader: EventReader<MovementEvent>,
    time: Res<DeltaTime>,
) {
    for event in reader.read() {
        player_sprite.sprite.playing = true;

        match event.direction {
            Direction::Up => {
                player_sprite.sprite.set_animation(0);
                player_sprite.transform.y -= PLAYER_SPEED * time.0;
            }
            Direction::Down => {
                player_sprite.sprite.set_animation(1);
                player_sprite.transform.y += PLAYER_SPEED * time.0;
            }
            Direction::Left => {
                player_sprite.sprite.set_animation(2);
                player_sprite.transform.x -= PLAYER_SPEED * time.0;
            }
            Direction::Right => {
                player_sprite.sprite.set_animation(3);
                player_sprite.transform.x += PLAYER_SPEED * time.0;
            }
            Direction::None => {
                player_sprite.sprite.playing = false;
                player_sprite.sprite.set_frame(0);
            }
        }
    }
}

fn update_camera_target(player_sprite: Res<PlayerSprite>) {
    set_camera(&Camera2D {
        target: vec2(player_sprite.transform.x, player_sprite.transform.y),
        zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
        ..Default::default()
    })
}

fn render_player(mut player_sprite: ResMut<PlayerSprite>) {
    draw_texture_ex(
        &player_sprite.texture,
        player_sprite.transform.x,
        player_sprite.transform.y,
        WHITE,
        DrawTextureParams {
            source: Some(player_sprite.sprite.frame().source_rect),
            dest_size: Some(player_sprite.sprite.frame().dest_size),
            ..Default::default()
        },
    );
    player_sprite.sprite.update();
}

pub async fn add_player_sprite(game: &mut Game) {
    let sprite = AnimatedSprite::new(
        60,
        60,
        &[
            Animation {
                name: "up".to_string(),
                row: 0,
                frames: 9,
                fps: 20,
            },
            Animation {
                name: "down".to_string(),
                row: 1,
                frames: 9,
                fps: 20,
            },
            Animation {
                name: "left".to_string(),
                row: 2,
                frames: 9,
                fps: 20,
            },
            Animation {
                name: "right".to_string(),
                row: 3,
                frames: 9,
                fps: 20,
            },
        ],
        true,
    );

    let image = load_texture("assets/walk.png").await.unwrap();
    let start_location = Vec2::new(screen_width() / 2., screen_height() / 2.);

    game.add_resource(PlayerSprite {
        transform: start_location,
        sprite,
        texture: image,
    });
}
