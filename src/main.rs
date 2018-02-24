#![allow(dead_code)]
extern crate ansi_term;

mod engines;
mod fuels;

use std::fmt;
use std::collections::HashMap;
use std::marker::PhantomData;
use ansi_term::Colour::{Red, Yellow, Blue};
use self::engines::*;

#[allow(unused_variables, unused_mut)]
fn main() {
    // Upper stages

    // Lower stages

    let mut rocket = Rocket {
        stages: vec![],
        payload_mass: 0.0,
    };

    // for (prop, amount) in rocket.stages().nth(0).unwrap().propellants_required() {
    //     println!("{} {}", prop, amount);
    // }
    // println!("");

    print_max_payloads(&mut rocket);
    // rocket.set_payload_for_target_deltav(DV_TO_ORBIT);

    println!("{:5}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}", "stage", "delta-v", "wet mass", "dry mass", "Start TWR", "End TWR", "burn time");
    let reversed_stages = rocket.stages().enumerate().collect::<Vec<_>>().into_iter().rev();
    for (i, stage) in reversed_stages {
        println!("{:5}: {:6.0} m/s  {:10.0}  {:10.0}  {:>10.2}  {:>10.2}  {:>10}", i, stage.delta_v(), stage.wet_mass(), stage.dry_mass(), stage.twr(), stage.max_g_force(), BurnTime(stage.burn_time()));
    }
    println!("{}", "-".repeat(78));
    println!("Total: {:6.0} m/s", rocket.delta_v());
    println!("Max G: {:10.2}", rocket.max_g_force());
    println!("");
    println!("");
    print_where_rocket_can_go(&rocket);
}

const GRAVITY: f64 = 9.82;
const DV_TO_ORBIT: f64 = 9400.0;
const DV_TO_GTO: f64 = DV_TO_ORBIT + 2440.0;
const DV_TO_GEO: f64 = DV_TO_GTO + 1850.0;
const DV_TO_TLI: f64 = DV_TO_GTO + 680.0; // 3120 from orbit
const DV_TO_LLO: f64 = DV_TO_TLI + 140.0 + 680.0;
const DV_TO_VENUS: f64 = DV_TO_TLI + 370.0; // 3490 from orbit
const DV_TO_VENUS_ORBIT: f64 = DV_TO_VENUS + 3800.0;
const DV_TO_MARS: f64 = DV_TO_TLI + 480.0;
const DV_TO_MARS_ORBIT: f64 = DV_TO_MARS + 1200.0;
const DV_TO_MERCURY: f64 = DV_TO_VENUS + 2060.0;
const DV_TO_JUPITER: f64 = DV_TO_MARS + 2700.0;

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

    fn twr(&self) -> f64 {
        self.engines().iter().map(|e| e.thrust * 1000.0).sum::<f64>() /
            self.wet_mass() / GRAVITY
    }

    fn propellants_required(&self) -> HashMap<&'static str, f64> {
        let mut result = HashMap::new();
        for engine in self.engines() {
            for (prop, amount) in engine.propellants_required() {
                *result.entry(prop.name).or_insert(0.0) += amount;
            }
        }
        result
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

    fn with_verniers(mut self, vernier: Engine) -> Self {
        let vernier = vernier.with_burn_time(self.engines[0].burn_time);
        self.engines.push(vernier);
        self.engines.push(vernier);
        self
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

    fn with_payload(mut self, payload: Box<Stage>) -> Self {
        self.stages.push(payload);
        self.payload_mass = 0.0;
        self
    }

    fn with_payload_mass(mut self, payload_mass: f64) -> Self {
        self.payload_mass = payload_mass;
        self
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
    print_if_rocket_can_go_to(dv, DV_TO_VENUS_ORBIT, "Low Venus Orbit");
    print_if_rocket_can_go_to(dv, DV_TO_MARS, "Mars");
    print_if_rocket_can_go_to(dv, DV_TO_MARS_ORBIT, "Low Martian Orbit");
    print_if_rocket_can_go_to(dv, DV_TO_MERCURY, "Mercury");
    print_if_rocket_can_go_to(dv, DV_TO_JUPITER, "Jupiter");
    if dv > DV_TO_GTO {
        println!("Note: Assumes no gravity assists");
    }
}

fn print_if_rocket_can_go_to(dv: f64, required_dv: f64, name: &str) {
    if dv > required_dv * 1.05 {
        let excess = dv - required_dv;
        println!("{}", Blue.paint(format!("This rocket can go to {} with {:.0} m/s excess dV", name, excess)));
    } else if dv > required_dv {
        println!("{}", Yellow.paint(format!("This rocket can go to {} without safety margins", name)));
    }
}

const ATLAS_DECOUPLER_MASS: f64 = 1610.0;

fn print_max_payloads(rocket: &mut Rocket) {
    print_max_payoad(rocket, DV_TO_ORBIT, "orbit");
    print_max_payoad(rocket, DV_TO_GTO * 1.015, "GTO");
    print_max_payoad(rocket, DV_TO_GEO * 1.015, "GEO");
    print_max_payoad(rocket, DV_TO_TLI * 1.015, "TLI");
    print_max_payoad(rocket, DV_TO_LLO * 1.015, "Lunar Orbit");
    print_max_payoad(rocket, DV_TO_VENUS * 1.015, "Venus");
    print_max_payoad(rocket, DV_TO_VENUS_ORBIT * 1.015, "Low Venus Orbit");
    print_max_payoad(rocket, DV_TO_MARS * 1.015, "Mars");
    print_max_payoad(rocket, DV_TO_MARS_ORBIT * 1.015, "Low Mars Orbit");
    print_max_payoad(rocket, DV_TO_MERCURY * 1.015, "Mercury");
    print_max_payoad(rocket, DV_TO_JUPITER * 1.015, "Jupiter");
}

fn print_max_payoad(rocket: &mut Rocket, required_dv: f64, name: &str) {
    let original_payload = rocket.payload_mass;
    rocket.set_payload_for_target_deltav(required_dv);
    if rocket.payload_mass > 0.0 {
        println!("Max to {}: {}", name, rocket.payload_mass);
    }
    rocket.payload_mass = original_payload;
}

fn probe(dry_mass: f64, burn_time: f64) -> SimpleStage {
    SimpleStage { dry_mass: dry_mass, engines: vec![THRUSTER_2.with_burn_time(burn_time)] }
}
