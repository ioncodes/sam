use std::process::Command;

#[macro_export]
macro_rules! sam {
    ($e:expr) => {
        {
            let mut asm = Vec::<u8>::new();
            let arg = $e;
            for a in arg.lines() {
                if a.trim().is_empty() {
                    continue;
                }
                println!("{:?}", a);
                let output = if cfg!(target_os = "windows") {
                        super::Command::new("cmd")
                            .args(&["/C", "kstool", "x64nasm", &a])
                            .output()
                            .expect("failed to execute process")
                    } else {
                        super::Command::new("sh")
                            .arg("-c")
                            .arg("kstool")
                            .arg("x64nasm")
                            .arg(&a)
                            .output()
                            .expect("failed to execute process")
                };
                let result = &String::from_utf8(output.stdout).unwrap();
                println!("{:?}", result);
                let mut temp: Vec<&str> = result.split("[ ").collect();
                temp = temp[1].split(" ]").collect();
                let out = temp[0];
                for opcode in out.split(" ") {
                    asm.push(u8::from_str_radix(&opcode, 16).unwrap());
                }
            }
            asm
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn x64() {
        let asm = sam!("
            mov rax, 0x1337
            ret
        ");
        assert_eq!(asm, vec![0x48, 0xC7, 0xC0, 0x37, 0x13, 0x00, 0x00, 0xC3]);
    }
}