pub mod mode;

pub use std::process::Command;
pub use mode::Mode;

pub fn resolve_mode(mode: Mode) -> &'static str {
    match mode {
        Mode::X64 => "x64nasm",
        Mode::X86 => "x32nasm",
    }
}

pub fn assemble(mode: Mode, arg: &str) -> Vec<u8> {
    let mut asm = Vec::<u8>::new();
    let s_mode = resolve_mode(mode);
    for a in arg.lines() {
        if a.trim().is_empty() {
            continue;
        }
        let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/C", "kstool", s_mode, &a])
                    .output()
                    .expect("failed to execute process")
            } else {
                Command::new("kstool")
                    .args(&[s_mode, &a])
                    .output()
                    .expect("failed to execute process")
        };
        let result = &String::from_utf8(output.stdout).unwrap();
        let mut temp: Vec<&str> = result.split("[ ").collect();
        temp = temp[1].split(" ]").collect();
        let out = temp[0];
        if out.contains(" ") {
            for opcode in out.split(" ") {
                asm.push(u8::from_str_radix(&opcode, 16).unwrap());
            }
        } else {
            asm.push(u8::from_str_radix(&out, 16).unwrap());
        }
    }
    asm
}

#[macro_export]
macro_rules! sam {
    (x64 => $e:expr) => {{
        $crate::assemble($crate::Mode::X64, $e)
    }};
    (x86 => $e:expr) => {{
        $crate::assemble($crate::Mode::X86, $e)
    }};
}
