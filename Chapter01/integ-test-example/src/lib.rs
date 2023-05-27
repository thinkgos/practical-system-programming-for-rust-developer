use std::process;

pub fn get_process_id() -> u32 {
    process::id()
}
