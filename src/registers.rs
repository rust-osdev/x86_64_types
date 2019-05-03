use bitflags::bitflags;

bitflags! {
    /// Configuration flags of the Cr0 register.
    #[derive(Default)]
    pub struct Cr0: u64 {
        /// Enables protected mode.
        const PROTECTED_MODE_ENABLE = 1 << 0;
        /// Enables monitoring of the coprocessor, typical for x87 instructions.
        ///
        /// Controls together with the `TASK_SWITCHED` flag whether a `wait` or `fwait`
        /// instruction should cause a device-not-available exception.
        const MONITOR_COPROCESSOR = 1 << 1;
        /// Force all x87 and MMX instructions to cause an exception.
        const EMULATE_COPROCESSOR = 1 << 2;
        /// Automatically set to 1 on _hardware_ task switch.
        ///
        /// This flags allows lazily saving x87/MMX/SSE instructions on hardware context switches.
        const TASK_SWITCHED = 1 << 3;
        /// Math coprocessor is 80287 (disabled) or 80387 (enabled).
        const EXTENSION_TYPE = 1 << 4;
        /// Enables the native error reporting mechanism for x87 FPU errors.
        const NUMERIC_ERROR = 1 << 5;
        /// Controls whether supervisor-level writes to read-only pages are inhibited.
        ///
        /// When set, it is not possible to write to read-only pages from ring 0.
        const WRITE_PROTECT = 1 << 16;
        /// Enables automatic alignment checking.
        const ALIGNMENT_MASK = 1 << 18;
        /// Ignored. Used to control write-back/write-through cache strategy on older CPUs.
        const NOT_WRITE_THROUGH = 1 << 29;
        /// Disables internal caches (only for some cases).
        const CACHE_DISABLE = 1 << 30;
        /// Enables page translation.
        const PAGING = 1 << 31;
    }
}

bitflags! {
    /// Controls cache settings for the level 4 page table.
    #[derive(Default)]
    pub struct Cr3: u64 {
        /// Use a writethrough cache policy for the P4 table (else a writeback policy is used).
        const PAGE_LEVEL_WRITETHROUGH = 1 << 3;
        /// Disable caching for the P4 table.
        const PAGE_LEVEL_CACHE_DISABLE = 1 << 4;
    }
}

bitflags! {
    /// Flags of the Extended Feature Enable Register.
    #[derive(Default)]
    pub struct Efer: u64 {
        /// Enables the `syscall` and `sysret` instructions.
        const SYSTEM_CALL_EXTENSIONS = 1 << 0;
        /// Activates long mode, requires activating paging.
        const LONG_MODE_ENABLE = 1 << 8;
        /// Indicates that long mode is active.
        const LONG_MODE_ACTIVE = 1 << 10;
        /// Enables the no-execute page-protection feature.
        const NO_EXECUTE_ENABLE = 1 << 11;
        /// Enables SVM extensions.
        const SECURE_VIRTUAL_MACHINE_ENABLE = 1 << 12;
        /// Enable certain limit checks in 64-bit mode.
        const LONG_MODE_SEGMENT_LIMIT_ENABLE = 1 << 13;
        /// Enable the `fxsave` and `fxrstor` instructions to execute faster in 64-bit mode.
        const FAST_FXSAVE_FXRSTOR = 1 << 14;
        /// Changes how the `invlpg` instruction operates on TLB entries of upper-level entries.
        const TRANSLATION_CACHE_EXTENSION = 1 << 15;
    }
}

bitflags! {
    /// The RFLAGS register.
    pub struct RFlags: u64 {
        /// Processor feature identification flag.
        ///
        /// If this flag is modifiable, the CPU supports CPUID.
        const ID = 1 << 21;
        /// Indicates that an external, maskable interrupt is pending.
        ///
        /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
        /// interrupts (CR4.PVI) are activated.
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;
        /// Virtual image of the INTERRUPT_FLAG bit.
        ///
        /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
        /// interrupts (CR4.PVI) are activated.
        const VIRTUAL_INTERRUPT = 1 << 19;
        /// Enable automatic alignment checking if CR0.AM is set. Only works if CPL is 3.
        const ALIGNMENT_CHECK = 1 << 18;
        /// Enable the virtual-8086 mode.
        const VIRTUAL_8086_MODE = 1 << 17;
        /// Allows to restart an instruction following an instrucion breakpoint.
        const RESUME_FLAG = 1 << 16;
        /// Used by `iret` in hardware task switch mode to determine if current task is nested.
        const NESTED_TASK = 1 << 14;
        /// Set by hardware to indicate that the sign bit of the result of the last signed integer
        /// operation differs from the source operands.
        const OVERFLOW_FLAG = 1 << 11;
        /// Determines the order in which strings are processed.
        const DIRECTION_FLAG = 1 << 10;
        /// Enable interrupts.
        const INTERRUPT_FLAG = 1 << 9;
        /// Enable single-step mode for debugging.
        const TRAP_FLAG = 1 << 8;
        /// Set by hardware if last arithmetic operation resulted in a negative value.
        const SIGN_FLAG = 1 << 7;
        /// Set by hardware if last arithmetic operation resulted in a zero value.
        const ZERO_FLAG = 1 << 6;
        /// Set by hardware if last arithmetic operation generated a carry ouf of bit 3 of the
        /// result.
        const AUXILIARY_CARRY_FLAG = 1 << 4;
        /// Set by hardware if last result has an even number of 1 bits (only for some operations).
        const PARITY_FLAG = 1 << 2;
        /// Set by hardware if last arithmetic operation generated a carry out of the
        /// most-significant bit of the result.
        const CARRY_FLAG = 1 << 0;
    }
}

impl RFlags {
    /// Get the I/O Privilege Level (0-3, inclusive)
    pub fn iopl(self) -> u8 {
        (self.bits >> 12) as u8 & 0b11
    }

    /// Set the I/O Privilege Level (0-3, inclusive)
    ///
    /// # Panics
    ///
    /// Panics on an invalid privilege level (>= 4).
    pub fn set_iopl(&mut self, level: u8) {
        assert!(level <= 0b11);
        self.bits &= !(0b11 << 12);
        self.bits |= (level as u64) << 12;
    }
}

impl Default for RFlags {
    fn default() -> Self {
        Self {
            bits: 1 << 1 // Bit 1 is always on in EFlags and RFlags
        }
    }
}
