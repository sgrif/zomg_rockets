use std::fmt;
use std::marker::PhantomData;

fn main() {
    // Upper stages
    let ablestar = SimpleStage { dry_mass: 667.0, engine: AJ10_104D, };
    let baby_sergeant_1 = SimpleStage { dry_mass: BABY_SERGEANT.mass, engine: BABY_SERGEANT };
    let baby_sergeant_3 = MultiEngine { dry_mass: BABY_SERGEANT.mass * 3.0, engine: BABY_SERGEANT, engine_count: 3 };
    let baby_sergeant_11 = MultiEngine { dry_mass: BABY_SERGEANT.mass * 11.0, engine: BABY_SERGEANT, engine_count: 11 };

    // Lower stages
    let thor = SimpleStage { dry_mass: 3350.0, engine: LR79_NA_9 };
    let atlas_a = BoostedStage {
        core: SimpleStage { dry_mass: 5400.0, engine: LR105_NA_3, },
        booster: SimpleStage { dry_mass: LR89_NA_3.mass + DECOUPLER_MASS, engine: LR89_NA_3, },
        booster_count: 2,
    };







    let mut rocket = Rocket {
        stages: vec![Box::new(atlas_a), Box::new(ablestar), Box::new(baby_sergeant_11), Box::new(baby_sergeant_3), Box::new(baby_sergeant_1)],
        payload_mass: 10.0,
    };
    rocket.set_payload_for_target_deltav(DV_TO_ORBIT);
    println!("Max to orbit: {}", rocket.payload_mass);
    rocket.set_payload_for_target_deltav(DV_TO_GTO * 1.05);
    println!("Max to GTO: {}", rocket.payload_mass);
    rocket.set_payload_for_target_deltav(DV_TO_TLI * 1.05);
    println!("Max to TLI: {}", rocket.payload_mass);

    // rocket.set_payload_for_target_deltav(DV_TO_ORBIT);
    rocket.payload_mass = 10.0;
    println!("{:5}  {:>10}  {:>10}  {:>10}  {:>10}", "stage", "delta-v", "wet mass", "dry mass", "burn time");
    let reversed_stages = rocket.stages().enumerate().collect::<Vec<_>>().into_iter().rev();
    for (i, stage) in reversed_stages {
        println!("{:5}: {:10.0}  {:10.0}  {:10.0}  {:>10}", i, stage.delta_v(), stage.wet_mass(), stage.dry_mass(), BurnTime(stage.burn_time()));
    }
    println!("Total: {:10.0}", rocket.delta_v());
}

const DECOUPLER_MASS: f64 = 52.0;
const GRAVITY: f64 = 9.82;
const DV_TO_ORBIT: f64 = 9400.0;
const DV_TO_GTO: f64 = DV_TO_ORBIT + 2440.0;
const DV_TO_TLI: f64 = DV_TO_GTO + 680.0;
const DV_TO_VENUS: f64 = DV_TO_TLI + 370.0;
const DV_TO_MARS: f64 = DV_TO_TLI + 480.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Fuel {
    name: &'static str,
    density: f64,
}

const KEROSENE: Fuel = Fuel { name: "Kerosene", density: 0.82 };
const LIQUID_OXYGEN: Fuel = Fuel { name: "LqdOxygen", density: 1.141 };
const UDMH: Fuel = Fuel { name: "UDMH", density: 0.791 };
const IRFNA_III: Fuel = Fuel { name: "IRFNA-III", density: 1.658 };
const PSPC: Fuel = Fuel { name: "PSPC", density: 1.74 };

#[derive(Debug, Clone, Copy)]
struct Engine {
    name: &'static str,
    fuel_consumption: &'static [(Fuel, f64)],
    isp: f64,
    thrust: f64,
    mass: f64,
    burn_time: f64,
}

const LR105_NA_3: Engine = Engine {
    name: "LR105-NA-3",
    fuel_consumption: &[(LIQUID_OXYGEN, 70.5326), (KEROSENE, 43.5978)],
    isp: 309.0,
    thrust: 352.2,
    mass: 460.0,
    burn_time: 330.0,
};

const LR105_NA_5: Engine = Engine {
    name: "LR105-NA-5/6",
    fuel_consumption: &[(LIQUID_OXYGEN, 70.5326), (KEROSENE, 43.5978)],
    isp: 311.0,
    thrust: 366.1,
    mass: 413.0,
    burn_time: 350.0,
};

