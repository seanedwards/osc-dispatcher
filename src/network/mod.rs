pub mod agent;
pub mod tcp;
pub mod udp;

use crate::config;

trait Filter {
    fn filter(config: &config::Config);
}
