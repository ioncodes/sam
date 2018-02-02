#[macro_export]
macro_rules! sam {
    ($e:expr) => {
        {
            let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(&["/C", "kstool", "x64nasm", $e])
                        .output()
                        .expect("failed to execute process")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg("kstool")
                        .arg("x64nasm")
                        .arg($e)
                        .output()
                        .expect("failed to execute process")
            };
            let result = &String::from_utf8(output.stdout).unwrap();
            let out = result.split("[")[1].split("]")[0].replace(" ", "");
            let mut asm = Vec::<u8>::new();
            for opcode in out.split(",") {
                asm.push(u8::from_str_radix(&opcode[2..], 16).unwrap());
            }
            asm
        }
    }
}

