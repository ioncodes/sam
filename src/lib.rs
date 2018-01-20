extern crate reqwest;

#[macro_export]
macro_rules! sam {
    ($e:expr) => {
        use reqwest;
        {
            let client = reqwest::Client::new();
            let mut res = client.post("http://localhost:1337/asm")
                .body($e)
                .send()
                .unwrap();
            let body = res.text().unwrap();
            let mut asm = Vec::<u8>::new();
            for opcode in body.split(",") {
                asm.push(u8::from_str_radix(&opcode[2..], 16).unwrap());
            }
            asm
        }
    }
}
