use crate::plugin::Plugin;
use bevy_ecs::prelude::{IntoScheduleConfigs, Resource, Schedule, World};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::ScheduleSystem;
use macroquad::prelude::get_frame_time;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(ScheduleLabel, Debug, Clone, Eq, PartialEq, Hash, EnumIter)]
pub enum GameSchedule {
    Startup,
    Main,
    Producers,
    Consumers,
}

#[derive(Resource)]
struct DeltaTime(f32);

pub struct Game {
    world: World,
    schedules: HashMap<GameSchedule, Schedule>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            schedules: schedule_init(),
            world: world_init(),
        }
    }
    
    pub fn add_resource<R: Resource>(&mut self, resource: R) {
        self.world.insert_resource(resource);
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: GameSchedule,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) {
        if let Some(sch) = self.schedules.get_mut(&schedule) {
            sch.add_systems(systems);
        } else {
            panic!("Schedule {:?} not found", schedule);
        }
    }

    pub fn run(&mut self) {
        self.world.insert_resource(DeltaTime(get_frame_time()));

        for schedule in self.schedules.values_mut() {
            schedule.run(&mut self.world);
        }
    }

    pub fn apply_plugins(&mut self, plugins: &[impl Plugin]) {
        for plugin in plugins {
            plugin.apply(self);
        }
    }
}

fn schedule_init() -> HashMap<GameSchedule, Schedule> {
    let mut schedules = HashMap::new();

    for label in GameSchedule::iter() {
        schedules.insert(label, Schedule::default());
    }

    schedules
}

fn world_init() -> World {
    World::new()
}
