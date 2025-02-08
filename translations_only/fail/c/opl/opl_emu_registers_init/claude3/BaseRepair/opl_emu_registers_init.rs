

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start as u32) & ((1 << length) - 1)
}

fn opl_emu_abs_sin_attenuation(input: u32) -> u32 {
    static S_SIN_TABLE: [u16; 256] = [
        0x859, 0x6c3, 0x607, 0x58b, 0x52e, 0x4e4, 0x4a6, 0x471, 0x443, 0x41a, 0x3f5, 0x3d3, 0x3b5,
        0x398, 0x37e, 0x365, 0x34e, 0x339, 0x324, 0x311, 0x2ff, 0x2ed, 0x2dc, 0x2cd, 0x2bd, 0x2af,
        0x2a0, 0x293, 0x286, 0x279, 0x26d, 0x261, 0x256, 0x24b, 0x240, 0x236, 0x22c, 0x222, 0x218,
        0x20f, 0x206, 0x1fd, 0x1f5, 0x1ec, 0x1e4, 0x1dc, 0x1d4, 0x1cd, 0x1c5, 0x1be, 0x1b7, 0x1b0,
        0x1a9, 0x1a2, 0x19b, 0x195, 0x18f, 0x188, 0x182, 0x17c, 0x177, 0x171, 0x16b, 0x166, 0x160,
        0x15b, 0x155, 0x150, 0x14b, 0x146, 0x141, 0x13c, 0x137, 0x133, 0x12e, 0x129, 0x125, 0x121,
        0x11c, 0x118, 0x114, 0x10f, 0x10b, 0x107, 0x103, 0x0ff, 0x0fb, 0x0f8, 0x0f4, 0x0f0, 0x0ec,
        0x0e9, 0x0e5, 0x0e2, 0x0de, 0x0db, 0x0d7, 0x0d4, 0x0d1, 0x0cd, 0x0ca, 0x0c7, 0x0c4, 0x0c1,
        0x0be, 0x0bb, 0x0b8, 0x0b5, 0x0b2, 0x0af, 0x0ac, 0x0a9, 0x0a7, 0x0a4, 0x0a1, 0x09f, 0x09c,
        0x099, 0x097, 0x094, 0x092, 0x08f, 0x08d, 0x08a, 0x088, 0x086, 0x083, 0x081, 0x07f, 0x07d,
        0x07a, 0x078, 0x076, 0x074, 0x072, 0x070, 0x06e, 0x06c, 0x06a, 0x068, 0x066, 0x064, 0x062,
        0x060, 0x05e, 0x05c, 0x05b, 0x059, 0x057, 0x055, 0x053, 0x052, 0x050, 0x04e, 0x04d, 0x04b,
        0x04a, 0x048, 0x046, 0x045, 0x043, 0x042, 0x040, 0x03f, 0x03e, 0x03c, 0x03b, 0x039, 0x038,
        0x037, 0x035, 0x034, 0x033, 0x031, 0x030, 0x02f, 0x02e, 0x02d, 0x02b, 0x02a, 0x029, 0x028,
        0x027, 0x026, 0x025, 0x024, 0x023, 0x022, 0x021, 0x020, 0x01f, 0x01e, 0x01d, 0x01c, 0x01b,
        0x01a, 0x019, 0x018, 0x017, 0x017, 0x016, 0x015, 0x014, 0x014, 0x013, 0x012, 0x011, 0x011,
        0x010, 0x00f, 0x00f, 0x00e, 0x00d, 0x00d, 0x00c, 0x00c, 0x00b, 0x00a, 0x00a, 0x009, 0x009,
        0x008, 0x008, 0x007, 0x007, 0x007, 0x006, 0x006, 0x005, 0x005, 0x005, 0x004, 0x004, 0x004,
        0x003, 0x003, 0x003, 0x002, 0x002, 0x002, 0x002, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001,
        0x001, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000,
    ];

    let mut input = input;
    if opl_emu_bitfield(input, 8, 1) != 0 {
        input = !input;
    }

    S_SIN_TABLE[input as usize & 0xff] as u32
}

fn opl_emu_registers_init(regs: &mut OplEmuRegisters) {
    regs.m_lfo_am_counter = 0;
    regs.m_lfo_pm_counter = 0;
    regs.m_noise_lfsr = 1;
    regs.m_lfo_am = 0;

    let mut wf0 = Box::new([0u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]);
    let mut wf1 = Box::new([0u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]);
    let mut wf2 = Box::new([0u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]);
    let mut wf3 = Box::new([0u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]);
    let mut wf4 = Box::new([0u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]);
    let mut wf5 = Box::new([0u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]);
    let mut wf6 = Box::new([0u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]);
    let mut wf7 = Box::new([0u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]);

    for index in 0..OPL_EMU_REGISTERS_WAVEFORM_LENGTH {
        wf0[index] = (opl_emu_abs_sin_attenuation(index as u32)
            | ((opl_emu_bitfield(index as u32, 9, 1) << 15) as u32)) as u16;
    }

    if OPL_EMU_REGISTERS_WAVEFORMS >= 4 {
        let zeroval = wf0[0];
        for index in 0..OPL_EMU_REGISTERS_WAVEFORM_LENGTH {
            wf1[index] = if opl_emu_bitfield(index as u32, 9, 1) != 0 {
                zeroval
            } else {
                wf0[index]
            };
            wf2[index] = wf0[index] & 0x7fff;
            wf3[index] = if opl_emu_bitfield(index as u32, 8, 1) != 0 {
                zeroval
            } else {
                wf0[index] & 0x7fff
            };
            if OPL_EMU_REGISTERS_WAVEFORMS >= 8 {
                wf4[index] = if opl_emu_bitfield(index as u32, 9, 1) != 0 {
                    zeroval
                } else {
                    wf0[index * 2]
                };
                wf5[index] = if opl_emu_bitfield(index as u32, 9, 1) != 0 {
                    zeroval
                } else {
                    wf0[(index * 2) & 0x1ff]
                };
                wf6[index] = (opl_emu_bitfield(index as u32, 9, 1) << 15) as u16;
                wf7[index] = ((if opl_emu_bitfield(index as u32, 9, 1) != 0 {
                    (index ^ 0x13ff)
                } else {
                    index
                }) << 3) as u16;
            }
        }
    }

    regs.m_waveform = Box::new([
        mem::replace(&mut wf0, [0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]),
        mem::replace(&mut wf1, [0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]),
        mem::replace(&mut wf2, [0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]),
        mem::replace(&mut wf3, [0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]),
        mem::replace(&mut wf4, [0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]),
        mem::replace(&mut wf5, [0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]),
        mem::replace(&mut wf6, [0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]),
        mem::replace(&mut wf7, [0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]),
    ]);
    regs.m_regdata = Box::new([0; OPL_EMU_REGISTERS_REGISTERS]);
}

