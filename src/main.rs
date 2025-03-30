use std::error::Error;

use codecrafters_kafka::run_server;

fn main() -> Result<(), Box<dyn Error>> {
    run_server()
}
