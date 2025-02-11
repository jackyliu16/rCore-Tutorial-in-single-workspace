#![no_std]
#![no_main]
#![feature(naked_functions, asm_sym, asm_const)]
#![deny(warnings)]

use output::*;
use sbi_rt::*;

/// Supervisor 汇编入口。
///
/// 设置栈并跳转到 Rust。
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    const STACK_SIZE: usize = 4096;

    #[link_section = ".bss.uninit"]
    static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

    core::arch::asm!(
        "   la  sp, {stack}
            li  t0, {stack_size}
            add sp, sp, t0
            j   {main}
        ",
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym rust_main,
        options(noreturn),
    )
}

/// 将传给 `output` 的控制台对象。
///
/// 这是一个 Unit struct，它不需要空间。否则需要传一个 static 对象。
struct Console;

/// 为 `Console` 实现 `output::Console` trait。
impl output::Console for Console {
    fn put_char(&self, c: u8) {
        #[allow(deprecated)]
        legacy::console_putchar(c as _);
    }
}

/// 使用 `output` 输出的 Supervisor 裸机程序。
///
/// 测试各种日志和输出后关机。
extern "C" fn rust_main() -> ! {
    // 初始化 console
    init_console(&Console);
    // 设置总的日志级别
    log::set_max_level(log::LevelFilter::Trace);

    println!("[PRINT] Hello, world!");
    log::trace!("Hello, world!");
    log::debug!("Hello, world!");
    log::info!("Hello, world!");
    log::warn!("Hello, world!");
    log::error!("Hello, world!");

    system_reset(RESET_TYPE_SHUTDOWN, RESET_REASON_NO_REASON);
    unreachable!()
}

/// Rust 异常处理函数，以异常方式关机。
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    system_reset(RESET_TYPE_SHUTDOWN, RESET_REASON_SYSTEM_FAILURE);
    unreachable!()
}
