use slog::{o, Logger};

use crate::error;
use crate::settings::Settings;

#[derive(Clone, Debug)]
pub struct State {
    pub logger: Logger,
    pub settings: Settings,
}

impl State {
    pub async fn new(settings: &Settings, logger: &Logger) -> Result<Self, error::Error> {
        let logger = logger.new(o!("path" => format!("{}", &settings.service.path.display())));

        // FIXME Check assets directory exists

        Ok(Self {
            logger,
            settings: settings.clone(),
        })
    }
}
