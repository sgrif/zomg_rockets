fn main() {
    let lr105_na_3 = Engine {
        mass: 460.0,
        isp: 309.0,
        thrust: 352.2,
        kerosene_per_second: 43.5978,
        lox_per_second: 70.5326,
        burn_time: 330,
    };
    let lr89_na_3 = Engine {
        mass: 641.0,
        isp: 282.0,
        thrust: 758.7,
        kerosene_per_second: 102.9093,
        lox_per_second: 166.4868,
        burn_time: 115,
    };

    let payload = Ballast {
        mass: 3.142,
    };
    let upper_stage = SimpleStage {
        isp: 278.0,
        mass_wet: 4534.0,
        mass_dry: 667.0,
        payload: payload,
    };
    let atlas_a = AtlasStage {
        mass_dry: 6774.0,
        sustainer: lr105_na_3,
        booster: lr89_na_3,
        kerosene_amount: 42220.8,
        lox_amount: 68304.9,
        payload: upper_stage,
    };

    println!("total mass: {}, delta v: {}", upper_stage.mass_wet(), upper_stage.delta_v());
}

const DECOUPLER_MASS: f64 = 52.0;
const KEROSENE_DENSITY: f64 = 0.82;
const LOX_DENSITY: f64 = 1.141;
const GRAVITY: f64 = 9.82;

trait Stage {
    fn delta_v(&self) -> f64;
    fn mass_wet(&self) -> f64;
}

#[derive(Debug, Clone, Copy)]
struct Ballast {
    mass: f64,
}

impl Stage for Ballast {
    fn delta_v(&self) -> f64 { 0.0 }
    fn mass_wet(&self) -> f64 { self.mass }
}

#[derive(Debug, Clone, Copy)]
struct AtlasStage<T> {
    mass_dry: f64,
    sustainer: Engine,
    booster: Engine,
    kerosene_amount: f64,
    lox_amount: f64,
    payload: T,
}

impl<T: Stage> AtlasStage<T> {
    fn isp_with_boosters(&self) -> f64 {
        (self.sustainer.thrust + self.booster.thrust * 2.0) /
            (self.sustainer.thrust / self.sustainer.isp + self.booster.thrust / self.booster.isp * 2.0)
    }

    fn mass_at_booster_burnout(&self) -> f64 {
        let kerosene_burned_by_boosters = self.booster.kerosene_per_second * self.booster.burn_time as f64 * 2.0;
        let lox_burned_by_boosters = self.booster.lox_per_second * self.booster.burn_time as f64 * 2.0;
        let kerosene_burned_by_sustainer = self.sustainer.kerosene_per_second * self.booster.burn_time as f64;
        let lox_burned_by_sustainer = self.sustainer.lox_per_second * self.booster.burn_time as f64;

        let kerosene_burned = kerosene_burned_by_boosters + kerosene_burned_by_sustainer;
        let lox_burned = lox_burned_by_boosters + lox_burned_by_sustainer;

        let mass_of_kerosene_burned = kerosene_burned * KEROSENE_DENSITY;
        let mass_of_lox_burned = lox_burned * LOX_DENSITY;

        self.mass_wet() - mass_of_kerosene_burned - mass_of_lox_burned
    }

    fn mass_after_booster_sep(&self) -> f64 {
        self.mass_at_booster_burnout() - self.booster.mass * 2.0 - DECOUPLER_MASS * 2.0
    }

    fn mass_at_burnout(&self) -> f64 {
        self.mass_dry - self.booster.mass * 2.0 - DECOUPLER_MASS * 2.0 + self.payload.mass_wet()
    }
}

impl<T: Stage> Stage for AtlasStage<T> {
    fn mass_wet(&self) -> f64 {
        self.mass_dry
            + self.kerosene_amount * KEROSENE_DENSITY
            + self.lox_amount * LOX_DENSITY
            + self.payload.mass_wet()
    }

    fn delta_v(&self) -> f64 {
        let delta_v_from_booster_stage = self.isp_with_boosters() * (self.mass_wet() / self.mass_at_booster_burnout()).ln() * GRAVITY;
        let delta_v_after_booster_sep = self.sustainer.isp * (self.mass_after_booster_sep() / self.mass_at_burnout()).ln() * GRAVITY;
        delta_v_from_booster_stage + delta_v_after_booster_sep + self.payload.delta_v()
    }
}

#[derive(Debug, Clone, Copy)]
struct SimpleStage<T: Stage> {
    mass_wet: f64,
    mass_dry: f64,
    isp: f64,
    payload: T,
}

impl<T: Stage> Stage for SimpleStage<T> {
    fn mass_wet(&self) -> f64 {
        self.mass_wet + self.payload.mass_wet()
    }

    fn delta_v(&self) -> f64 {
        let mass_dry_with_payload = self.mass_dry + self.payload.mass_wet();
        self.isp * (self.mass_wet() / mass_dry_with_payload).ln() * GRAVITY
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Engine {
    mass: f64,
    isp: f64,
    thrust: f64,
    kerosene_per_second: f64,
    lox_per_second: f64,
    burn_time: u16,
}
