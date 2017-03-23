pub use super::fuels::*;

#[derive(Debug, Clone, Copy)]
pub struct Engine {
    pub name: &'static str,
    pub fuel_consumption: &'static [(Fuel, f64)],
    pub isp: f64,
    pub thrust: f64,
    pub mass: f64,
    pub burn_time: f64,
}

impl Engine {
    pub fn propellant_mass_per_second(&self) -> f64 {
        self.fuel_consumption.iter()
            .map(|&(fuel, rate)| fuel.density * rate)
            .sum()
    }

    pub fn propellants_required(&self) -> Vec<(Fuel, f64)> {
        self.fuel_consumption.iter()
            .map(|&(fuel, rate)| (fuel, rate * self.burn_time))
            .collect()
    }

    pub fn propellant_mass_for_full_burn(&self) -> f64 {
        self.propellant_mass_per_second() * self.burn_time
    }

    pub fn with_burn_time(&self, burn_time: f64) -> Self {
        let mut result = self.clone();
        result.burn_time = burn_time;
        result
    }
}

pub const BELL_8048: Engine = Engine {
    name: "Bell 8048 (XLR81-BA-5, Agena A)",
    fuel_consumption: &[(UDMH, 8.8115), (IRFNA_III, 10.7262)],
    isp: 276.0,
    thrust: 67.0,
    mass: 132.0,
    burn_time: 120.0,
};

pub const BELL_8081: Engine = Engine {
    name: "Bell 8081 (XLR81-BA-7, Agena B)",
    fuel_consumption: &[(UDMH, 8.9903), (IRFNA_III, 11.0327)],
    isp: 285.0,
    thrust: 71.0,
    mass: 132.0,
    burn_time: 240.0,
};

pub const BELL_8247: Engine = Engine {
    name: "Bell 8247 (XLR81-BA-13, Gemini ATV)",
    fuel_consumption: &[(UDMH, 8.8049), (IRFNA_III, 10.8052)],
    isp: 291.0,
    thrust: 71.0,
    mass: 132.0,
    burn_time: 240.0,
};

pub const LR105_NA_3: Engine = Engine {
    name: "LR105-NA-3",
    fuel_consumption: &[(LIQUID_OXYGEN, 70.5326), (KEROSENE, 43.5978)],
    isp: 309.0,
    thrust: 352.2,
    mass: 460.0,
    burn_time: 330.0,
};

pub const LR105_NA_5: Engine = Engine {
    name: "LR105-NA-5/6",
    fuel_consumption: &[(LIQUID_OXYGEN, 72.8447), (KEROSENE, 45.0270)],
    isp: 311.0,
    thrust: 366.1,
    mass: 413.0,
    burn_time: 350.0,
};

pub const LR101_NA_3: Engine = Engine {
    name: "LR101-NA-3 Vernier",
    fuel_consumption: &[(LIQUID_OXYGEN, 1.3296), (KEROSENE, 0.8222)],
    isp: 238.0,
    thrust: 4.448,
    mass: 24.0,
    burn_time: 360.0,
};

pub const LR101_NA_11: Engine = Engine {
    name: "LR101-NA-11 Vernier",
    fuel_consumption: &[(LIQUID_OXYGEN, 1.3153), (KEROSENE, 0.8512)],
    isp: 249.0,
    thrust: 4.524,
    mass: 24.0,
    burn_time: 360.0,
};

pub const LR89_NA_3: Engine = Engine {
    name: "LR89-NA-3",
    fuel_consumption: &[(LIQUID_OXYGEN, 166.4868), (KEROSENE, 102.9093)],
    isp: 282.0,
    thrust: 758.7,
    mass: 641.0,
    burn_time: 135.0,
};

pub const LR89_NA_5: Engine = Engine {
    name: "LR89-NA-5",
    fuel_consumption: &[(LIQUID_OXYGEN, 166.4868), (KEROSENE, 102.9093)],
    isp: 282.0,
    thrust: 758.7,
    mass: 720.0,
    burn_time: 150.0,
};

pub const LR89_NA_6: Engine = Engine {
    name: "LR89-NA-6",
    fuel_consumption: &[(LIQUID_OXYGEN, 177.4070), (KEROSENE, 109.5694)],
    isp: 290.0,
    thrust: 733.926,
    mass: 782.0,
    burn_time: 160.0,
};

pub const LR89_NA_7_1: Engine = Engine {
    name: "LR89-NA-7.1",
    fuel_consumption: &[(LIQUID_OXYGEN, 197.3125), (KEROSENE, 121.9634)],
    isp: 292.2,
    thrust: 922.651,
    mass: 1019.0,
    burn_time: 165.0,
};

pub const LR79_NA_9: Engine = Engine {
    name: "LR79-NA-9",
    fuel_consumption: &[(LIQUID_OXYGEN, 166.2447), (KEROSENE, 107.5894)],
    isp: 284.0,
    thrust: 774.0,
    mass: 934.0,
    burn_time: 165.0,
};

pub const LR79_NA_11: Engine = Engine {
    name: "LR79-NA-11",
    fuel_consumption: &[(LIQUID_OXYGEN, 181.1651), (KEROSENE, 117.2455)],
    isp: 286.2,
    thrust: 850.0,
    mass: 980.0,
    burn_time: 165.0,
};

pub const AJ10_104D: Engine = Engine {
    name: "AJ10-104D",
    fuel_consumption: &[(UDMH, 4.2831), (IRFNA_III, 5.7219)],
    isp: 278.0,
    thrust: 35.1,
    mass: 90.0,
    burn_time: 300.0,
};

pub const BABY_SERGEANT: Engine = Engine {
    name: "Baby Sergeant",
    fuel_consumption: &[(PSPC, 1.9950)],
    isp: 235.0,
    thrust: 8.0,
    mass: 5.670,
    burn_time: 6.345,
};

pub const THRUSTER: Engine = Engine {
    name: "1kN Thruster",
    fuel_consumption:  &[(HYDRAZINE, 0.5643)],
    isp: 198.0,
    thrust: 1.1,
    mass: 15.0,
    burn_time: 20.0 * 60.0,
};

pub const THRUSTER_2: Engine = Engine {
    name: "1kN Thruster",
    fuel_consumption: &[(AEROZINE50, 0.2817), (NTO, 0.2795)],
    isp: 281.725,
    thrust: 1.82,
    mass: 15.0,
    burn_time: 20.0 * 60.0,
};

pub const H1: Engine = Engine {
    name: "H-1 Saturn I",
    fuel_consumption: &[(LIQUID_OXYGEN, 202.1917), (KEROSENE, 126.1482)],
    isp: 289.0,
    thrust: 947.0,
    mass: 635.0,
    burn_time: 150.0,
};
