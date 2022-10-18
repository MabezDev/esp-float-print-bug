## esp float print bug

```
cargo run --release
``` 

Change release mode opt levels inside `Cargo.toml`

Error output:

```rust
I (153) boot: Disabling RNG early entropy source...
16.0f64 literal is=16
16.0f32 literal is= 
 
!! A panic occured in '/home/mabez/development/rust/xtensa/rust-xtensa-dev/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust/library/core/src/num/flt2dec/mod.rs', at line 189, column 5
 
PanicInfo {
    payload: Any { .. },
    message: Some(
        assertion failed: buf[0] > b\'0\',
    ),
    location: Location {
        file: "/home/mabez/development/rust/xtensa/rust-xtensa-dev/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust/library/core/src/num/flt2dec/mod.rs",
        line: 189,
        col: 5,
    },
    can_unwind: true,
}
 
Backtrace:
 
0x4008652e
0x4008652e - core::num::flt2dec::digits_to_dec_str
    at /home/mabez/development/rust/xtensa/rust-xtensa-dev/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust/library/core/src/num/flt2dec/mod.rs:??
0x40086974
0x40086974 - core::num::flt2dec::to_shortest_str
    at /home/mabez/development/rust/xtensa/rust-xtensa-dev/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust/library/core/src/num/flt2dec/mod.rs:398
0x40086186
0x40086186 - core::fmt::float::float_to_decimal_common_shortest
    at /home/mabez/development/rust/xtensa/rust-xtensa-dev/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust/library/core/src/fmt/float.rs:??
0x400863a5
0x400863a5 - core::fmt::float::float_to_decimal_display
    at /home/mabez/development/rust/xtensa/rust-xtensa-dev/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust/library/core/src/fmt/float.rs:??
0x40082823
0x40082823 - core::fmt::write
    at /home/mabez/development/rust/xtensa/rust-xtensa-dev/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:1198
0x400808a1
0x400808a1 - core::fmt::Write::write_fmt
    at /home/mabez/development/rust/xtensa/rust-xtensa-dev/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:??
0x40081500
0x40081500 - Reset
    at /home/mabez/.cargo/registry/src/github.com-1ecc6299db9ec823/xtensa-lx-rt-0.13.0/src/lib.rs:77
0x40080ab4
0x40080ab4 - ESP32Reset
    at /home/mabez/.cargo/registry/src/github.com-1ecc6299db9ec823/esp32s2-hal-0.2.0/src/lib.rs:78
0x40000000
0x40000000 - _stack_end_cpu0
    at ??:??
0x40051588
0x40051588 - _text_heap_end
    at ??:??
```