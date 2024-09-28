fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    println!("{uefi_path}");
    println!("{:?}", ovmf_prebuilt::ovmf_pure_efi());

    let uefi = true;
    let high_performance = false;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }
    cmd.arg("-device").arg("isa-debug-exit,iobase=0xf4,iosize=0x04");
    cmd.arg("-serial").arg("stdio");

    if high_performance{
        cmd.arg("-enable-kvm");
        cmd.arg("-cpu").arg("host");
    }

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
