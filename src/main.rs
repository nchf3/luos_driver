use json::object;

use serial::prelude::*;
use std::io::prelude::*;

const MINIMUM_JSON_SIZE: u32 = 2;

fn receive_json(port: &mut serial::SystemPort) {
    // receive json response
    let mut read_byte: [u8; 1] = [0];
    let mut saved_string: char = '0';
    let mut json_flag = false;
    let mut json_size = 0;
    for _bytes in 1..400 {
        port.read_exact(&mut read_byte).unwrap();
        let read_string = read_byte[0] as char;

        if read_string == '{' && !json_flag {
            // json start
            json_flag = true;
        }

        if read_string == '}' && saved_string == '{' {
            // empty json
            // stop counting json
            json_flag = false;
            json_size = 0;
        }

        if json_flag {
            json_size += 1;
        }

        if json_size >= MINIMUM_JSON_SIZE {
            print!("{}", saved_string);
        }

        // shift read data
        saved_string = read_string.clone();
    }
}

fn main() {
    // create json command
    let data_cmd = object! {
        detection: {
        }
    };
    let json_cmd = json::stringify(data_cmd);

    let luos_json = json_cmd + "\r";

    // open serial port
    let mut port = serial::open("COM6").unwrap();

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

    // wait for a json
    receive_json(&mut port);
}
