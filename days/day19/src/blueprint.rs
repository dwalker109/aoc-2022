#[derive(Debug)]
pub struct Blueprint {
    pub id: usize,
    pub ore_cost: usize,
    pub clay_cost: usize,
    pub obs_cost: [usize; 2],
    pub geode_cost: [usize; 2],
    pub ore_ceil: usize,
    pub clay_ceil: usize,
    pub obs_ceil: usize,
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let mut nums = value
            .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .filter_map(|w| w.parse::<usize>().ok());

        let id = nums.next().unwrap();
        let ore_cost = nums.next().unwrap();
        let clay_cost = nums.next().unwrap();
        let obs_cost = nums.next_chunk().unwrap();
        let geode_cost = nums.next_chunk().unwrap();

        let ore_ceil = *[ore_cost, clay_cost, obs_cost[0], geode_cost[0]]
            .iter()
            .max()
            .unwrap();

        let clay_ceil = obs_cost[1];

        let obs_ceil = geode_cost[1];

        Self {
            id,
            ore_cost,
            clay_cost,
            obs_cost,
            geode_cost,
            // No idea why, but I have to add 2 for ore and clay to get passes for both test and real input ¯\_(ツ)_/¯
            ore_ceil: ore_ceil + 2,
            clay_ceil: clay_ceil + 2,
            obs_ceil,
        }
    }
}
