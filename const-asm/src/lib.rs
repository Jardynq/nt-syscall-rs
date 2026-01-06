use proc_macro::{TokenStream, TokenTree};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::LazyLock;
use std::time::Duration;
use wait_timeout::ChildExt;

#[proc_macro]
pub fn assemble32(input: TokenStream) -> TokenStream {
    inner("x32", input).parse().unwrap()
}

#[proc_macro]
pub fn assemble64(input: TokenStream) -> TokenStream {
    inner("x64", input).parse().unwrap()
}

#[proc_macro]
pub fn __debug_clear_cache(_: TokenStream) -> TokenStream {
    let _ = fs::remove_dir_all(&*CACHE_PATH);
    TokenStream::new()
}

static KSTOOL_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("kstool.exe");
    assert!(
        path.try_exists().unwrap(),
        "kstool.exe not found at expected path: {}",
        path.display()
    );
    path
});

static CACHE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = std::env::temp_dir().join("rust_const_asm_cache");
    std::fs::create_dir_all(&path).unwrap();
    path
});

fn inner(arch: &str, input: TokenStream) -> String {
    let insts = parse_input(input);

    let mut hasher = DefaultHasher::new();
    arch.hash(&mut hasher);
    insts.hash(&mut hasher);
    let hash = hasher.finish();
    let cache = CACHE_PATH.join(format!("{:x}.txt", hash));
    let result = if let Ok(result) = fs::read_to_string(&cache) {
        result
    } else {
        let result = kstool(arch, &insts)
            .into_iter()
            .map(|b| format!("{b:#04x}"))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = fs::write(&cache, &result);
        result
    };

    format!("\".byte {}\n\"", result)
}

fn kstool(arch: &str, insts: &str) -> Vec<u8> {
    let mut command = Command::new(&*KSTOOL_PATH);
    let command = command
        .env_clear()
        .arg("-b")
        .arg(arch)
        .arg(insts)
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .stdin(Stdio::null());

    let mut child = command
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to run kstool.exe at {:?}: {}", *KSTOOL_PATH, e));

    let status = match child.wait_timeout(Duration::from_secs(5)).unwrap() {
        Some(status) => status,
        None => {
            child.kill().unwrap();
            panic!("kstool.exe timed out after 5 seconds");
        }
    };

    let mut stdout = Vec::new();
    child.stdout.unwrap().read_to_end(&mut stdout).unwrap();
    let stdout_str = String::from_utf8_lossy(&stdout);

    if !status.success() || stdout.is_empty() || stdout_str.starts_with("ERROR:") {
        panic!(
            "kstool.exe did not execute successfully: {}",
            stdout_str
                .split("\nKstool v")
                .next()
                .unwrap_or("No error message")
        );
    }
    stdout
}

fn parse_input(input: TokenStream) -> String {
    let mut lines = Vec::new();
    let mut current = String::new();
    for token in input.into_iter() {
        match token {
            TokenTree::Literal(lit) => {
                let line = lit.to_string();
                current.push_str(line.trim_matches('"'));
            }
            TokenTree::Punct(punct) => match punct.as_char() {
                '+' | '-' | '*' | '/' => {
                    current.push(punct.as_char());
                }
                ',' => {
                    if !current.is_empty() {
                        lines.push(current.trim().to_string());
                        current.clear();
                    }
                }
                _ => panic!("Unexpected token in input: {:?}", punct),
            },
            TokenTree::Group(group) => match group.delimiter() {
                proc_macro::Delimiter::Parenthesis => {
                    let inner = parse_input(group.stream());
                    current.push('(');
                    current.push_str(&inner);
                    current.push(')');
                }
                proc_macro::Delimiter::Bracket => {
                    let inner = parse_input(group.stream());
                    current.push('[');
                    current.push_str(&inner);
                    current.push(']');
                }
                proc_macro::Delimiter::Brace => {
                    let inner = parse_input(group.stream());
                    current.push('{');
                    current.push_str(&inner);
                    current.push('}');
                }
                proc_macro::Delimiter::None => {
                    let inner = parse_input(group.stream());
                    current.push_str(&inner);
                }
            },
            _ => panic!("Unexpected token in input: {:?}", token),
        }
    }
    if !current.is_empty() {
        lines.push(current.trim().to_string());
    }

    lines
        .iter()
        .map(|inst| inst.split_whitespace().collect::<Vec<&str>>().join(" "))
        .collect::<Vec<String>>()
        .join(";")
}
