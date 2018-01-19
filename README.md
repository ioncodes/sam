# sam
[![Foo](https://img.shields.io/crates/v/sam.svg)](https://crates.io/crates/sam)  

## What is this?
I'm currently working on many assembly related projects. Most of them are written in Rust.
Since inline assembly is ugly (atleast in my opinion), I work mostly with writable memory pages.
This means that I have to assemble my instructions to the proper architecture via a tool (for example [sam-web](https://github.com/ioncodes/sam-web)).
This work is really boring and repetetive, also it makes it hard to trace bugs in your assembly if you only have the opcodes laying in front of you.  
Therefore I decided to make a macro which utilizes [amber](https://github.com/ioncodes/amber) (a fast assembler and disassembler server) to create the opcodes out of a string containing your assembly.

**TLDR** This is a macro which converts an assembly string to a ```Vec<u8>``` at compile time.

## Current state of things
This project has just been released, amber too. Therefore, only x64 assembly is implemented. Usually, when amber receives an update, this repo should get one too.  
However, the project itself is stable.

## Prerequisites
* [amber](https://github.com/ioncodes/amber)

## Setup
Just start amber when you're using the plugin.

## Usage
You can use the plugin as follows:  
```rust
#![macro_use]
extern crate sam;

fn main() {
  let v = sam!("
    mov eax, 3
    mov ebx, 0
    mov ecx, 1337
    mov edx, 1
    int 0x80
  ");
  println!("{:?}", v);
}
```
