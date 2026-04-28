use actix_governor::{
    GlobalKeyExtractor, GovernorConfig, GovernorConfigBuilder, governor::middleware::NoOpMiddleware,
};

pub fn dump_governor() -> GovernorConfig<GlobalKeyExtractor, NoOpMiddleware> {
    GovernorConfigBuilder::default()
        .seconds_per_request(86400)
        .burst_size(1)
        .key_extractor(GlobalKeyExtractor)
        .finish()
        .unwrap()
}
