

use std::boxed::Box;

#[derive(PartialEq)]
pub enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    pub m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],  
}

pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; OPL_EMU_EG_STATES],
    pub eg_shift: u8,
}

pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: OplEmuEnvelopeState,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: Box<OplEmuOpdataCache>,
    pub m_regs: Box<OplEmuRegisters>,
}

pub fn opl_emu_fm_operator_start_attack(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state == OplEmuEnvelopeState::OplEmuEgAttack {
        return;
    }

    fmop.m_env_state = OplEmuEnvelopeState::OplEmuEgAttack;
    fmop.m_phase = 0;

    if fmop.m_cache.eg_rate[0] >= 62 {
        fmop.m_env_attenuation = 0;
    }
}

pub const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
pub const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
pub const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;  
pub const OPL_EMU_EG_STATES: usize = 6;

