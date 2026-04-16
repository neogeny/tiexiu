pub mod constants;
pub use constants::*;

pub use crate::util::cfg::*;

pub fn cfg(input: CfgA) -> Cfg {
    Cfg::fromenv(ENV_PREFIX).merge(&Cfg::new(input))
}
