use std::env;
use std::fs::OpenOptions;
use std::io::{self, ErrorKind, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::thread::sleep;
use std::time::{Duration, Instant};

const DEFAULT_DEV: &str = "/dev/hidraw1";
const O_NONBLOCK: i32 = 0o4000;
const ROWS: usize = 5;
const COLS: usize = 12;
const LAYERS: usize = 4;

struct KeyName {
    name: &'static str,
    code: u16,
}

const KEY_NAMES: &[KeyName] = &[
    KeyName {
        name: "NO",
        code: 0x0000,
    },
    KeyName {
        name: "TRNS",
        code: 0x0001,
    },
    KeyName {
        name: "A",
        code: 0x0004,
    },
    KeyName {
        name: "B",
        code: 0x0005,
    },
    KeyName {
        name: "C",
        code: 0x0006,
    },
    KeyName {
        name: "D",
        code: 0x0007,
    },
    KeyName {
        name: "E",
        code: 0x0008,
    },
    KeyName {
        name: "F",
        code: 0x0009,
    },
    KeyName {
        name: "G",
        code: 0x000a,
    },
    KeyName {
        name: "H",
        code: 0x000b,
    },
    KeyName {
        name: "I",
        code: 0x000c,
    },
    KeyName {
        name: "J",
        code: 0x000d,
    },
    KeyName {
        name: "K",
        code: 0x000e,
    },
    KeyName {
        name: "L",
        code: 0x000f,
    },
    KeyName {
        name: "M",
        code: 0x0010,
    },
    KeyName {
        name: "N",
        code: 0x0011,
    },
    KeyName {
        name: "O",
        code: 0x0012,
    },
    KeyName {
        name: "P",
        code: 0x0013,
    },
    KeyName {
        name: "Q",
        code: 0x0014,
    },
    KeyName {
        name: "R",
        code: 0x0015,
    },
    KeyName {
        name: "S",
        code: 0x0016,
    },
    KeyName {
        name: "T",
        code: 0x0017,
    },
    KeyName {
        name: "U",
        code: 0x0018,
    },
    KeyName {
        name: "V",
        code: 0x0019,
    },
    KeyName {
        name: "W",
        code: 0x001a,
    },
    KeyName {
        name: "X",
        code: 0x001b,
    },
    KeyName {
        name: "Y",
        code: 0x001c,
    },
    KeyName {
        name: "Z",
        code: 0x001d,
    },
    KeyName {
        name: "1",
        code: 0x001e,
    },
    KeyName {
        name: "2",
        code: 0x001f,
    },
    KeyName {
        name: "3",
        code: 0x0020,
    },
    KeyName {
        name: "4",
        code: 0x0021,
    },
    KeyName {
        name: "5",
        code: 0x0022,
    },
    KeyName {
        name: "6",
        code: 0x0023,
    },
    KeyName {
        name: "7",
        code: 0x0024,
    },
    KeyName {
        name: "8",
        code: 0x0025,
    },
    KeyName {
        name: "9",
        code: 0x0026,
    },
    KeyName {
        name: "0",
        code: 0x0027,
    },
    KeyName {
        name: "ENT",
        code: 0x0028,
    },
    KeyName {
        name: "ENTER",
        code: 0x0028,
    },
    KeyName {
        name: "ESC",
        code: 0x0029,
    },
    KeyName {
        name: "BSPC",
        code: 0x002a,
    },
    KeyName {
        name: "BACKSPACE",
        code: 0x002a,
    },
    KeyName {
        name: "TAB",
        code: 0x002b,
    },
    KeyName {
        name: "SPC",
        code: 0x002c,
    },
    KeyName {
        name: "SPACE",
        code: 0x002c,
    },
    KeyName {
        name: "MINS",
        code: 0x002d,
    },
    KeyName {
        name: "MINUS",
        code: 0x002d,
    },
    KeyName {
        name: "EQL",
        code: 0x002e,
    },
    KeyName {
        name: "EQUAL",
        code: 0x002e,
    },
    KeyName {
        name: "LBRC",
        code: 0x002f,
    },
    KeyName {
        name: "LBRACKET",
        code: 0x002f,
    },
    KeyName {
        name: "RBRC",
        code: 0x0030,
    },
    KeyName {
        name: "RBRACKET",
        code: 0x0030,
    },
    KeyName {
        name: "BSLS",
        code: 0x0031,
    },
    KeyName {
        name: "BACKSLASH",
        code: 0x0031,
    },
    KeyName {
        name: "SCLN",
        code: 0x0033,
    },
    KeyName {
        name: "SEMICOLON",
        code: 0x0033,
    },
    KeyName {
        name: "QUOT",
        code: 0x0034,
    },
    KeyName {
        name: "QUOTE",
        code: 0x0034,
    },
    KeyName {
        name: "GRV",
        code: 0x0035,
    },
    KeyName {
        name: "GRAVE",
        code: 0x0035,
    },
    KeyName {
        name: "COMM",
        code: 0x0036,
    },
    KeyName {
        name: "COMMA",
        code: 0x0036,
    },
    KeyName {
        name: "DOT",
        code: 0x0037,
    },
    KeyName {
        name: "SLASH",
        code: 0x0038,
    },
    KeyName {
        name: "CAPS",
        code: 0x0039,
    },
    KeyName {
        name: "F1",
        code: 0x003a,
    },
    KeyName {
        name: "F2",
        code: 0x003b,
    },
    KeyName {
        name: "F3",
        code: 0x003c,
    },
    KeyName {
        name: "F4",
        code: 0x003d,
    },
    KeyName {
        name: "F5",
        code: 0x003e,
    },
    KeyName {
        name: "F6",
        code: 0x003f,
    },
    KeyName {
        name: "F7",
        code: 0x0040,
    },
    KeyName {
        name: "F8",
        code: 0x0041,
    },
    KeyName {
        name: "F9",
        code: 0x0042,
    },
    KeyName {
        name: "F10",
        code: 0x0043,
    },
    KeyName {
        name: "F11",
        code: 0x0044,
    },
    KeyName {
        name: "F12",
        code: 0x0045,
    },
    KeyName {
        name: "INS",
        code: 0x0049,
    },
    KeyName {
        name: "HOME",
        code: 0x004a,
    },
    KeyName {
        name: "PGUP",
        code: 0x004b,
    },
    KeyName {
        name: "DEL",
        code: 0x004c,
    },
    KeyName {
        name: "END",
        code: 0x004d,
    },
    KeyName {
        name: "PGDN",
        code: 0x004e,
    },
    KeyName {
        name: "RIGHT",
        code: 0x004f,
    },
    KeyName {
        name: "LEFT",
        code: 0x0050,
    },
    KeyName {
        name: "DOWN",
        code: 0x0051,
    },
    KeyName {
        name: "UP",
        code: 0x0052,
    },
    KeyName {
        name: "LCTL",
        code: 0x00e0,
    },
    KeyName {
        name: "LCTRL",
        code: 0x00e0,
    },
    KeyName {
        name: "LSFT",
        code: 0x00e1,
    },
    KeyName {
        name: "LSHIFT",
        code: 0x00e1,
    },
    KeyName {
        name: "LALT",
        code: 0x00e2,
    },
    KeyName {
        name: "LGUI",
        code: 0x00e3,
    },
    KeyName {
        name: "RCTL",
        code: 0x00e4,
    },
    KeyName {
        name: "RCTRL",
        code: 0x00e4,
    },
    KeyName {
        name: "RSFT",
        code: 0x00e5,
    },
    KeyName {
        name: "RSHIFT",
        code: 0x00e5,
    },
    KeyName {
        name: "RALT",
        code: 0x00e6,
    },
    KeyName {
        name: "RGUI",
        code: 0x00e7,
    },
    KeyName {
        name: "MO1",
        code: 0x5101,
    },
    KeyName {
        name: "MO2",
        code: 0x5102,
    },
    KeyName {
        name: "MO3",
        code: 0x5103,
    },
    KeyName {
        name: "TO0",
        code: 0x5000,
    },
    KeyName {
        name: "TO1",
        code: 0x5001,
    },
    KeyName {
        name: "TO2",
        code: 0x5002,
    },
    KeyName {
        name: "TO3",
        code: 0x5003,
    },
];

fn usage(program: &str) {
    eprintln!(
        "usage:\n  {0} [dev] protocol\n  {0} [dev] rawget <layer> <row> <col>\n  {0} [dev] rawbuf <offset> <size>\n  {0} [dev] bufdump <offset> <size>\n  {0} [dev] setbufkey <offset> <key|0xNNNN>\n  {0} [dev] dump [layers]\n  {0} [dev] get <layer> <row> <col>\n  {0} [dev] set <layer> <row> <col> <key|0xNNNN>\n\ndefault dev: {1}",
        program, DEFAULT_DEV
    );
}

fn parse_i32(s: &str, min: i32, max: i32, label: &str) -> Result<i32, String> {
    let value = s
        .parse::<i32>()
        .map_err(|_| format!("{label} must be {min}..{max}"))?;
    if value < min || value > max {
        return Err(format!("{label} must be {min}..{max}"));
    }
    Ok(value)
}

fn parse_keycode(s: &str) -> Result<u16, String> {
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return u16::from_str_radix(hex, 16).map_err(|_| format!("invalid keycode: {s}"));
    }
    KEY_NAMES
        .iter()
        .find(|item| item.name.eq_ignore_ascii_case(s))
        .map(|item| item.code)
        .ok_or_else(|| format!("unknown key name: {s}"))
}

