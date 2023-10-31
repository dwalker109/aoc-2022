use std::cmp::{max};

use crate::blueprint::Blueprint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct State {
    pub remaining: usize,
    pub robots: Bom,
    pub materials: Bom,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Bom {
    pub ore: usize,
    pub clay: usize,
    pub obs: usize,
    pub geode: usize,
}

impl State {
    pub fn new(mins: usize) -> Self {
        Self {
            remaining: mins,
            robots: Bom {
                ore: 1,
                ..Default::default()
            },
            materials: Default::default(),
        }
    }

    fn tick(&mut self, mins: usize) {
        let mins = match mins.cmp(&self.remaining) {
            std::cmp::Ordering::Greater => self.remaining,
            _ => mins,
        };

        self.remaining -= mins;
        self.materials.ore += self.robots.ore * mins;
        self.materials.clay += self.robots.clay * mins;
        self.materials.obs += self.robots.obs * mins;
        self.materials.geode += self.robots.geode * mins;
    }

    fn build_ore_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.robots.ore == 0 || self.materials.ore >= blueprint.ore_ceil {
            return None;
        }

        let mut next = *self;

        let req = blueprint.ore_cost.saturating_sub(next.materials.ore);
        let act = req.div_ceil(next.robots.ore);

        next.tick(act);

        if next.materials.ore >= blueprint.ore_cost {
            next.materials.ore -= blueprint.ore_cost;
            next.tick(1);
            next.robots.ore += 1;
        }

        Some(next)
    }

    fn build_clay_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.robots.ore == 0 || self.materials.clay >= blueprint.clay_ceil {
            return None;
        }

        let mut next = *self;

        let req = blueprint.clay_cost.saturating_sub(next.materials.ore);
        let act = req.div_ceil(next.robots.ore);

        next.tick(act);

        if next.materials.ore >= blueprint.clay_cost {
            next.materials.ore -= blueprint.clay_cost;
            next.tick(1);
            next.robots.clay += 1;
        }

        Some(next)
    }

    fn build_obs_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.robots.ore == 0 || self.robots.clay == 0 || self.materials.obs >= blueprint.obs_ceil
        {
            return None;
        }

        let mut next = *self;

        let req_ore = blueprint.obs_cost[0].saturating_sub(next.materials.ore);
        let act_ore = req_ore.div_ceil(next.robots.ore);

        let req_clay = blueprint.obs_cost[1].saturating_sub(next.materials.clay);
        let act_clay = req_clay.div_ceil(next.robots.clay);

        next.tick(max(act_ore, act_clay));

        if next.materials.ore >= blueprint.obs_cost[0]
            && next.materials.clay >= blueprint.obs_cost[1]
        {
            next.materials.ore -= blueprint.obs_cost[0];
            next.materials.clay -= blueprint.obs_cost[1];
            next.tick(1);
            next.robots.obs += 1;
        }

        Some(next)
    }

    fn build_geode_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.robots.ore == 0 || self.robots.obs == 0 {
            return None;
        }

        let mut next = *self;

        let req_ore = blueprint.geode_cost[0].saturating_sub(next.materials.ore);
        let act_ore = req_ore.div_ceil(next.robots.ore);

        let req_obs = blueprint.geode_cost[1].saturating_sub(next.materials.obs);
        let act_obs = req_obs.div_ceil(next.robots.obs);

        next.tick(max(act_ore, act_obs));

        if next.materials.ore >= blueprint.geode_cost[0]
            && next.materials.obs >= blueprint.geode_cost[1]
        {
            next.materials.ore -= blueprint.geode_cost[0];
            next.materials.obs -= blueprint.geode_cost[1];
            next.tick(1);
            next.robots.geode += 1;
        }

        Some(next)
    }

    pub fn next(self, blueprint: &Blueprint) -> impl Iterator<Item = State> {
        [
            self.build_ore_robot(blueprint),
            self.build_clay_robot(blueprint),
            self.build_obs_robot(blueprint),
            self.build_geode_robot(blueprint),
        ]
        .into_iter()
        .flatten()
    }
}
