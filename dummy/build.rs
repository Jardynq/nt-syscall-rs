#![feature(trim_prefix_suffix)]

use core::panic;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").expect("$TARGET not set");
    let objdump = get_llvm_tool("llvm-objdump.exe");

    let target_dir = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .unwrap()
        .target_directory;
    let out_dir = target_dir.join(&target).join("dummy");
    std::fs::create_dir_all(&out_dir).unwrap();
    let out = out_dir.join("dummy").with_extension("rs");

    let obj = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("dummy")
        .with_extension("o");
    let obj = obj.to_str().unwrap();

    let status = Command::new("rustc")
        .args([
            "--crate-type=lib",
            "--target",
            &target,
            "--emit=obj",
            "-Copt-level=3",
            "-Cpanic=abort",
            "-Cdebuginfo=0",
            "-Crelocation-model=pic",
            "-o",
            obj,
            "src/lib.rs",
        ])
        .status()
        .unwrap();
    if !status.success() {
        panic!("Compilation failed");
    }

    let output = Command::new(objdump)
        .args([
            "--disassemble",
            "--section=.payload",
            "--demangle",
            "--disassembler-options=intel",
            "--disassembler-color=off",
            "--print-imm-hex",
            obj,
        ])
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("Objdump failed");
    }

    //panic!("{}", String::from_utf8_lossy(&output.stdout));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let output = parse_objdump(&stdout);
    let mut file = File::create(out).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn get_llvm_tool(tool: &str) -> PathBuf {
    let output = Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .expect("Failed to get sysroot");

    let sysroot = std::str::from_utf8(&output.stdout)
        .expect("Failed to parse sysroot")
        .trim();

    let toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    let (_, host) = toolchain.split_once('-').unwrap();

    let mut path = PathBuf::from(sysroot);
    path.push("lib");
    path.push("rustlib");
    path.push(host);
    path.push("bin");
    path.push(tool);

    if path.exists() {
        path
    } else {
        panic!("{} not found in sysroot", tool)
    }
}

type Lines = Vec<(Vec<u8>, String)>;
type Function = (String, Lines);

fn parse_objdump(stdout: &str) -> String {
    let header_pattern = regex::Regex::new(r"^[0-9a-fA-F]+\s+<([a-zA-Z0-9@_]+)>:\s*$").unwrap();
    let symbol_pattern = regex::Regex::new(r"<.+>").unwrap();
    let asm_pattern =
        regex::Regex::new(r"^\s+[0-9a-fA-F]+:\s((?:[0-9a-fA-F]{2}\s?)+)\s*(.+)$").unwrap();

    let mut functions: Vec<String> = Vec::new();
    let mut current: Option<Function> = None;
    for line in stdout.lines() {
        if let Some(captures) = header_pattern.captures(line) {
            if let Some((name, lines)) = current.take() {
                functions.push(gen_function(name, lines));
            }

            let name = captures[1].trim_ascii();
            let name = name.trim_prefix("_"); // cdecl
            let name = name.trim_prefix("_").split("@").next().unwrap(); // stdcall
            let name = name.trim_prefix("@").split("@").next().unwrap(); // fastcall
            let name = name.to_uppercase();

            current = Some((name, Vec::new()));
        } else if let Some((_, lines)) = &mut current
            && let Some(captures) = asm_pattern.captures(line)
        {
            let bytes: Vec<u8> = captures[1]
                .trim_ascii()
                .split_ascii_whitespace()
                .map(|byte_str| u8::from_str_radix(byte_str, 16).unwrap())
                .collect();

            let comment = symbol_pattern
                .replace_all(&captures[2], "")
                .trim_ascii()
                .replace("\t", " ");

            lines.push((bytes, comment));
        }
    }
    if let Some((name, lines)) = current.take() {
        functions.push(gen_function(name, lines));
    }

    functions.join("\n\n")
}

fn gen_function(name: String, lines: Vec<(Vec<u8>, String)>) -> String {
    let len: usize = lines.iter().map(|(bytes, _)| bytes.len()).sum();

    let body = lines
        .into_iter()
        .map(|(bytes, comment)| {
            let bytes = bytes
                .iter()
                .map(|b| format!("0x{b:02x},"))
                .collect::<Vec<String>>()
                .join("");
            (format!("\t{bytes}"), comment)
        })
        .collect::<Vec<(String, String)>>();

    let width = body.iter().map(|(b, _)| b.len()).max().unwrap_or(0);
    let body = body
        .iter()
        .map(|(bytes, comment)| {
            if comment.is_empty() {
                bytes.to_string()
            } else {
                format!("{bytes:width$} // {comment}")
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!("pub static {name}: [u8; {len}] = [\n{body}\n];")
}