fn key_name(code: u16) -> String {
    KEY_NAMES
        .iter()
        .find(|item| item.code == code)
        .map(|item| item.name.to_string())
        .unwrap_or_else(|| format!("0x{code:04x}"))
}

fn open_device(path: &str) -> io::Result<std::fs::File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(O_NONBLOCK)
        .open(path)
}

fn drain(file: &mut std::fs::File) -> io::Result<()> {
    let mut discard = [0u8; 64];
    loop {
        match file.read(&mut discard) {
            Ok(0) => return Ok(()),
            Ok(_) => {}
            Err(err)
                if err.kind() == ErrorKind::WouldBlock || err.kind() == ErrorKind::Interrupted =>
            {
                return Ok(());
            }
            Err(err) => return Err(err),
        }
    }
}

fn transact(file: &mut std::fs::File, req: [u8; 32]) -> io::Result<[u8; 32]> {
    drain(file)?;
    file.write_all(&req)?;

    let deadline = Instant::now() + Duration::from_secs(1);
    let mut resp = [0u8; 32];
    let mut offset = 0;
    loop {
        match file.read(&mut resp[offset..]) {
            Ok(0) => {}
            Ok(n) => {
                offset += n;
                if offset >= resp.len() {
                    return Ok(resp);
                }
            }
            Err(err)
                if err.kind() == ErrorKind::WouldBlock || err.kind() == ErrorKind::Interrupted => {}
            Err(err) => return Err(err),
        }

        if Instant::now() >= deadline {
            return Err(io::Error::new(
                ErrorKind::TimedOut,
                "timeout waiting for response",
            ));
        }
        sleep(Duration::from_millis(5));
    }
}

