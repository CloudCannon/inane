pub use options::{InaneInboundConfig, InaneOptions};
mod logging;
mod options;

pub struct InaneState {
    pub options: InaneOptions,
}

impl InaneState {
    pub fn new(options: InaneOptions) -> Self {
        Self { options }
    }

    pub fn run(&mut self) {
        let log = &self.options.logger;
        log.status(&format!("Running Inane v{}", self.options.version));
        log.v_info("Running in verbose mode");

        log.info(format!("Message: {}", self.options.message));
    }
}
