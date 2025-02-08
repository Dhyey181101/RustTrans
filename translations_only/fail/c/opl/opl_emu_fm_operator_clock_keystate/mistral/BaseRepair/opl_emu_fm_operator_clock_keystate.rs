

use std::mem;
use std::ops::BitOr;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

#[repr(C)]
struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OplEmuEnvelopeState::States as usize],
    eg_shift: u8,
}

#[repr(C)]
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
    fn start_release(&mut self) {
        if self.m_env_state >= OplEmuEnvelopeState::Release {
            return;
        }
        self.m_env_state = OplEmuEnvelopeState::Release;
    }

    fn start_attack(&mut self) {
        if self.m_env_state == OplEmuEnvelopeState::Attack {
            return;
        }
        self.m_env_state = OplEmuEnvelopeState::Attack;
        self.m_phase = 0;

        if self.m_cache.eg_rate[OplEmuEnvelopeState::Attack as usize] >= 62 {
            self.m_env_attenuation = 0;
        }
    }

    fn clock_keystate(&mut self, keystate: u32) {
        let key_changed = (keystate.bitor(self.m_key_state as u32)) as u8 != self.m_key_state;
        if key_changed {
            self.m_key_state = (keystate & 0x01) as u8; // convert u32 to u8

            if keystate != 0 {
                self.start_attack();
            } else {
                self.start_release();
            }
        }
    }
}

