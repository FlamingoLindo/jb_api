use super::dump;
use actix_governor::{GlobalKeyExtractor, GovernorConfig, governor::middleware::NoOpMiddleware};

#[derive(Clone)]
pub struct Governors {
    pub dump: GovernorConfig<GlobalKeyExtractor, NoOpMiddleware>,
}

impl Governors {
    pub fn init() -> Self {
        Self {
            dump: dump::dump_governor(),
        }
    }
}
