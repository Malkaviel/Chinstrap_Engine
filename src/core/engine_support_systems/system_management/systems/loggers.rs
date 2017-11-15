use core::engine_support_systems::error_handling::error::GameResult;
use core::engine_support_systems::system_management::system_types::{SystemType, VSystem};

//TODO: Rewrite the Log trait

pub trait VLog : VSystem {
    fn start_up(&self) -> GameResult<Box<VLog>>;
    fn write_to_dedicated_log(&self, subsystem_type: SystemType, message: &str) -> GameResult<()>; //The dedicated log file of a system.
    fn write_to_main_log(&self, message: &str) -> GameResult<()>;
}