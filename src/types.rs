use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    Bitcoin,
    Testnet,
    Testnet4,
    Fractal,
    Regtest,
    Signet,
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Bitcoin => "bitcoin",
            Self::Testnet => "testnet",
            Self::Testnet4 => "testnet4",
            Self::Fractal => "fractal",
            Self::Regtest => "regtest",
            Self::Signet => "signet",
        };
        s.fmt(f)
    }
}

impl std::str::FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bitcoin" => Ok(Self::Bitcoin),
            "testnet" => Ok(Self::Testnet),
            "testnet4" => Ok(Self::Testnet4),
            "fractal" => Ok(Self::Fractal),
            "regtest" => Ok(Self::Regtest),
            "signet" => Ok(Self::Signet),
            _ => Err(format!("Unknown log level: {s}")),
        }
    }
}