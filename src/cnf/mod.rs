//! Configuration

extern crate env_logger;

pub const LOGO: &str = r"
  ______ _______ ______  ______  _  ______ 
 / _____|_______|_____ \(______)| |/ _____)
( (____  _____   _____) )_     _| ( (____  
 \____ \|  ___) |  __  /| |   | | |\____ \ 
 _____) ) |_____| |  \ \| |__/ /| |_____) )
(______/|_______)_|   |_|_____/ |_(______/ 
                                          
";

pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_NAMESPACE: &str = "serdis";
pub const DEFAULT_DATABASE_NAME: &str = "services";

pub fn log_cfg() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();
}
