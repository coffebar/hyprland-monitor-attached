use std::env; // read env variables
use std::io::BufRead; // read unix socket
use std::io::BufReader; // read unix socket
use std::os::unix::net::UnixStream;
use std::process::Command; // execute system command

// listen Hyprland socket
fn listen(socket_addr: String) -> std::io::Result<()> {
    let stream = match UnixStream::connect(socket_addr) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect: {e:?}");
            return Err(e);
        }
    };
    let mut args: Vec<String> = env::args().collect();
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
                .args(args.clone())
                .output()
                .expect("Failed to execute command");
        }
    }
}

// read env variables and listen Hyprland unix socket
fn main() {
    match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(val) => {
            let socket = format!("/tmp/hypr/{}/.socket2.sock", val);
            // listen Hyprland socket
            match listen(socket) {
                Ok(()) => {}
                Err(..) => {}
            }
        }
        Err(e) => println!("Fatal Error: Hyprland is not run. {e}"),
    }
    std::process::exit(1);
}
