extern crate keystone;

pub use keystone::*;
pub use keystone::gen::*;

#[macro_export]
macro_rules! sam {
    (x64 => $e:expr) => {{
        let engine = Keystone::new(KS_ARCH_X86, KS_MODE_64)
            .expect("Could not initialize Keystone engine");

        engine.option(KS_OPT_SYNTAX, KS_OPT_SYNTAX_INTEL)
            .expect("Could not set option to nasm syntax");

        let result = engine.asm($e, 0x0)
            .expect("Could not assemble");
        
        let mut buf = Vec::<u8>::new();
        for str_byte in result.to_string().split(" ") {
            buf.push(u8::from_str_radix(str_byte, 16).unwrap());
        }

        buf
    }};


    (x86 => $e:expr) => {{
        let engine = Keystone::new(KS_ARCH_X86, KS_MODE_32)
            .expect("Could not initialize Keystone engine");

        engine.option(KS_OPT_SYNTAX, KS_OPT_SYNTAX_INTEL)
            .expect("Could not set option to nasm syntax");

        let result = engine.asm($e, 0x0)
            .expect("Could not assemble");
        
        let mut buf = Vec::<u8>::new();
        for str_byte in result.to_string().split(" ") {
            buf.push(u8::from_str_radix(str_byte, 16).unwrap());
        }

        buf
    }}
}

#[cfg(test)]
mod tests {
    use keystone::*;
    use keystone::gen::*;

    #[test]
    fn x64() {
        let asm = sam!(x64 => "mov rax, 0x1337\nret");
        assert_eq!(asm, vec![0x48, 0xC7, 0xC0, 0x37, 0x13, 0x00, 0x00, 0xC3]);
    }

    #[test]
    fn x86() {
        let asm = sam!(x64 => "mov eax, 0x1337\nret");
        assert_eq!(asm, vec![0xb8, 0x37, 0x13, 0x00, 0x00, 0xc3]);
    }
}