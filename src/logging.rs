use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) fn setup(verbosity_level: u8) {
    let (crate_verbosity, deps_verbosity) = match verbosity_level {
        0 => (Level::INFO, Level::WARN),
        1 => (Level::DEBUG, Level::INFO),
        2 => (Level::TRACE, Level::DEBUG),
        _ => (Level::TRACE, Level::TRACE),
    };
    let filter = filter::Targets::new()
        .with_target(env!("CARGO_CRATE_NAME"), crate_verbosity)
        .with_default(deps_verbosity);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::event_enabled;
    const MY_CRATE: &str = env!("CARGO_CRATE_NAME");
    const DEP_CRATE: &str = "DEP";

    #[test]
    fn verbosity_0() {
        setup(0);
        assert!(!event_enabled!(target: MY_CRATE, Level::TRACE, ""));
        assert!(!event_enabled!(target: MY_CRATE, Level::DEBUG, ""));
        assert!(event_enabled!(target: MY_CRATE, Level::INFO, ""));
        assert!(event_enabled!(target: MY_CRATE, Level::WARN, ""));
        assert!(event_enabled!(target: MY_CRATE, Level::ERROR, ""));

        assert!(!event_enabled!(target: DEP_CRATE, Level::TRACE, ""));
        assert!(!event_enabled!(target: DEP_CRATE, Level::DEBUG, ""));
        assert!(!event_enabled!(target: DEP_CRATE, Level::INFO, ""));
        assert!(event_enabled!(target: DEP_CRATE, Level::WARN, ""));
        assert!(event_enabled!(target: DEP_CRATE, Level::ERROR, ""));
    }

    // We can't test other verbosity levels, since Subscriber::init() can only be called once
}