const LR89_NA_3: Engine = Engine {
    name: "LR89-NA-3",
    fuel_consumption: &[(LIQUID_OXYGEN, 166.4868), (KEROSENE, 102.9093)],
    isp: 282.0,
    thrust: 758.7,
    mass: 641.0,
    burn_time: 135.0,
};

const LR89_NA_5: Engine = Engine {
    name: "LR89-NA-5",
    fuel_consumption: &[(LIQUID_OXYGEN, 166.4868), (KEROSENE, 102.9093)],
    isp: 282.0,
    thrust: 758.7,
    mass: 720.0,
    burn_time: 150.0,
};

const LR79_NA_9: Engine = Engine {
    name: "LR79-NA-9",
    fuel_consumption: &[(LIQUID_OXYGEN, 166.2447), (KEROSENE, 107.5894)],
    isp: 284.0,
    thrust: 774.0,
    mass: 934.0,
    burn_time: 165.0,
};

const AJ10_104D: Engine = Engine {
    name: "AJ10-104D",
    fuel_consumption: &[(UDMH, 4.2831), (IRFNA_III, 5.7219)],
    isp: 278.0,
    thrust: 35.1,
    mass: 90.0,
    burn_time: 300.0,
};

const BABY_SERGEANT: Engine = Engine {
    name: "Baby Sergeant",
    fuel_consumption: &[(PSPC, 1.9950)],
    isp: 235.0,
    thrust: 8.0,
    mass: 5.670,
    burn_time: 6.345,
};

impl Engine {
    fn propellant_mass_per_second(&self) -> f64 {
        self.fuel_consumption.iter()
            .map(|&(fuel, rate)| fuel.density * rate)
            .sum()
    }

    fn propellant_mass_for_full_burn(&self) -> f64 {
        self.propellant_mass_per_second() * self.burn_time
    }
}

trait Stage {
    fn isp(&self) -> f64;
    fn dry_mass(&self) -> f64;
    fn wet_mass(&self) -> f64;
    fn burn_time(&self) -> f64;

    /// Simple stages don't need to implement this method. It is used to
    /// calculate delta-v when there are boosters involved. To combine multiple
    /// simple stages, use `Rocket` instead.
    fn next_stage(&self) -> Option<Box<Stage>> {
        None
    }

    fn delta_v(&self) -> f64 {
        self.isp() * (self.wet_mass() / self.dry_mass()).ln() * GRAVITY
    }
}

impl<T: ?Sized + Stage> Stage for Box<T> {
    fn isp(&self) -> f64 { (&**self).isp() }
    fn dry_mass(&self) -> f64 { (&**self).dry_mass() }
    fn wet_mass(&self) -> f64 { (&**self).wet_mass() }
    fn burn_time(&self) -> f64 { (&**self).burn_time() }
    fn next_stage(&self) -> Option<Box<Stage>> { (&**self).next_stage() }
    fn delta_v(&self) -> f64 { (&**self).delta_v() }
}

impl<'a, T: ?Sized + Stage> Stage for &'a T {
    fn isp(&self) -> f64 { (&**self).isp() }
    fn dry_mass(&self) -> f64 { (&**self).dry_mass() }
    fn wet_mass(&self) -> f64 { (&**self).wet_mass() }
    fn burn_time(&self) -> f64 { (&**self).burn_time() }
    fn next_stage(&self) -> Option<Box<Stage>> { (&**self).next_stage() }
    fn delta_v(&self) -> f64 { (&**self).delta_v() }
}

#[derive(Debug, Clone, Copy)]
struct SimpleStage {
    dry_mass: f64,
    engine: Engine,
}

impl SimpleStage {
    fn with_remaining_burn_time(&self, burn_time: f64) -> Self {
        let mut new_stage = self.clone();
        new_stage.engine.burn_time = burn_time;
        new_stage
    }
}

impl Stage for SimpleStage {
    fn isp(&self) -> f64 { self.engine.isp }
    fn dry_mass(&self) -> f64 { self.dry_mass }
    fn burn_time(&self) -> f64 { self.engine.burn_time }

    fn wet_mass(&self) -> f64 {
        self.dry_mass + self.engine.propellant_mass_for_full_burn()
    }
}

struct MultiEngine {
    dry_mass: f64,
    engine: Engine,
    engine_count: usize,
}

impl Stage for MultiEngine {
    fn isp(&self) -> f64 { self.engine.isp }
    fn dry_mass(&self) -> f64 { self.dry_mass }
    fn burn_time(&self) -> f64 { self.engine.burn_time }

