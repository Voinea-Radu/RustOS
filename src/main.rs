fn main() {
    let uefi_path = env!("UEFI_PATH");
    #[cfg(feature = "bios")]
    let bios_path = env!("BIOS_PATH");

    println!("{uefi_path}");
    println!("{:?}", ovmf_prebuilt::ovmf_pure_efi());

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-device").arg("isa-debug-exit,iobase=0xf4,iosize=0x04");
    cmd.arg("-serial").arg("stdio");

    #[cfg(feature = "uefi")]
    {
        println!("Starting in uefi mode");
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    }

    #[cfg(feature = "bios")]
    {
        println!("Starting in bios mode");
        cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }

    #[cfg(feature = "high-performance")]
    {
        println!("Starting in high performance mode");
        cmd.arg("-enable-kvm");
        cmd.arg("-cpu").arg("host");
    }

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
