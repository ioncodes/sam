extern crate keystone;

pub use keystone::*;

#[macro_export]
macro_rules! sam {
    ($e:expr) => {{
        let engine = super::Keystone::new(super::Arch::X86, super::keystone::MODE_64)
            .expect("Could not initialize Keystone engine");

        engine.option(super::OptionType::SYNTAX, super::keystone::OPT_SYNTAX_NASM)
            .expect("Could not set option to nasm syntax");

        let result = engine.asm($e, 0)
            .expect("Could not assemble");

            println!("{:?}", result);
        
        // let mut asm = Vec::<u8>::new();
        // for opcode in result.split(",") {
        //     asm.push(u8::from_str_radix(&opcode[2..], 16).unwrap());
        // }
        // asm
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn x64() {
        let asm = sam!("mov rax, 0x1337\nret".to_string());
        assert_eq!(vec![0x48, 0xC7, 0xC0, 0x37, 0x13, 0x00, 0x00, 0xC3], vec![0x48, 0xC7, 0xC0, 0x37, 0x13, 0x00, 0x00, 0xC3]);
    }
}