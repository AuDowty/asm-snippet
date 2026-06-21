use std::io::Read;
use std::process::ExitCode;

use clap::Parser;
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter};

#[derive(Parser)]
#[command(
    name = "asm-snippet",
    version,
    about = "Disassemble x86/x64 byte strings into Intel-syntax instructions"
)]
struct Cli {
    input: String,
    #[arg(long, default_value_t = 64)]
    bits: u32,
    #[arg(long, value_parser = parse_hex_u64, default_value = "0")]
    base: u64,
    #[arg(long)]
    raw: bool,
    #[arg(long)]
    binary: bool,
}

fn parse_hex_u64(s: &str) -> Result<u64, String> {
    let s = s.trim_start_matches("0x").trim_start_matches("0X");
    u64::from_str_radix(s, 16).map_err(|e| format!("not hex: {e}"))
}

fn main() -> ExitCode {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let msg = info.payload().downcast_ref::<String>().map(|s| s.as_str())
            .or_else(|| info.payload().downcast_ref::<&str>().copied())
            .unwrap_or("");
        if msg.contains("failed printing to stdout") {
            std::process::exit(0);
        }
        default_hook(info);
    }));

    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), String> {
    if cli.bits != 16 && cli.bits != 32 && cli.bits != 64 {
        return Err("--bits must be 16, 32, or 64".into());
    }
    let bytes = collect_bytes(&cli.input, cli.binary)?;
    if bytes.is_empty() {
        return Err("no input bytes".into());
    }

    let mut decoder = Decoder::with_ip(cli.bits, &bytes, cli.base, DecoderOptions::NONE);
    let mut formatter = IntelFormatter::new();
    formatter.options_mut().set_first_operand_char_index(8);
    let mut instr = Instruction::default();
    let mut text = String::new();

    while decoder.can_decode() {
        decoder.decode_out(&mut instr);
        text.clear();
        formatter.format(&instr, &mut text);
        if cli.raw {
            println!("{text}");
        } else {
            let start = (instr.ip() - cli.base) as usize;
            let len = instr.len();
            let raw_bytes: Vec<String> = bytes[start..start + len]
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            println!("0x{:08x}  {:<24}  {text}", instr.ip(), raw_bytes.join(" "));
        }
    }
    Ok(())
}

fn collect_bytes(input: &str, binary: bool) -> Result<Vec<u8>, String> {
    if input == "-" {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf).map_err(|e| e.to_string())?;
        if binary {
            return Ok(buf);
        }
        let s = String::from_utf8_lossy(&buf);
        return parse_hex(&s);
    }
    if binary {
        return std::fs::read(input)
            .map_err(|e| format!("read {input}: {e}"));
    }
    parse_hex(input)
}

fn parse_hex(input: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = input
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    if cleaned.len() % 2 != 0 {
        return Err(format!("odd number of hex digits ({})", cleaned.len()));
    }
    let mut out = Vec::with_capacity(cleaned.len() / 2);
    for i in (0..cleaned.len()).step_by(2) {
        out.push(
            u8::from_str_radix(&cleaned[i..i + 2], 16)
                .map_err(|e| format!("bad hex pair at {i}: {e}"))?,
        );
    }
    Ok(out)
}
