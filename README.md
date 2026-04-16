# XiangqiOS

A hobby operating system built around Chinese Chess (Xiangqi) — combining a from-scratch x86 OS kernel with a fully featured no_std Xiangqi game engine.

## Overview

XiangqiOS is split into two components that live in this repo:

- **OS Kernel** — a bare metal x86 operating system written in Rust, originally scaffolded from [Phil Opp's Writing an OS in Rust](https://os.phil-opp.com/) and now being rewritten with a custom bootloader from scratch
- **Xiangqi Engine** (`xiangqi-engine/`) — a no_std Rust game logic engine for Chinese Chess, designed to run natively on the OS

The long term goal is a standalone Xiangqi OS that boots into a playable game with puzzle mode, no host OS required.

---

## OS Kernel

> Primary development by [@kamui-fin](https://github.com/kamui-fin)

The kernel started from Phil Opp's blog OS series as a foundation and has since diverged significantly with a custom bootloader and expanding feature set.

### Current State

- Custom two-stage bootloader written in x86 Assembly and Rust (BIOS boot)
- VESA VBE framebuffer initialized and passed to kernel
- VGA text mode output
- GDT and IDT setup
- Hardware interrupt handling (PIC, PIT timer)
- Serial port output
- Memory paging and page table initialization
- Kernel heap allocation
- Basic kernel thread support and round-robin scheduler with preemption
- Lock guard pattern for synchronization
- Process loading via ELF loader
- `sbrk()` with argc/argv passed through stack
- System call interface
- PCB (Process Control Block) implementation in progress
- Compositor thread and userland graphics syscalls in progress

### Bootloader

The bootloader is written in a combination of x86 Assembly (stage 1) and Rust (stage 2), targeting BIOS boot. It handles unreal mode for loading large kernels, sets up the memory map, initializes VESA VBE framebuffer, and jumps to the kernel.

---

## Xiangqi Engine

> Primary development by [@l7ev](https://github.com/l7ev)

A no_std Xiangqi game logic engine designed to run in bare metal environments. Array-based board representation with complete game logic.

### Current State

- Full board representation with all piece types
- Complete move generation for all pieces:
  - General (palace restriction + opposition rule)
  - Advisor (diagonal palace movement)
  - Elephant (river restriction + blocking)
  - Horse (leg blocking)
  - Chariot
  - Cannon (platform capture logic)
  - Soldier (pre/post river movement)
- FEN string parsing and position loading
- Check detection
- Stalemate detection (loss in Xiangqi, not draw)
- General opposition enforcement
- Perpetual check/chase detection

### Planned

- no_std GUI rendering to framebuffer
- Puzzle mode with FEN-based position loading
- Move validation test suite
- Engine support (search + evaluation)

---

## Repository Structure

```
.
├── src/                  # OS kernel source
│   ├── main.rs
│   ├── interrupts.rs
│   ├── gdt.rs
│   ├── vga_buffer.rs
│   └── ...
├── tests/                # Kernel integration tests
├── xiangqi-engine/       # Xiangqi game logic engine (no_std)
│   └── src/
├── .cargo/
│   └── config.toml       # Target: x86_64 bare metal
├── x86_64-xiangqios.json # Custom target spec
└── Cargo.toml
```

---

## Building

Requires a nightly Rust toolchain and `bootimage`:

```bash
rustup override set nightly
cargo build
```

To run in QEMU:

```bash
cargo run
```

---

## Contributors

- [@kamui-fin](https://github.com/kamui-fin) — OS kernel, bootloader, scheduler, memory, syscalls
- [@l7ev](https://github.com/l7ev) — Xiangqi engine, graphics

---

## Acknowledgements

- [Phil Opp's Writing an OS in Rust](https://os.phil-opp.com/) — initial kernel scaffolding
- [Pikafish](https://pikafish.org/) — reference for Xiangqi engine concepts
