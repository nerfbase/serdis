//! Configuration

extern crate env_logger;
extern crate std;

pub const LOGO: &str = r"
  ______ _______ ______  ______  _  ______ 
 / _____|_______|_____ \(______)| |/ _____)
( (____  _____   _____) )_     _| ( (____  
 \____ \|  ___) |  __  /| |   | | |\____ \ 
 _____) ) |_____| |  \ \| |__/ /| |_____) )
(______/|_______)_|   |_|_____/ |_(______/ 
";

pub const DEFAULT_HOST: &str = "localhost";

pub fn log_cfg() {
    std::env::set_var("RUST_LOG", "info");

    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();
}
