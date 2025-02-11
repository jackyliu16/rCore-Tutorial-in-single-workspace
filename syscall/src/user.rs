﻿use crate::{ClockId, SyscallId, TimeSpec};
use native::*;

/// see <https://man7.org/linux/man-pages/man2/write.2.html>.
#[inline]
pub fn write(fd: usize, buffer: &[u8]) -> isize {
    unsafe { syscall3(SyscallId::WRITE, fd, buffer.as_ptr() as _, buffer.len()) }
}

/// see <https://man7.org/linux/man-pages/man2/exit.2.html>.
#[inline]
pub fn exit(exit_code: i32) -> isize {
    unsafe { syscall1(SyscallId::EXIT, exit_code as _) }
}

/// see <https://man7.org/linux/man-pages/man2/sched_yield.2.html>.
#[inline]
pub fn sched_yield() -> isize {
    unsafe { syscall0(SyscallId::SCHED_YIELD) }
}

/// see <https://man7.org/linux/man-pages/man2/clock_gettime.2.html>.
#[inline]
pub fn clock_gettime(clockid: ClockId, tp: *mut TimeSpec) -> isize {
    unsafe { syscall2(SyscallId::CLOCK_GETTIME, clockid.0, tp as _) }
}

/// 这个模块包含调用系统调用的最小封装，用户可以直接使用这些函数调用自定义的系统调用。
pub mod native {
    use crate::SyscallId;
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn syscall0(id: SyscallId) -> isize {
        let ret: isize;
        asm!("ecall",
            in("a7") id.0,
            out("a0") ret,
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(id: SyscallId, a0: usize) -> isize {
        let ret: isize;
        asm!("ecall",
            inlateout("a0") a0 => ret,
            in("a7") id.0,
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall2(id: SyscallId, a0: usize, a1: usize) -> isize {
        let ret: isize;
        asm!("ecall",
            in("a7") id.0,
            inlateout("a0") a0 => ret,
            in("a1") a1,
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall3(id: SyscallId, a0: usize, a1: usize, a2: usize) -> isize {
        let ret: isize;
        asm!("ecall",
            in("a7") id.0,
            inlateout("a0") a0 => ret,
            in("a1") a1,
            in("a2") a2,
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall4(id: SyscallId, a0: usize, a1: usize, a2: usize, a3: usize) -> isize {
        let ret: isize;
        asm!("ecall",
            in("a7") id.0,
            inlateout("a0") a0 => ret,
            in("a1") a1,
            in("a2") a2,
            in("a3") a3,
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall5(
        id: SyscallId,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
    ) -> isize {
        let ret: isize;
        asm!("ecall",
            in("a7") id.0,
            inlateout("a0") a0 => ret,
            in("a1") a1,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall6(
        id: SyscallId,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> isize {
        let ret: isize;
        asm!("ecall",
            in("a7") id.0,
            inlateout("a0") a0 => ret,
            in("a1") a1,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
        );
        ret
    }
}