fn get_buffer(file: &mut std::fs::File, offset: u16, size: u8) -> io::Result<Vec<u8>> {
    if size > 28 {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "buffer size must be <= 28",
        ));
    }
    let mut req = [0u8; 32];
    req[0] = 0x12;
    req[1] = (offset >> 8) as u8;
    req[2] = (offset & 0xff) as u8;
    req[3] = size;
    let resp = transact(file, req)?;
    if resp[0] != 0x12 {
        return Err(io::Error::other(format!(
            "unexpected get_buffer response command 0x{:02x}",
            resp[0]
        )));
    }
    Ok(resp[4..4 + size as usize].to_vec())
}

fn set_buffer(file: &mut std::fs::File, offset: u16, data: &[u8]) -> io::Result<()> {
    if data.len() > 28 {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "buffer size must be <= 28",
        ));
    }
    let mut req = [0u8; 32];
    req[0] = 0x13;
    req[1] = (offset >> 8) as u8;
    req[2] = (offset & 0xff) as u8;
    req[3] = data.len() as u8;
    req[4..4 + data.len()].copy_from_slice(data);
    let resp = transact(file, req)?;
    if resp[0] != 0x13 {
        return Err(io::Error::other(format!(
            "unexpected set_buffer response command 0x{:02x}",
            resp[0]
        )));
    }
    Ok(())
}

fn key_offset(layer: i32, row: i32, col: i32) -> u16 {
    (((layer as usize * ROWS * COLS) + (row as usize * COLS) + col as usize) * 2) as u16
}

fn get_key(file: &mut std::fs::File, layer: i32, row: i32, col: i32) -> io::Result<u16> {
    let buf = get_buffer(file, key_offset(layer, row, col), 2)?;
    Ok(u16::from_be_bytes([buf[0], buf[1]]))
}

fn set_key(file: &mut std::fs::File, layer: i32, row: i32, col: i32, code: u16) -> io::Result<()> {
    let mut req = [0u8; 32];
    req[0] = 0x05;
    req[1] = layer as u8;
    req[2] = row as u8;
    req[3] = col as u8;
    req[4] = (code >> 8) as u8;
    req[5] = (code & 0xff) as u8;
    let resp = transact(file, req)?;
    if resp[0] != 0x05 {
        return Err(io::Error::other(format!(
            "unexpected set response command 0x{:02x}",
            resp[0]
        )));
    }
    Ok(())
}

