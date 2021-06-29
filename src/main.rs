use std::io::{self, BufWriter, Read, Write};
use std::process;

fn main() {
    let mut buf = [0; 4096];

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while let Ok(len) = reader.read(&mut buf) {
        if len == 0 {
            break;
        }

        for byte in &buf[..len] {
            let res;

            if (32..=128).contains(byte) || byte.is_ascii_whitespace() {
                res = writer.write(&[*byte]).map(|_| ());
            } else {
                res = write!(&mut writer, "<{:02X}>", *byte);
            }

            if let Err(e) = res {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }
}
