#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::{bpf_get_current_task, bpf_probe_read_kernel},
    macros::kprobe,
    programs::ProbeContext,
};
use aya_log_ebpf::info;

use vmlinux::task_struct;

mod vmlinux;

#[kprobe]
pub fn aya_co_re_issue(ctx: ProbeContext) -> u32 {
    match unsafe { try_aya_co_re_issue(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_aya_co_re_issue(ctx: ProbeContext) -> Result<u32, u32> {
    let task = bpf_get_current_task() as *const task_struct;
    let parent = bpf_probe_read_kernel(&(*task).real_parent).map_err(|x| x as u32)?;
    let ppid = bpf_probe_read_kernel(&(*parent).tgid).map_err(|x| x as u32)? as u32;
    info!(&ctx, "My parent: {}", ppid);
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
