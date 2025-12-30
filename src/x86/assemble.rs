pub macro assemble {
    (@ "jmp ecx")                           => { ".byte 0xff, 0xe1" },
    (@ "call ecx")                          => { ".byte 0xff, 0xd1" },
    (@ "call $0")                           => { ".byte 0xe8, 0x00, 0x00, 0x00, 0x00" },
    (@ "ret")                               => { ".byte 0xc3" },
    (@ "retf")                              => { ".byte 0xcb" },

    (@ "push 0x33")                         => { ".byte 0x6a, 0x33" },

    (@ "mov ax, ds")                        => { ".byte 0x66, 0x8c, 0xd8" },
    (@ "mov ss, ax")                        => { ".byte 0x66, 0x8e, 0xd0" },
    (@ "mov eax, eax")                      => { ".byte 0x89, 0xc0" },
    (@ "mov esp, eax")                      => { ".byte 0x89, 0xc4" },
    (@ "mov esp, edx")                      => { ".byte 0x89, 0xd4" },
    (@ "mov rsp, rdx")                      => { ".byte 0x48, 0x89, 0xd4" },
    (@ "mov eax, fs:[0x30]")                => { ".byte 0x64, 0xA1, 0x30, 0x00, 0x00, 0x00" },
    (@ "mov eax, fs:[0x18]")                => { ".byte 0x64, 0xA1, 0x18, 0x00, 0x00, 0x00" },

    (@ "add dword [esp], 0x5")              => { ".byte 0x83, 0x04, 0x24, 0x05" },

    (@ "sub esp, 8")                          => { ".byte 0x83, 0xec, 0x08" },
    (@ "mov dword [esp], eax")                => { ".byte 0x89, 0x04, 0x24" },

    (@ "sub esp, 4")                           => { ".byte 0x83, 0xec, 0x04" },
    (@ "mov dword [esp], eax")                => { ".byte 0x89, 0x04, 0x24" },

    (@ "add ecx, 0x0")                  => { ".byte 0x83, 0xc1, 0x00" },
    (@ "add ecx, 0x4")                  => { ".byte 0x83, 0xc1, 0x04" },
    (@ "add ecx, 0x8")                  => { ".byte 0x83, 0xc1, 0x08" },
    (@ "add ecx, 0xc")                  => { ".byte 0x83, 0xc1, 0x0c" },
    (@ "add ecx, 0x10")                 => { ".byte 0x83, 0xc1, 0x10" },
    (@ "add ecx, 0x14")                 => { ".byte 0x83, 0xc1, 0x14" },
    (@ "add ecx, 0x18")                 => { ".byte 0x83, 0xc1, 0x18" },
    (@ "add ecx, 0x1c")                 => { ".byte 0x83, 0xc1, 0x1c" },
    (@ "add ecx, 0x20")                 => { ".byte 0x83, 0xc1, 0x20" },
    (@ "add ecx, 0x24")                 => { ".byte 0x83, 0xc1, 0x24" },
    (@ "add ecx, 0x28")                 => { ".byte 0x83, 0xc1, 0x28" },
    (@ "add ecx, 0x2c")                 => { ".byte 0x83, 0xc1, 0x2c" },
    (@ "add ecx, 0x30")                 => { ".byte 0x83, 0xc1, 0x30" },
    (@ "add ecx, 0x34")                 => { ".byte 0x83, 0xc1, 0x34" },

    (@ "mov dword [edx], eax")         => { ".byte 0x89, 0x02" },
    (@ "mov dword [edx], ecx")         => { ".byte 0x89, 0x0A" },
    (@ "mov dword [edx], edx")         => { ".byte 0x89, 0x12" },
    (@ "mov edx, dword [ecx]")         => { ".byte 0x8B, 0x11" },

    (@ "mov ax, cs")                        => { ".byte 0x66, 0x8c, 0xc8" },

    (@ $instr:tt) => {
        compile_error!( concat!("Unsupported instruction ", stringify!($instr)))
    },
    ( $( $instr:tt ),* $(,)? ) => {
        concat!( $( assemble!(@ $instr), "\n" ),* )
    },
}
