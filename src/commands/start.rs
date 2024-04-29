use crate::runtime::crun::*;
use anyhow::{ensure, Result};
use std::ffi::OsStr;
use std::process::Command;

pub fn start(args: &liboci_cli::Start, raw_args: &[impl AsRef<OsStr>]) -> Result<()> {
    // auto detect arch
    let arch_cmd = if cfg!(target_arch = "x86_64") {
        "qemu-system-x86_64"
    } else {
        "qemu-system-arm64"
    };

    let arg = vec![
        "-name",
        &args.container_id,
        "-machine",
        "q35",
        "-device",
        "pcie-root-port,port=0x10,chassis=1,id=pci.1,bus=pcie.0,multifunction=on,addr=0x3",
        "-device",
        "pcie-root-port,port=0x11,chassis=2,id=pci.2,bus=pcie.0,addr=0x3.0x1",
        "-device",
        "pcie-root-port,port=0x12,chassis=3,id=pci.3,bus=pcie.0,addr=0x3.0x2",
        "-device",
        "virtio-scsi-pci,bus=pci.2,addr=0x0,id=scsi0",
        "-device",
        "scsi-hd,bus=scsi0.0,drive=hd0",
        "-vga",
        "none",
        "-device",
        "isa-debug-exit",
        "-m",
        "512M",
        "-device",
        "virtio-rng-pci",
        "-machine",
        "accel=kvm:tcg",
        "-cpu",
        "host",
        "-no-reboot",
        "-cpu",
        "max",
        "-drive",
        "file=rootfs/image,format=raw,if=none,id=hd0",
        "-device",
        "virtio-net,bus=pci.3,addr=0x0,netdev=n0,mac=de:09:ec:88:42:a2",
        "-netdev",
        "user,id=n0,hostfwd=tcp::6379-:6379",
        "-display",
        "none",
        "-serial",
        "mon:stdio",
    ];
    let child = Command::new(arch_cmd).args(arg).spawn()?;
    println!("pid: {:?}", child.id());
    if child.stderr.is_some() {
        println!("stderr: {:?}", child.stderr);
    }
    crun(raw_args)?;
    Ok(())
}