fn parse_position(args: &[String], start: usize) -> Result<(i32, i32, i32), String> {
    let layer = parse_i32(&args[start], 0, 15, "layer")?;
    let row = parse_i32(&args[start + 1], 0, ROWS as i32 - 1, "row")?;
    let col = parse_i32(&args[start + 2], 0, COLS as i32 - 1, "col")?;
    Ok((layer, row, col))
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map(String::as_str).unwrap_or("th40-via9");

    let mut dev = DEFAULT_DEV;
    let mut arg = 1;
    if args.get(1).is_some_and(|value| value.starts_with("/dev/")) {
        dev = &args[1];
        arg = 2;
    }
    let Some(command) = args.get(arg).map(String::as_str) else {
        usage(program);
        return Err("missing command".to_string());
    };

    let mut file = open_device(dev).map_err(|err| format!("open {dev}: {err}"))?;

    match command {
        "protocol" => {
            let mut req = [0u8; 32];
            req[0] = 0x01;
            let resp = transact(&mut file, req).map_err(|err| err.to_string())?;
            let version = u16::from_be_bytes([resp[1], resp[2]]);
            println!("protocol 0x{version:04x} ({version})");
        }
        "rawget" => {
            if args.len() != arg + 4 {
                usage(program);
                return Err("invalid rawget arguments".to_string());
            }
            let (layer, row, col) = parse_position(&args, arg + 1)?;
            let mut req = [0u8; 32];
            req[0] = 0x04;
            req[1] = layer as u8;
            req[2] = row as u8;
            req[3] = col as u8;
            let resp = transact(&mut file, req).map_err(|err| err.to_string())?;
            print!("request:");
            for byte in req {
                print!(" {byte:02x}");
            }
            print!("\nresponse:");
            for byte in resp {
                print!(" {byte:02x}");
            }
            println!();
        }
        "rawbuf" => {
            if args.len() != arg + 3 {
                usage(program);
                return Err("invalid rawbuf arguments".to_string());
            }
            let offset = parse_i32(&args[arg + 1], 0, 65535, "offset")? as u16;
            let size = parse_i32(&args[arg + 2], 1, 28, "size")? as u8;
            let buf = get_buffer(&mut file, offset, size).map_err(|err| err.to_string())?;
            print!("buffer {offset} {size}:");
            for byte in buf {
                print!(" {byte:02x}");
            }
            println!();
        }
        "bufdump" => {
            if args.len() != arg + 3 {
                usage(program);
                return Err("invalid bufdump arguments".to_string());
            }
            let offset = parse_i32(&args[arg + 1], 0, 65535, "offset")?;
            let size = parse_i32(&args[arg + 2], 1, 1024, "size")?;
            let mut pos = 0;
            while pos < size {
                let chunk = (size - pos).min(28) as u8;
                let buf = get_buffer(&mut file, (offset + pos) as u16, chunk)
                    .map_err(|err| err.to_string())?;
                print!("{:04x}:", offset + pos);
                for byte in buf {
                    print!(" {byte:02x}");
                }
                println!();
                pos += chunk as i32;
                sleep(Duration::from_millis(20));
            }
        }
        "setbufkey" => {
            if args.len() != arg + 3 {
                usage(program);
                return Err("invalid setbufkey arguments".to_string());
            }
            let offset = parse_i32(&args[arg + 1], 0, 65534, "offset")?;
            if offset % 2 != 0 {
                return Err("offset must be even for a keycode".to_string());
            }
            let code = parse_keycode(&args[arg + 2])?;
            set_buffer(&mut file, offset as u16, &code.to_be_bytes())
                .map_err(|err| err.to_string())?;
            println!(
                "set buffer key at {offset} = {} (0x{code:04x})",
                key_name(code)
            );
        }
        "dump" => {
            let layers = if args.len() == arg + 2 {
                parse_i32(&args[arg + 1], 1, 16, "layers")? as usize
            } else {
                LAYERS
            };
            for layer in 0..layers {
                println!("layer {layer}");
                for row in 0..ROWS {
                    print!("R{row}:");
                    for col in 0..COLS {
                        let code = get_key(&mut file, layer as i32, row as i32, col as i32)
                            .map_err(|err| err.to_string())?;
                        print!(" {:<8}", key_name(code));
                    }
                    println!();
                }
                println!();
            }
        }
        "get" => {
            if args.len() != arg + 4 {
                usage(program);
                return Err("invalid get arguments".to_string());
            }
            let (layer, row, col) = parse_position(&args, arg + 1)?;
            let code = get_key(&mut file, layer, row, col).map_err(|err| err.to_string())?;
            println!("L{layer} R{row} C{col} = {} (0x{code:04x})", key_name(code));
        }
        "set" => {
            if args.len() != arg + 5 {
                usage(program);
                return Err("invalid set arguments".to_string());
            }
            let (layer, row, col) = parse_position(&args, arg + 1)?;
            let code = parse_keycode(&args[arg + 4])?;
            set_key(&mut file, layer, row, col, code).map_err(|err| err.to_string())?;
            println!(
                "set L{layer} R{row} C{col} = {} (0x{code:04x})",
                key_name(code)
            );
        }
        _ => {
            usage(program);
            return Err(format!("unknown command: {command}"));
        }
    }

    Ok(())
}
