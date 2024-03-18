use anyhow::Result;
use kvm_bindings::kvm_userspace_memory_region;
use kvm_ioctls::{Kvm, VcpuExit};
use std::io::Write;
use vm_memory::mmap::{GuestMemoryMmap, GuestRegionMmap};
use vm_memory::{
    Bytes, GuestAddress, GuestMemory, GuestMemoryRegion, MemoryRegionAddress, MmapRegion,
};

fn main() -> Result<()> {
    let kvm = Kvm::new()?;

    let vm = kvm.create_vm()?;

    let mem: GuestRegionMmap =
        GuestRegionMmap::new(MmapRegion::new(0x1000)?, GuestAddress(0x1000)).unwrap();

    mem.write_slice(
        &[
            0xba, 0xf8, 0x03, // mov $0x3f8, %dx
            0x00, 0xd8, // add %b1, %al
            0x04, b'0', // add $'0', $al
            0xee, // out %al, %dx
            0xf4, // hlt
        ],
        MemoryRegionAddress(0),
    )?;

    unsafe {
        vm.set_user_memory_region(kvm_userspace_memory_region {
            slot: 0,
            guest_phys_addr: mem.start_addr().0,
            memory_size: mem.len() as u64,
            userspace_addr: mem.as_ptr() as u64,
            flags: 0,
        })?;
    }

    let vcpu = vm.create_vcpu(0)?;

    let mut sregs = vcpu.get_sregs()?;
    sregs.cs.base = 0;
    sregs.cs.selector = 0;
    vcpu.set_sregs(&sregs)?;

    let mut regs = vcpu.get_regs()?;
    regs.rip = mem.start_addr().0;
    regs.rax = 2;
    regs.rbx = 2;
    regs.rflags = 2;
    vcpu.set_regs(&regs)?;

    loop {
        match vcpu.run()? {
            VcpuExit::IoOut(0x3f8, data) => std::io::stdout().write_all(data)?,
            VcpuExit::Hlt => break,
            e => panic!("Unknown exit reason: {:?}", e),
        }
    }

    Ok(())
}
