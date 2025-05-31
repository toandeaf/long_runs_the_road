use crate::game::{Game, Stage};
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use macroquad::input::{is_key_pressed, KeyCode};
use std::process::exit;

pub struct ExitPlugin;

#[derive(Event)]
struct ExitEvent;

impl Plugin for ExitPlugin {
    fn apply(&self, game: &mut Game) {
        game.register_event::<ExitEvent>();

        game.add_systems(Stage::Producers, exit_evaluator);
        game.add_systems(Stage::Consumers, exit_consumer);
    }
}

fn exit_evaluator(mut writer: EventWriter<ExitEvent>) {
    if is_key_pressed(KeyCode::Escape) {
        writer.write(ExitEvent);
    }
}

fn exit_consumer(mut reader: EventReader<ExitEvent>) {
    if reader.read().next().is_some() {
        println!("Exit event received, exiting...");
        exit(0);
    }
}
