use std::env::consts::ARCH;


/// This enum represents the different CPU types that are supported by the `wedp` tool.
/// 
/// # Fields
/// * `X86` - The x86 CPU type
/// * `X86_64` - The x86_64 CPU type
/// * `Arm` - The arm CPU type
/// * `Aarch64` - The aarch64 CPU type
/// * `M68k` - The m68k CPU type
/// * `Mips` - The mips CPU type
/// * `Mips64` - The mips64 CPU type
/// * `Powerpc` - The powerpc CPU type
/// * `Powerpc64` - The powerpc64 CPU type
/// * `Riscv64` - The riscv64 CPU type
/// * `S390x` - The s390x CPU type
/// * `Sparc64` - The sparc64 CPU type
pub enum CpuType {
    X86,
    X86_64,
    Arm,
    Aarch64,
    M68k,
    Mips,
    Mips64,
    Powerpc,
    Powerpc64,
    Riscv64,
    S390x,
    Sparc64,
}

impl CpuType {

    /// Get the current CPU type
    ///
    /// # Returns
    /// * The current CPU type
    pub fn get() -> Self {
        match ARCH {
            "x86" => CpuType::X86,
            "x86_64" => CpuType::X86_64,
            "arm" => CpuType::Arm,
            "aarch64" => CpuType::Aarch64,
            "m68k" => CpuType::M68k,
            "mips" => CpuType::Mips,
            "mips64" => CpuType::Mips64,
            "powerpc" => CpuType::Powerpc,
            "powerpc64" => CpuType::Powerpc64,
            "riscv64" => CpuType::Riscv64,
            "s390x" => CpuType::S390x,
            "sparc64" => CpuType::Sparc64,
            _ => panic!("Unsupported CPU type: {}", ARCH)
        }
    }

    /// Convert the CPU type to a string.
    ///
    /// # Returns
    /// * The string representation of the CPU type
    pub fn to_string(self) -> String {
        match self {
            CpuType::X86 => "x86".to_string(),
            CpuType::X86_64 => "x86_64".to_string(),
            CpuType::Arm => "arm".to_string(),
            CpuType::Aarch64 => "aarch64".to_string(),
            CpuType::M68k => "m68k".to_string(),
            CpuType::Mips => "mips".to_string(),
            CpuType::Mips64 => "mips64".to_string(),
            CpuType::Powerpc => "powerpc".to_string(),
            CpuType::Powerpc64 => "powerpc64".to_string(),
            CpuType::Riscv64 => "riscv64".to_string(),
            CpuType::S390x => "s390x".to_string(),
            CpuType::Sparc64 => "sparc64".to_string(),
        }
    }
}