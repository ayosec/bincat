mod ascii;
mod cycles;

use std::io::{self, BufWriter, Read, Write};
use std::process;

use ascii::EscapedChar;
use cycles::Cycle;

fn main() {
    let mut buf = [0; 4096];

    let (mut escape_prefix, escape_suffix) = if atty::is(atty::Stream::Stdout) {
        (Cycle::new(&["\x1b[7m", "\x1b[7;34m"]), "\x1b[m")
    } else {
        (Cycle::new(&["<"]), ">")
    };

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

            if (32..=128).contains(byte) || *byte == b'\n' {
                escape_prefix.reset();
                res = writer.write(&[*byte]).map(|_| ());
            } else {
                res = write!(
                    &mut writer,
                    "{}{}{}",
                    escape_prefix.next(),
                    EscapedChar(*byte),
                    escape_suffix
                );
            }

            if let Err(e) = res {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }
}
