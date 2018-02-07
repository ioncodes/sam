# sam
[![Foo](https://img.shields.io/crates/v/sam.svg)](https://crates.io/crates/sam) [![Build Status](https://travis-ci.org/ioncodes/sam.svg?branch=master)](https://travis-ci.org/ioncodes/sam) 

## What is this?
I'm currently working on many assembly related projects. Most of them are written in Rust.
Since inline assembly is ugly (atleast in my opinion), I work mostly with writable memory pages.
This means that I have to assemble my instructions to the proper architecture via a tool (for example [sam-web](https://github.com/ioncodes/sam-web)).
This work is really boring and repetetive, also it makes it hard to trace bugs in your assembly if you only have the opcodes laying in front of you.  
Therefore I decided to make a macro which utilizes kstool to create the opcodes out of a string containing your assembly. There will also be a full native solution as soon as I created the needed bindings for Keystone.

**TLDR** This is a macro which converts an assembly string to a ```Vec<u8>``` at compile time.

## Setup
If you're Windows user, you can just skip this part, because everything is done via Powershell.  
If you're on Mac OS X make sure Homebrew is installed!  
If you're on Linux you have to run ```cargo build``` as root user (```sudo```). To do that you have to use this command:  
```bash
sudo env "PATH=$PATH" cargo build
```

On Linux and Mac OS X, the script will install these dependencies:
* CMake
* Git

Keep in mind that you only need to do these steps if you're running it for the first time.  After successfully installing the dependencies the script will clone Keystone, build it and install it. On Windows, the script will just download prebuilt binaries.  

Depending on your OS you may also have to add ```/usr/local/lib``` to ```/etc/ld.so.conf``` and run ```sudo ldconfig```.

You can also do all of this manually, just make sure that ```kstool``` is available in your PATH!

## Modes
Supported modes are (all with NASM syntax):
```
x64
x86
```

## Example
```rust
#[macro_use]
extern crate sam;

fn main() {
  let asm: Vec<u8> = sam!(x64 => "mov rax, 0x1337");
  println!("{:?}", asm);
}
```
