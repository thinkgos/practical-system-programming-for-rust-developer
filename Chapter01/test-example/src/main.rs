use std::process;

fn main() {
    println!("Hello, world!");
}

fn get_process_id() -> u32 {
    process::id()
}

#[cfg(test)]
mod tests {
    use super::get_process_id;

    #[test]
    fn test_if_process_id_is_returned() {
        assert!(get_process_id() > 0)
    }
}
