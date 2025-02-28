
use std::boxed::Box;

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; 0x200]>,
    m_waveform: Box<[[u16; 0x400]; 8]>,
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: Box<[u8; 6]>,
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

impl OplEmuFmOperator {
    fn new(regs: Box<OplEmuRegisters>, opoffs: u32) -> Self {
        OplEmuFmOperator {
            m_choffs: 0,
            m_opoffs: opoffs,
            m_phase: 0,
            m_env_attenuation: 0x3ff,
            m_env_state: OplEmuEnvelopeState::OplEmuEgRelease,
            m_key_state: 0,
            m_keyon_live: 0,
            m_cache: OplEmuOpdataCache {
                phase_step: 0,
                total_level: 0,
                block_freq: 0,
                detune: 0,
                multiple: 0,
                eg_sustain: 0,
                eg_rate: Box::new([0; 6]),
                eg_shift: 0,
            },
            m_regs: regs,
        }
    }
}
