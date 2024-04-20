use dyn_clone::DynClone;

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

// trait DamageInstance {
//     fn value(&self) -> u64;
//     fn direction(&self) -> Direction;
// }

// #[derive(Debug, Clone)]
// struct DamageInstance {
//     value: u64,
//     direction: Direction,
// }
//
// impl DamageInstance {
//     fn value(&self) -> u64 {
//         self.value
//     }
//
//     fn set_value(&mut self, new_value: u64) {
//         self.value = new_value;
//     }
// }

type Damage = f64;

trait DamageInstance: DynClone {
    fn value(&self) -> Damage;
    fn add(&self, value: Damage) -> dyn DamageInstance;
    fn multiply_with_probability(&self, multiplier: f64, probability: f64) -> dyn DamageInstance;
}

struct DirectionInstances {
    north: Option<DamageInstance>,
    east: Option<DamageInstance>,
    south: Option<DamageInstance>,
    west: Option<DamageInstance>,
}

impl DirectionInstances {
    fn empty() -> DirectionInstances {
        DirectionInstances {
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }
}

trait Block {
    fn input(&self, inputs: &DirectionInstances) -> DirectionInstances;
}

struct GuidesRight {}

impl Block for GuidesRight {
    fn input(&self, inputs: &DirectionInstances) -> DirectionInstances {
        DirectionInstances {
            north: None,
            east: inputs.north.clone(),
            south: None,
            west: None,
        }
    }
}

struct GuidesLeft {}

impl Block for GuidesLeft {
    fn input(&self, inputs: &DirectionInstances) -> DirectionInstances {
        DirectionInstances {
            north: None,
            east: None,
            south: None,
            west: inputs.north.clone(),
        }
    }
}

struct AddsOne {}

impl Block for AddsOne {
    fn input(&self, inputs: &DirectionInstances) -> DirectionInstances {
        DirectionInstances {
            north: inputs.north.as_ref().map(|dmg| {
                let mut cloned = dmg.clone();
                cloned.set_value(cloned.value() + 1);
                cloned
            }),
            east: None,
            south: None,
            west: None,
        }
    }
}
