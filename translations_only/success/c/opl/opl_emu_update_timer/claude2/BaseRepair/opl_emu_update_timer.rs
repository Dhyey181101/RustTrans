
use std::boxed::Box;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;

const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;

const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,  
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 4],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,  
}

struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: Box<OplEmuRegisters>,
    m_channel: [Box<OplEmuFmChannel>; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [Box<OplEmuFmOperator>; OPL_EMU_REGISTERS_OPERATORS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize] as u32, start as i32, count as i32)  
}

fn opl_emu_registers_timer_b_value(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(&regs, 0x03, 0, 8)
}

fn opl_emu_registers_timer_a_value(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(&regs, 0x02, 0, 8) * 4  
}

fn opl_emu_update_timer(emu: &mut OplEmu, tnum: u32, enable: u32) {
    // implementation
}

