#![allow(dead_code)]
extern crate ansi_term;

use std::fmt;
use std::marker::PhantomData;
use ansi_term::Colour::{Red, Yellow, Blue};

#[allow(unused_variables, unused_mut)]
fn main() {
    // Probes
    let probe_200kg = SimpleStage { dry_mass: 138.0, engines: vec![THRUSTER.with_burn_time(108.0)] };

    // Upper stages
    let ablestar = SimpleStage { dry_mass: 667.0, engines: vec![AJ10_104D] };
    let agena_a = SimpleStage { dry_mass: 640.0, engines: vec![BELL_8048] };
    let agena_b = SimpleStage { dry_mass: 675.0, engines: vec![BELL_8081] };
    let baby_sergeant_1 = SimpleStage { dry_mass: BABY_SERGEANT.mass, engines: vec![BABY_SERGEANT] };
    let baby_sergeant_3 = SimpleStage { dry_mass: BABY_SERGEANT.mass * 3.0, engines: vec![BABY_SERGEANT; 3] };
    let baby_sergeant_11 = SimpleStage { dry_mass: BABY_SERGEANT.mass * 11.0, engines: vec![BABY_SERGEANT; 11] };

    // Lower stages
    let thor_vernier = LR101_NA_3.with_burn_time(LR79_NA_9.burn_time);
    let thor = SimpleStage { dry_mass: 3350.0, engines: vec![LR79_NA_9, thor_vernier, thor_vernier] };
    let thor_vernier = LR101_NA_3.with_burn_time(LR79_NA_11.burn_time);
    let thor_b = SimpleStage { dry_mass: 3530.0, engines:vec![LR79_NA_11, thor_vernier, thor_vernier] };
    let atlas_vernier = LR101_NA_3.with_burn_time(LR105_NA_3.burn_time);
    let atlas_a = BoostedStage {
        core: SimpleStage { dry_mass: 5400.0, engines: vec![LR105_NA_3, atlas_vernier, atlas_vernier] },
        booster: SimpleStage { dry_mass: LR89_NA_3.mass + DECOUPLER_MASS, engines: vec![LR89_NA_3] },
        booster_count: 2,
    };
    let atlas_vernier = LR101_NA_3.with_burn_time(LR105_NA_5.burn_time);
    let atlas_b = BoostedStage {
        core: SimpleStage { dry_mass: 3920.0, engines: vec![LR105_NA_5, atlas_vernier, atlas_vernier] },
        booster: SimpleStage { dry_mass: LR89_NA_5.mass + DECOUPLER_MASS, engines: vec![LR89_NA_5] },
        booster_count: 2,
    };





    let mut rocket = Rocket {
        stages: vec![Box::new(atlas_b), Box::new(agena_b), Box::new(probe_200kg)],
        payload_mass: 0.0,
    };
    // rocket.set_payload_for_target_deltav(DV_TO_ORBIT);
    // println!("Max to orbit: {}", rocket.payload_mass);
    // rocket.set_payload_for_target_deltav(DV_TO_GTO * 1.05);
    // println!("Max to GTO: {}", rocket.payload_mass);
    // rocket.set_payload_for_target_deltav(DV_TO_GEO * 1.05);
    // println!("Max to GEO: {}", rocket.payload_mass);
    // rocket.set_payload_for_target_deltav(DV_TO_TLI * 1.05);
    // println!("Max to TLI: {}", rocket.payload_mass);

    // rocket.set_payload_for_target_deltav(DV_TO_ORBIT);
    println!("{:5}  {:>10}  {:>10}  {:>10}  {:>10}", "stage", "delta-v", "wet mass", "dry mass", "burn time");
    let reversed_stages = rocket.stages().enumerate().collect::<Vec<_>>().into_iter().rev();
    for (i, stage) in reversed_stages {
        println!("{:5}: {:10.0}  {:10.0}  {:10.0}  {:>10}", i, stage.delta_v(), stage.wet_mass(), stage.dry_mass(), BurnTime(stage.burn_time()));
    }
    println!("Total: {:10.0}", rocket.delta_v());
    println!("Max G Force Endured: {}", rocket.max_g_force());

    print_where_rocket_can_go(&rocket);
}

