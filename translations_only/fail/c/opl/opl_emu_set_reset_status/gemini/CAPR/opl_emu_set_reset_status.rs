

use std::ops::{Add, BitAnd, BitOr, Shl, Shr};

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

struct OplEmuRegisters {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 6],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: OplEmuEnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OplEmuOpdataCache,
}

struct OplEmuFmChannel {
    choffs: u32,
    feedback: [i16; 2],
    feedback_in: i16,
}

struct OplEmu {
    env_counter: u32,
    status: u8,
    timer_running: [u8; 2],
    active_channels: u32,
    modified_channels: u32,
    prepare_count: u32,
    regs: OplEmuRegisters,
    channel: [OplEmuFmChannel; 18],
    operator: [OplEmuFmOperator; 36],
}

const OPL_EMU_REGISTERS_STATUS_BUSY: u8 = 0x80;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(regs.regdata[offset as usize + extra_offset as usize] as u32, start as i32, count as i32)
}

fn opl_emu_registers_status_mask(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x04, 0, 8, 0) & 0x78
}

fn opl_emu_set_reset_status(emu: &mut OplEmu, set: u8, reset: u8) -> u8 {
    emu.status = (emu.status | set) & !(reset | OPL_EMU_REGISTERS_STATUS_BUSY);
    emu.status & !opl_emu_registers_status_mask(&emu.regs) as u8
}