    fn wet_mass(&self) -> f64 {
        self.dry_mass + self.engine.propellant_mass_for_full_burn() * self.engine_count as f64
    }
}

#[derive(Debug, Clone, Copy)]
struct BoostedStage {
    core: SimpleStage,
    booster: SimpleStage,
    booster_count: usize,
}

impl BoostedStage {
    fn stage_after_booster_separation(&self) -> SimpleStage {
        self.core.with_remaining_burn_time(self.core.engine.burn_time - self.booster.engine.burn_time)
    }
}

impl Stage for BoostedStage {
    fn isp(&self) -> f64 {
        let core_engine = self.core.engine;
        let booster_engine = self.booster.engine;
        (core_engine.thrust + booster_engine.thrust * self.booster_count as f64) /
            (core_engine.thrust / core_engine.isp + booster_engine.thrust / booster_engine.isp * self.booster_count as f64)
    }

    fn burn_time(&self) -> f64 {
        self.booster.burn_time()
    }

    fn dry_mass(&self) -> f64 {
        self.stage_after_booster_separation().wet_mass() + self.booster.dry_mass() * self.booster_count as f64
    }

    fn wet_mass(&self) -> f64 {
        self.core.wet_mass() + self.booster.wet_mass() * self.booster_count as f64
    }

    fn next_stage(&self) -> Option<Box<Stage>> {
        Some(Box::new(self.stage_after_booster_separation()))
    }
}

struct StageWithPayload<T> {
    stage: T,
    payload_mass: f64,
}

impl<T: Stage> Stage for StageWithPayload<T> {
    fn isp(&self) -> f64 {
        self.stage.isp()
    }

    fn dry_mass(&self) -> f64 {
        self.stage.dry_mass() + self.payload_mass
    }

    fn wet_mass(&self) -> f64 {
        self.stage.wet_mass() + self.payload_mass
    }

    fn burn_time(&self) -> f64 {
        self.stage.burn_time()
    }

    fn next_stage(&self) -> Option<Box<Stage>> {
        self.stage.next_stage().map(|s| Box::new(StageWithPayload {
            stage: s,
            payload_mass: self.payload_mass,
        }) as Box<Stage>)
    }
}

struct Rocket {
    stages: Vec<Box<Stage>>,
    payload_mass: f64,
}

impl Rocket {
    fn stages<'a>(&'a self) -> Box<Iterator<Item=Box<Stage + 'a>> + 'a> {
        let mut iterator = RocketStages {
            current: None,
            remaining: self.stages.iter(),
            payload_mass: self.payload_mass,
            _marker: PhantomData,
        };
        iterator.next();
        Box::new(iterator)
    }

    fn delta_v(&self) -> f64 {
        self.stages().map(|s| s.delta_v()).sum()
    }

    fn set_payload_for_target_deltav(&mut self, target_delta_v: f64) {
        self.payload_mass = 0.0;
        let mut last_mass = 0.0;
        while self.delta_v() > target_delta_v {
            last_mass = self.payload_mass;
            if self.payload_mass < 50.0 {
                self.payload_mass += 10.0;
            } else if self.payload_mass < 500.0 {
                self.payload_mass += 50.0;
            } else {
                self.payload_mass += 100.0;
            }
        }
        self.payload_mass = last_mass;
    }
}

struct RocketStages<'a, T> {
    current: Option<Box<Stage + 'a>>,
    remaining: T,
    payload_mass: f64,
    _marker: PhantomData<&'a ()>
}

impl<'a, T, U> Iterator for RocketStages<'a, U> where
    T: Stage + 'a,
    U: Iterator<Item=T> + Clone,
{
    type Item = Box<Stage + 'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current.take();
        self.current = result.as_ref().and_then(|s| s.next_stage())
            .or_else(|| {
                self.remaining.next().map(|s| {
                    let upper_stage_weight = self.remaining.clone().map(|s| s.wet_mass()).sum::<f64>();
                    Box::new(StageWithPayload {
                        stage: s,
                        payload_mass: upper_stage_weight + self.payload_mass,
                    }) as Box<Stage>
                })
            });
        result
    }
}

struct BurnTime(f64);

impl fmt::Display for BurnTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let minutes = self.0 as u64 / 60;
        let seconds = self.0 as u64 % 60;
        if seconds == 0 {
            format!("{}m", minutes).fmt(fmt)
        } else if minutes == 0 {
            format!("{}s", seconds).fmt(fmt)
        } else {
            format!("{}m {}s", minutes, seconds).fmt(fmt)
        }
    }
}
