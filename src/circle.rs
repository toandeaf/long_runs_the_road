use crate::game::{DeltaTime, Game, Stage};
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use macroquad::color::RED;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::shapes::draw_circle;

pub struct CirclePlugin;

#[derive(Event)]
struct CreateCircleEvent;

#[derive(Event)]
struct DestroyCircleEvent;

#[derive(Event)]
struct GravityEvent(f32);

#[derive(Component)]
struct CirclePosition {
    x: f32,
    y: f32,
}

const GRAVITY: f32 = 40.;

impl Plugin for CirclePlugin {
    fn apply(&self, game: &mut Game) {
        game.register_event::<CreateCircleEvent>();
        game.register_event::<DestroyCircleEvent>();
        game.register_event::<GravityEvent>();

        game.add_systems(Stage::Producers, (circle_creator, circle_destroyer, gravity_emitter));
        game.add_systems(Stage::Consumers, (circle_create_consumer, circle_destroy_consumer, circle_gravity_applier));
        game.add_systems(Stage::Renders, circle_renderer);
    }
}

fn circle_creator(mut writer: EventWriter<CreateCircleEvent>) {
    if is_key_pressed(KeyCode::E) {
        writer.write(CreateCircleEvent);
    }
}

fn circle_create_consumer(mut commands: Commands, mut reader: EventReader<CreateCircleEvent>) {
    if reader.read().next().is_some() {
        commands.spawn((CirclePosition { x: 300., y: 350. },));
    }
}

fn circle_destroyer(mut writer: EventWriter<DestroyCircleEvent>) {
    if is_key_pressed(KeyCode::R) {
        writer.write(DestroyCircleEvent);
    }
}

fn circle_destroy_consumer(mut commands: Commands, mut reader: EventReader<DestroyCircleEvent>, query: Query<Entity, With<CirclePosition>>) {
    if reader.read().next().is_some() {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn gravity_emitter(mut writer: EventWriter<GravityEvent>) {
    // TODO - rewrite this to use timer deltas so i dont just dump a tonne of these
    writer.write(GravityEvent(GRAVITY));
}

fn circle_gravity_applier(mut query: Query<&mut CirclePosition>, mut reader: EventReader<GravityEvent>, time: Res<DeltaTime>) {
    for event in reader.read() {
        for mut circle in query.iter_mut() {
            circle.y -= event.0 * time.0;
        }
    }
}

fn circle_renderer(query: Query<&CirclePosition>) {
    for circle in query.iter() {
        draw_circle(circle.x, circle.y, 10., RED);
    }
}
