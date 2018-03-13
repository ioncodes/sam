# sam
[![Foo](https://img.shields.io/crates/v/sam.svg)](https://crates.io/crates/sam)  

## What is this?
I'm currently working on many assembly related projects. Most of them are written in Rust.
Since inline assembly is ugly (atleast in my opinion), I work mostly with writable memory pages.
This means that I have to assemble my instructions to the proper architecture via a tool (for example [sam-web](https://github.com/ioncodes/sam-web)).
This work is really boring and repetetive, also it makes it hard to trace bugs in your assembly if you only have the opcodes laying in front of you.  
Therefore I decided to make a macro which uses Keystone to assembly my assembly at compile time. 

**TLDR** This is a macro which converts an assembly string to a ```Vec<u8>``` at compile time.

## Credit
* [tathanhdinh](https://github.com/tathanhdinh) for [keystone-rs](https://github.com/tathanhdinh/keystone-rs)

## Don't know a title for this
This branch currently uses my fork of tathanhdinh's "keystone-rs" (see above). The fix was implemented in this [commit](https://github.com/ioncodes/sam/commit/fc9f8c607bc6b0e66faeb45444a5ae0a8adb150a). This dependency will change to keystone-rs as soon as it gets fixed. The issue can be found [here](https://github.com/tathanhdinh/keystone-rs/issues/1).

## Usage
You can use the plugin as follows:  
```rust
#[macro_use]
extern crate sam;

use keystone::*;
use keystone::gen::*;

fn main() {
  let asm = sam!(x64 => "mov rax, 0x1337\nret");
  assert_eq!(asm, vec![0x48, 0xC7, 0xC0, 0x37, 0x13, 0x00, 0x00, 0xC3]);
}
```
