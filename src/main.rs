use std::env; // read env variables
use std::io::BufRead; // read unix socket
use std::io::BufReader; // read unix socket
use std::os::unix::net::UnixStream;
use std::process::Command; // execute system command

// listen Hyprland socket
fn listen(socket_addr: String, script_attached: &str, script_detached: Option<&str>) -> std::io::Result<()> {
    let stream = match UnixStream::connect(socket_addr) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect: {e:?}");
            return Err(e);
        }
    };
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: provide a script to execute.");
        std::process::exit(1);
    }
    args.remove(0);
    let mut reader = BufReader::new(stream);
    loop {
        // read message from socket
        let mut buf: Vec<u8> = vec![];
        reader.read_until(b'\n', &mut buf).unwrap();
        let data = String::from_utf8_lossy(&buf);
        let data_parts: Vec<&str> = data.trim().split(">>").collect();
        if data_parts[0] == "monitoradded" {
            Command::new("/bin/sh")
                .arg(script_attached)
                .args(args.clone())
                .spawn()
                .expect("Failed to execute command");
        } else if data_parts[0] == "monitorremoved" {
            if let Some(script_detached) = script_detached {
                Command::new("/bin/sh")
                    .arg(script_detached)
                    .args(args.clone())
                    .spawn()
                    .expect("Failed to execute command");
            }
        }
    }
}

// read env variables and listen Hyprland unix socket
fn main() {
    match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(val) => {
            let socket = format!("/tmp/hypr/{}/.socket2.sock", val);
            let script_attached = std::env::args().nth(1).expect("Missing script for monitor attached");
            let script_detached = std::env::args().nth(2);
            // listen Hyprland socket
            match listen(socket, &script_attached, script_detached.as_deref()) {
                Ok(()) => {}
                Err(..) => {}
            }
        }
        Err(e) => println!("Fatal Error: Hyprland is not run. {e}"),
    }
    std::process::exit(1);
}
