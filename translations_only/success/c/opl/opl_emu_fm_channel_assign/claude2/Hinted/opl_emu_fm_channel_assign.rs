
use std::boxed::Box;
use std::convert::TryInto;

#[derive(Clone)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

#[derive(Clone)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,   
}

#[derive(Clone)]
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

#[derive(Clone)]
struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: Box<OplEmuOpdataCache>,
    m_regs: Box<OplEmuRegisters>,
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
    m_regs: Box<OplEmuRegisters>,   
}

fn opl_emu_fm_operator_set_choffs(fmop: &mut OplEmuFmOperator, choffs: u32) {
    fmop.m_choffs = choffs;
}

fn opl_emu_fm_channel_assign(fmch: &mut OplEmuFmChannel, index: u32, op: Option<Box<OplEmuFmOperator>>) {
    fmch.m_op[index as usize] = op;
}

