#![feature(test)]

#[macro_use]
extern crate sam;

#[cfg(test)]
mod tests {
    extern crate test;

    use self::test::Bencher;

    #[test]
    fn x64() {
        let asm = sam!(x64 =>
            "
            mov rax, 0x1337
            ret
        "
        );
        assert_eq!(asm, vec![0x48, 0xC7, 0xC0, 0x37, 0x13, 0x00, 0x00, 0xC3]);
    }

    #[test]
    fn x86() {
        let asm = sam!(x86 =>
            "
            mov eax, 0x1337
            ret
        "
        );
        assert_eq!(asm, vec![0xb8, 0x37, 0x13, 0x00, 0x00, 0xC3]);
    }

    #[bench]
    fn x64_bench(b: &mut Bencher) {
        b.iter(|| x64());
    }

    #[bench]
    fn x86_bench(b: &mut Bencher) {
        b.iter(|| x86());
    }
}
