use crate::game::{DeltaTime, Game, Stage};
use crate::plugin::Plugin;
use bevy_ecs::event::{Event, EventReader, EventWriter};
use bevy_ecs::prelude::{Res, ResMut, Resource};
use input::KeyCode;
use macroquad::color::WHITE;
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::input;
use macroquad::input::is_key_down;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn apply(&self, game: &mut Game) {
        game.register_event::<MovementEvent>();

        game.add_systems(Stage::Producers, movement_eval);
        game.add_systems(Stage::Consumers, movement_applier);
        game.add_systems(Stage::Renders, render_player);
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

fn movement_eval(mut writer: EventWriter<MovementEvent>) {
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
}

fn movement_applier(
    mut player_sprite: ResMut<PlayerSprite>,
    mut reader: EventReader<MovementEvent>,
    time: Res<DeltaTime>
) {
    println!("delta time is {}", time.0);
    for event in reader.read() {
        player_sprite.sprite.playing = true;
        
        match event.direction {
            Direction::Up => {
                player_sprite.sprite.set_animation(0);
                player_sprite.transform.y -= PLAYER_SPEED * time.0; 
            },
            Direction::Down => {
                player_sprite.sprite.set_animation(1);
                player_sprite.transform.y += PLAYER_SPEED * time.0;
            },
            Direction::Left => {
                player_sprite.sprite.set_animation(2);
                player_sprite.transform.x -= PLAYER_SPEED * time.0;
            },
            Direction::Right => {
                player_sprite.sprite.set_animation(3);
                player_sprite.transform.x += PLAYER_SPEED * time.0;
            },
            Direction::None => {
                player_sprite.sprite.playing = false;
                player_sprite.sprite.set_frame(0);
            }
        }
    }
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
    // Update frame
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
    let start_location = Vec2::new(300., 300.);
    game.add_resource(PlayerSprite {
        transform: start_location,
        sprite,
        texture: image,
    });
}
