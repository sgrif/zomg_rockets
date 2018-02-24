#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fuel {
    pub name: &'static str,
    pub density: f64,
}

pub const KEROSENE: Fuel = Fuel { name: "Kerosene", density: 0.82 };
pub const LIQUID_OXYGEN: Fuel = Fuel { name: "LqdOxygen", density: 1.141 };

pub const UDMH: Fuel = Fuel { name: "UDMH", density: 0.791 };
pub const IRFNA_III: Fuel = Fuel { name: "IRFNA-III", density: 1.658 };
pub const IWFNA: Fuel = Fuel { name: "IWFNA", density: 1.513 };
pub const LIQUID_HYDROGEN: Fuel = Fuel { name: "Liquid Hydrogen", density: 0.07085 };

pub const PSPC: Fuel = Fuel { name: "PSPC", density: 1.74 };
pub const HTPB: Fuel = Fuel { name: "HTPB", density: 1.77 };

pub const HYDRAZINE: Fuel = Fuel { name: "Hydrazine", density: 1.004 };
pub const CAVEA_B: Fuel = Fuel { name: "Cavea-B", density: 1.501 };
pub const AEROZINE50: Fuel = Fuel { name: "Aerozine50", density: 0.9 };
pub const NTO: Fuel = Fuel { name: "NTO", density: 1.45 };
