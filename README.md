# asm-snippet

Disassemble x86 / x64 / x16 byte strings into Intel-syntax instructions. Built on [iced-x86](https://github.com/icedland/iced).

Pairs naturally with [`pe-info`](https://github.com/AuDowty/pe-info) and [`pdb-info`](https://github.com/AuDowty/pdb-info): grab bytes from a section dump, paste them in, see what they do.

## Install

```
cargo install --git https://github.com/AuDowty/asm-snippet
```

## Use

```
asm-snippet "48 89 ce 48 8b 3e"
# 0x00000000  48 89 ce                  mov     rsi,rcx
# 0x00000003  48 8b 3e                  mov     rdi,[rsi]

asm-snippet 4889ce488b3e --base 0x1000
# 0x00001000  48 89 ce                  mov     rsi,rcx
# 0x00001003  48 8b 3e                  mov     rdi,[rsi]

asm-snippet "55 89 e5 90" --bits 32

cat shellcode.bin | asm-snippet - --binary

asm-snippet 4889ce488b3e --raw     # bare instruction text, no addr/bytes
```

Hex input is whitespace- and separator-tolerant: `48 89 ce`, `4889ce`, `\x48\x89\xce`, `0x48,0x89,0xce` all parse identically.

## Status

v0.1 is **disassembly only**. Assembly (Intel-syntax string → bytes) is planned but blocked on iced-x86's lack of a string parser — wiring up Keystone or NASM is out of v1 scope.

## License

MIT.
