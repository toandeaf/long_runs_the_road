use crate::game::{Game, GameSchedule};
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use macroquad::color::RED;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::shapes::draw_circle;

pub struct CirclePlugin;

#[derive(Event)]
struct CircleEvent;

impl Plugin for CirclePlugin {
    fn apply(&self, game: &mut Game) {
        game.register_event::<CircleEvent>();

        game.add_systems(GameSchedule::Producers, circle_evaluator);
        game.add_systems(GameSchedule::Consumers, circle_consumer);
    }
}

fn circle_evaluator(mut writer: EventWriter<CircleEvent>) {
    if is_key_down(KeyCode::E) {
        writer.write(CircleEvent);
    }
}

fn circle_consumer(mut reader: EventReader<CircleEvent>) {
    if reader.read().next().is_some() {
        draw_circle(300., 300., 15., RED);
    }
}
