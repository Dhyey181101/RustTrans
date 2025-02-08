
use std::ptr::null_mut;

#[derive(Clone, Copy)]
pub enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Clone, Copy)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 0x200],
    pub m_waveform: [[u16; 0x400]; 8],
}

#[derive(Clone, Copy)]
pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; 6],
    pub eg_shift: u8,
}

#[derive(Clone, Copy)]
pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: OplEmuEnvelopeState,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: OplEmuOpdataCache,
    pub m_regs: *mut OplEmuRegisters,
}

#[derive(Clone, Copy)]
pub struct OplEmuFmChannel {
    pub m_choffs: u32,
    pub m_feedback: [i16; 2],
    pub m_feedback_in: i16,
    pub m_op: [*mut OplEmuFmOperator; 4],
    pub m_regs: *mut OplEmuRegisters,
}

pub fn opl_emu_fm_channel_init(fmch: &mut OplEmuFmChannel, regs: &mut OplEmuRegisters, choffs: u32) {
    fmch.m_choffs = choffs;
    fmch.m_feedback[0] = 0;
    fmch.m_feedback[1] = 0;
    fmch.m_feedback_in = 0;
    fmch.m_op[0] = null_mut();
    fmch.m_op[1] = null_mut();
    fmch.m_op[2] = null_mut();
    fmch.m_op[3] = null_mut();
    fmch.m_regs = regs;
}