const DECOUPLER_MASS: f64 = 52.0;
const GRAVITY: f64 = 9.82;
const DV_TO_ORBIT: f64 = 9200.0;
const DV_TO_GTO: f64 = DV_TO_ORBIT + 2440.0;
const DV_TO_GEO: f64 = DV_TO_GTO + 1850.0;
const DV_TO_TLI: f64 = DV_TO_GTO + 680.0;
const DV_TO_LLO: f64 = DV_TO_TLI + 140.0 + 680.0;
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
const HYDRAZINE: Fuel = Fuel { name: "Hydrazine", density: 1.004 };

#[derive(Debug, Clone, Copy)]
struct Engine {
    name: &'static str,
    fuel_consumption: &'static [(Fuel, f64)],
    isp: f64,
    thrust: f64,
    mass: f64,
    burn_time: f64,
}

const BELL_8048: Engine = Engine {
    name: "Bell 8048 (XLR81-BA-5, Agena A)",
    fuel_consumption: &[(UDMH, 8.8115), (IRFNA_III, 10.7262)],
    isp: 276.0,
    thrust: 67.0,
    mass: 132.0,
    burn_time: 120.0,
};

const BELL_8081: Engine = Engine {
    name: "Bell 8081 (XLR81-BA-11, Agena B)",
    fuel_consumption: &[(UDMH, 8.9903), (IRFNA_III, 11.0327)],
    isp: 285.0,
    thrust: 71.0,
    mass: 132.0,
    burn_time: 240.0,
};

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
    fuel_consumption: &[(LIQUID_OXYGEN, 72.8447), (KEROSENE, 45.0270)],
    isp: 311.0,
    thrust: 366.1,
    mass: 413.0,
    burn_time: 350.0,
};

