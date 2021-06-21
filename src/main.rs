use json::object;

use std::io;
use std::io::prelude::*;
use serial::prelude::*;

fn main() {

    let data_cmd = object!{detection:{}};
    let json_cmd = json::stringify(data_cmd);

    // open serial port
    let mut port = serial::open("COM3").unwrap();

    // configure port
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::BaudOther(1000000));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    });

    // send json
    port.write(json_cmd.as_bytes());

    // receive json response

    println!("{}", json_cmd);
}
