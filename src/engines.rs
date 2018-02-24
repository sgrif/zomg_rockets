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

pub const BELL_8096: Engine = Engine {
    name: "Bell 8096 (XLR81-BA-13, Gemini ATV)",
    fuel_consumption: &[(UDMH, 8.8049), (IRFNA_III, 10.8052)],
    isp: 291.0,
    thrust: 71.0,
    mass: 132.0,
    burn_time: 240.0,
};

pub const LR43_NA_5: Engine = Engine {
    name: "LR43-NA-5",
    fuel_consumption: &[(LIQUID_OXYGEN, 49.3816), (KEROSENE, 30.5239)],
    isp: 301.0,
    thrust: 240.2,
    mass: 844.0,
    burn_time: 330.0,
};

pub const LR105_NA_3: Engine = Engine {
    name: "LR105-NA-3",
    fuel_consumption: &[(LIQUID_OXYGEN, 70.5326), (KEROSENE, 43.5978)],
    isp: 309.0,
    thrust: 352.2,
    mass: 844.0,
    burn_time: 330.0,
};

pub const LR105_NA_5: Engine = Engine {
    name: "LR105-NA-5",
    fuel_consumption: &[(LIQUID_OXYGEN, 72.3793), (KEROSENE, 44.7393)],
    isp: 313.0,
    thrust: 366.1,
    mass: 758.0,
    burn_time: 350.0,
};

pub const LR105_NA_6: Engine = Engine {
    name: "LR105-NA-6",
    fuel_consumption: &[(LIQUID_OXYGEN, 73.7830), (KEROSENE, 45.6070)],
    isp: 313.0,
    thrust: 373.2,
    mass: 758.0,
    burn_time: 350.0,
};

pub const LR105_NA_7_1: Engine = Engine {
    name: "LR105-NA-7.1",
    fuel_consumption: &[(LIQUID_OXYGEN, 75.4324), (KEROSENE, 46.6265)],
    isp: 316.0,
    thrust: 385.2,
    mass: 862.0,
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
    thrust: 5.369,
    mass: 24.0,
    burn_time: 360.0,
};

pub const LR43_NA_3: Engine = Engine {
    name: "LR43-NA-3",
    fuel_consumption: &[(LIQUID_OXYGEN, 148.5149), (KEROSENE, 91.8005)],
    isp: 278.0,
    thrust: 667.2,
    mass: 720.0,
    burn_time: 135.0,
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
    fuel_consumption: &[(LIQUID_OXYGEN, 177.4070), (KEROSENE, 109.6594)],
    isp: 290.0,
    thrust: 831.4,
    mass: 828.0,
    burn_time: 150.0,
};

pub const LR89_NA_6: Engine = Engine {
    name: "LR89-NA-6",
    fuel_consumption: &[(LIQUID_OXYGEN, 180.6504), (KEROSENE, 111.6642)],
    isp: 290.0,
    thrust: 846.6,
    mass: 883.0,
    burn_time: 160.0,
};

pub const LR89_NA_7_1: Engine = Engine {
    name: "LR89-NA-7.1",
    fuel_consumption: &[(LIQUID_OXYGEN, 197.3125), (KEROSENE, 121.9634)],
    isp: 292.2,
    thrust: 931.7,
    mass: 1018.0,
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

pub const AJ10_42: Engine = Engine {
    name: "AJ10-42",
    fuel_consumption: &[(UDMH, 4.1946), (IRFNA_III, 6.1370)],
    isp: 267.0,
    thrust: 33.0,
    mass: 80.0,
    burn_time: 150.0,
};

pub const AJ10_142: Engine = Engine {
    name: "AJ10-142",
    fuel_consumption: &[(UDMH, 4.3052), (IWFNA, 6.2987)],
    isp: 270.0,
    thrust: 30.444,
    mass: 80.0,
    burn_time: 150.0,
};

pub const AJ10_104: Engine = Engine {
    name: "AJ10-104",
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

pub const HYDRAZINE_THRUSTER: Engine = Engine {
    name: "1kN Thruster",
    fuel_consumption:  &[(HYDRAZINE, 0.4911)],
    isp: 198.0,
    thrust: 0.957,
    mass: 16.0,
    burn_time: 20.0 * 60.0,
};

pub const CAVEA_THRUSTER: Engine = Engine {
    name: "2.2/3.6kN Thruster",
    fuel_consumption: &[(CAVEA_B, 0.7786)],
    isp: 258.225,
    thrust: 2.959,
    mass: 34.0,
    burn_time: 20.0 * 60.0,
};

pub const THRUSTER_1: Engine = Engine {
    name: "1kN Thruster",
    fuel_consumption: &[(AEROZINE50, 0.3022), (NTO, 0.2998)],
    isp: 262.625,
    thrust: 1.82,
    mass: 15.0,
    burn_time: 20.0 * 60.0,
};

pub const THRUSTER_2: Engine = Engine {
    name: "2.2/3.6kN Thruster",
    fuel_consumption: &[(AEROZINE50, 0.5634), (NTO, 0.5589)],
    isp: 281.725,
    thrust: 3.64,
    mass: 32.0,
    burn_time: 20.0 * 60.0,
};

pub const ALTAIR: Engine = Engine {
    name: "Altair",
    fuel_consumption: &[(PSPC, 3.4339)],
    isp: 256.0,
    thrust: 15.0,
    mass: 30.0,
    burn_time: 34.8,
};

pub const CASTOR_1: Engine = Engine {
    name: "Castor 1",
    fuel_consumption: &[(HTPB, 66.7076)],
    isp: 247.0,
    thrust: 268.632,
    mass: 535.0,
    burn_time: 28.1,
};

pub const H1: Engine = Engine {
    name: "H1 Saturn I",
    fuel_consumption: &[(KEROSENE, 126.1482), (LIQUID_OXYGEN, 202.1917)],
    isp: 289.0,
    thrust: 947.0,
    mass: 635.0,
    burn_time: 150.0,
};

pub const H1B: Engine = Engine {
    name: "H1 Saturn IB",
    fuel_consumption: &[(KEROSENE, 133.9858), (LIQUID_OXYGEN, 214.7539)],
    isp: 296.0,
    thrust: 1030.2,
    mass: 988.0,
    burn_time: 180.0,
};

pub const RL10A_1: Engine = Engine {
    name: "RL10A-1",
    fuel_consumption: &[(LIQUID_HYDROGEN, 38.0877), (LIQUID_OXYGEN, 11.8241)],
    isp: 422.0,
    thrust: 67.0,
    mass: 145.0,
    burn_time: 430.0,
};

pub const RL10A_3_1: Engine = Engine {
    name: "RL10A-3-1",
    fuel_consumption: &[(LIQUID_HYDROGEN, 37.1201), (LIQUID_OXYGEN, 11.5237)],
    isp: 433.0,
    thrust: 67.0,
    mass: 139.0,
    burn_time: 470.0,
};

pub const RL10A_3_3: Engine = Engine {
    name: "RL10A-3-3",
    fuel_consumption: &[(LIQUID_HYDROGEN, 36.2004), (LIQUID_OXYGEN, 11.2382)],
    isp: 444.0,
    thrust: 67.0,
    mass: 137.0,
    burn_time: 470.0,
};

pub const J2_200KLBF: Engine = Engine {
    name: "J-2-200klbf",
    fuel_consumption: &[(LIQUID_HYDROGEN, 464.3834), (LIQUID_OXYGEN, 158.6155)],
    isp: 424.0,
    thrust: 889.325,
    mass: 1610.0,
    burn_time: 350.0,
};