const LR101_NA_3: Engine = Engine {
    name: "LR101-NA-3 Vernier",
    fuel_consumption: &[(LIQUID_OXYGEN, 1.3296), (KEROSENE, 0.8222)],
    isp: 238.0,
    thrust: 4.448,
    mass: 24.0,
    burn_time: 360.0,
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

const LR79_NA_11: Engine = Engine {
    name: "LR79-NA-11",
    fuel_consumption: &[(LIQUID_OXYGEN, 181.1651), (KEROSENE, 117.2455)],
    isp: 286.2,
    thrust: 850.0,
    mass: 980.0,
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

const THRUSTER: Engine = Engine {
    name: "1kN Thruster",
    fuel_consumption:  &[(HYDRAZINE, 0.5643)],
    isp: 198.0,
    thrust: 1.1,
    mass: 15.0,
    burn_time: 20.0 * 60.0,
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

    fn with_burn_time(&self, burn_time: f64) -> Self {
        let mut result = self.clone();
        result.burn_time = burn_time;
        result
    }
}

trait Stage {
    fn engines(&self) -> Vec<Engine>;
    fn dry_mass(&self) -> f64;
    fn wet_mass(&self) -> f64;

    fn burn_time(&self) -> f64 {
        self.engines().iter().map(|e| e.burn_time)
            .max_by_key(|x| *x as u64).unwrap_or(0.0)
    }

    fn isp(&self) -> f64 {
        let engines = self.engines();
        engines.iter().map(|e| e.thrust).sum::<f64>() /
            engines.iter().map(|e| e.thrust / e.isp).sum::<f64>()
    }

    /// Simple stages don't need to implement this method. It is used to
    /// calculate delta-v when there are boosters involved. To combine multiple
    /// simple stages, use `Rocket` instead.
    fn next_stage(&self) -> Option<Box<Stage>> {
        None
    }

    fn delta_v(&self) -> f64 {
        self.isp() * (self.wet_mass() / self.dry_mass()).ln() * GRAVITY
    }

    fn max_g_force(&self) -> f64 {
        self.engines().iter().map(|e| e.thrust * 1000.0).sum::<f64>() /
            self.dry_mass() / GRAVITY
    }
}

impl<T: ?Sized + Stage> Stage for Box<T> {
    fn engines(&self) -> Vec<Engine> { (&**self).engines() }
    fn dry_mass(&self) -> f64 { (&**self).dry_mass() }
    fn wet_mass(&self) -> f64 { (&**self).wet_mass() }
    fn burn_time(&self) -> f64 { (&**self).burn_time() }
    fn next_stage(&self) -> Option<Box<Stage>> { (&**self).next_stage() }
    fn delta_v(&self) -> f64 { (&**self).delta_v() }
}

impl<'a, T: ?Sized + Stage> Stage for &'a T {
    fn engines(&self) -> Vec<Engine> { (&**self).engines() }
    fn dry_mass(&self) -> f64 { (&**self).dry_mass() }
    fn wet_mass(&self) -> f64 { (&**self).wet_mass() }
    fn burn_time(&self) -> f64 { (&**self).burn_time() }
    fn next_stage(&self) -> Option<Box<Stage>> { (&**self).next_stage() }
    fn delta_v(&self) -> f64 { (&**self).delta_v() }
}

#[derive(Debug, Clone)]
struct SimpleStage {
    dry_mass: f64,
    engines: Vec<Engine>,
}

impl SimpleStage {
    fn with_remaining_burn_time(&self, burn_time: f64) -> Self {
        let mut new_stage = self.clone();
        for engine in &mut new_stage.engines {
            engine.burn_time = burn_time;
        }
        new_stage
    }
}

impl Stage for SimpleStage {
    fn engines(&self) -> Vec<Engine> { self.engines.clone() }
    fn dry_mass(&self) -> f64 { self.dry_mass }

    fn wet_mass(&self) -> f64 {
        self.dry_mass + self.engines.iter().map(|e| e.propellant_mass_for_full_burn()).sum::<f64>()
    }
}

#[derive(Debug, Clone)]
struct BoostedStage {
    core: SimpleStage,
    booster: SimpleStage,
    booster_count: usize,
}

impl BoostedStage {
    fn stage_after_booster_separation(&self) -> SimpleStage {
        self.core.with_remaining_burn_time(self.core.burn_time() - self.booster.burn_time())
    }
}

impl Stage for BoostedStage {
    fn engines(&self) -> Vec<Engine> {
        let mut engines = self.core.engines();
        let booster_engines = self.booster.engines();
        for _ in 0..self.booster_count {
            engines.extend_from_slice(&booster_engines);
        }
        engines
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
    fn engines(&self) -> Vec<Engine> {
        self.stage.engines()
    }

    fn dry_mass(&self) -> f64 {
        self.stage.dry_mass() + self.payload_mass
    }

    fn wet_mass(&self) -> f64 {
        self.stage.wet_mass() + self.payload_mass
    }

    fn next_stage(&self) -> Option<Box<Stage>> {
        self.stage.next_stage().map(|s| Box::new(StageWithPayload {
            stage: s,
            payload_mass: self.payload_mass,
        }) as Box<Stage>)
    }

    fn burn_time(&self) -> f64 {
        self.stage.burn_time()
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

    fn max_g_force(&self) -> f64 {
        let mut g_forces = self.stages().map(|s| s.max_g_force()).collect::<Vec<_>>();
        g_forces.sort_by(|a, b| b.partial_cmp(a).expect("We should never get NaN here"));
        *g_forces.first().unwrap_or(&0.0)
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

fn print_where_rocket_can_go(rocket: &Rocket) {
    let dv = rocket.delta_v();
    if dv <= DV_TO_ORBIT {
        println!("{}", Red.bold().paint("This rocket will not reach orbit"));
    }

    print_if_rocket_can_go_to(dv, DV_TO_GTO, "GTO");
    print_if_rocket_can_go_to(dv, DV_TO_GEO, "GEO");
    print_if_rocket_can_go_to(dv, DV_TO_TLI, "The Moon");
    print_if_rocket_can_go_to(dv, DV_TO_LLO, "Lunar Orbit");
    print_if_rocket_can_go_to(dv, DV_TO_VENUS, "Venus");
    print_if_rocket_can_go_to(dv, DV_TO_MARS, "Mars");
}

fn print_if_rocket_can_go_to(dv: f64, required_dv: f64, name: &str) {
    if dv > required_dv * 1.05 {
        println!("{}", Blue.paint(format!("This rocket can go to {}", name)));
    } else if dv > required_dv {
        println!("{}", Yellow.paint(format!("This rocket can go to {} without safety margins", name)));
    }
}
