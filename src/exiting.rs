use crate::game::{Game, GameSchedule};
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use macroquad::input::{is_key_pressed, KeyCode};
use std::process::exit;

pub struct ExitPlugin;

#[derive(Event)]
struct ExitEvent;

fn exit_evaluator(mut writer: EventWriter<ExitEvent>) {
    if is_key_pressed(KeyCode::Escape) {
        writer.write(ExitEvent);
    }
}

fn exit_consumer(mut reader: EventReader<ExitEvent>) {
    if let Some(_) = reader.read().next() {
        println!("Exit event received, exiting...");
        exit(0);
    }
}

impl Plugin for ExitPlugin {
    fn apply(&self,  game: &mut Game) {
        game.add_resource(Events::<ExitEvent>::default());

        game.add_systems(GameSchedule::Producers, exit_evaluator);
        game.add_systems(GameSchedule::Consumers, exit_consumer);
    }
}
