

use std::mem;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

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
    eg_rate: [u8; OplEmuEnvelopeState::OplEmuEgStates as usize],
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
    m_regs: Box<OplEmuRegisters>,
}

fn opl_emu_fm_operator_start_release(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state <= OplEmuEnvelopeState::OplEmuEgRelease {
        fmop.m_env_state = OplEmuEnvelopeState::OplEmuEgRelease;
    }
}

fn opl_emu_fm_operator_start_attack(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state != OplEmuEnvelopeState::OplEmuEgAttack {
        fmop.m_env_state = OplEmuEnvelopeState::OplEmuEgAttack;
        fmop.m_phase = 0;
        if fmop.m_cache.eg_rate[OplEmuEnvelopeState::OplEmuEgAttack as usize] >= 62 {
            fmop.m_env_attenuation = 0;
        }
    }
}

fn opl_emu_fm_operator_clock_keystate(fmop: &mut OplEmuFmOperator, keystate: u32) {
    if (keystate ^ fmop.m_key_state as u32).wrapping_add(0) != 0 {
        fmop.m_key_state = keystate as u8;
        if keystate != 0 {
            opl_emu_fm_operator_start_attack(fmop);
        } else {
            opl_emu_fm_operator_start_release(fmop);
        }
    }
}


