use json::object;

use serial::prelude::*;
use std::io::prelude::*;

fn main() {
    // create json command
    let data_cmd = object! {detection:{}};
    let json_cmd = json::stringify(data_cmd);

    let luos_json = json_cmd + "\r";

    // open serial port
    let mut port = serial::open("COM3").unwrap();

    // configure port
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::BaudOther(1000000)).unwrap();
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })
    .unwrap();

    // send json
    port.write(luos_json.as_bytes()).unwrap();

    // receive json response
}
