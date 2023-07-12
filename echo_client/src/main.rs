use std::{
    error,
    io::{self, prelude::*},
    net, process, str,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut stream = net::TcpStream::connect("127.0.0.1:50000")?;
    let stream_clone = stream.try_clone()?;
    ctrlc::set_handler(move || {
        stream_clone
            .shutdown(net::Shutdown::Both)
            .expect("shutdown call failed");
        process::exit(0);
    })?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

        let mut reader = io::BufReader::new(&stream);
        reader.fill_buf()?;
        print!("{}", str::from_utf8(reader.buffer())?);
    }
}
