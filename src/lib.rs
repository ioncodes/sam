extern crate keystone;

use keystone::*;

pub fn asm(e: &'static str, arch: Arch, mode: Mode) -> Vec<u8> {
    let engine = Keystone::new(arch, mode)
        .expect("Could not initialize Keystone engine");

    engine.option(OptionType::Syntax, OptionValue::SyntaxIntel)
        .expect("Could not set option to nasm syntax");

    let result = engine.asm(e, 0x0)
        .expect("Could not assemble");

    let mut buf = Vec::<u8>::new();
    for str_byte in result.to_string().split(" ") {
        buf.push(u8::from_str_radix(str_byte, 16).unwrap());
    }

    buf
}

#[macro_export]
macro_rules! sam {
    (x64 => $e:expr) => {{
        use keystone::{Arch, Mode};
        $crate::asm($e, Arch::X86, Mode::Bit64)
    }};

    (x86 => $e:expr) => {{
        use keystone::{Arch, Mode};
        $crate::asm($e, Arch::X86, Mode::Bit32)
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn x64() {
        let asm = sam!(x64 => "mov rax, 0x1337\nret");
        assert_eq!(asm, vec![0x48, 0xC7, 0xC0, 0x37, 0x13, 0x00, 0x00, 0xC3]);
    }

    #[test]
    fn x86() {
        let asm = sam!(x86 => "mov eax, 0x1337\nret");
        assert_eq!(asm, vec![0xb8, 0x37, 0x13, 0x00, 0x00, 0xc3]);
    }
}
