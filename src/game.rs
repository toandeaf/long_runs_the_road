use crate::plugin::Plugin;
use bevy_ecs::prelude::{Event, Events, IntoScheduleConfigs, Resource, Schedule, World};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::ScheduleSystem;
use macroquad::prelude::get_frame_time;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Game {
    world: World,
    schedules: HashMap<Stage, Schedule>,
}

#[derive(ScheduleLabel, Debug, Clone, Eq, PartialEq, Hash, EnumIter)]
pub enum Stage {
    Startup,
    Main,
    Producers,
    Consumers,
    Renders,
}

#[derive(Resource)]
pub struct DeltaTime(pub f32);

impl Game {
    pub fn new() -> Self {
        Self {
            schedules: schedule_init(),
            world: world_init(),
        }
    }

    pub fn register_event<E: Event>(&mut self) {
        self.add_resource(Events::<E>::default());
    }

    pub fn add_resource<R: Resource>(&mut self, resource: R) {
        self.world.insert_resource(resource);
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: Stage,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) {
        if let Some(sch) = self.schedules.get_mut(&schedule) {
            sch.add_systems(systems);
        } else {
            panic!("Schedule {:?} not found", schedule);
        }
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin) {
        plugin.apply(self);
    }

    pub fn run(&mut self) {
        // Update frame time
        self.add_resource(DeltaTime(get_frame_time()));
        
        // Run all schedules
        for schedule in self.schedules.values_mut() {
            schedule.run(&mut self.world);
        }
    }
}

fn schedule_init() -> HashMap<Stage, Schedule> {
    let mut schedules = HashMap::new();

    for label in Stage::iter() {
        schedules.insert(label, Schedule::default());
    }

    schedules
}

fn world_init() -> World {
    World::new()
}
