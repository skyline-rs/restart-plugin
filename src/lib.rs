use std::io::Read;
use std::net::TcpListener;

use skyline::nn;

const RESTART_PORT: u16 = 45423;

fn check_for_restart(tid: u64) -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", RESTART_PORT))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0u8; 8];

        stream.read(&mut buffer);

        if u64::from_be_bytes(buffer) == tid {
            unsafe {
                nn::oe::RequestToRelaunchApplication();
            }
        }
    }

    Ok(())
}

#[skyline::main(name = "restart")]
pub fn main() {
    std::thread::spawn(|| {
        let title_id = skyline::info::get_program_id();

        loop {
            check_for_restart(title_id);
        }
    });
}