
#[derive(PartialEq, PartialOrd)]
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
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
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
    fn opl_emu_fm_operator_start_release(&mut self) {
        if self.m_env_state >= OplEmuEnvelopeState::OplEmuEgRelease {
            return;
        }
        self.m_env_state = OplEmuEnvelopeState::OplEmuEgRelease;
    }

    fn opl_emu_fm_operator_start_attack(&mut self) {
        if self.m_env_state == OplEmuEnvelopeState::OplEmuEgAttack {
            return;
        }
        self.m_env_state = OplEmuEnvelopeState::OplEmuEgAttack;
        self.m_phase = 0;
        if self.m_cache.eg_rate[OplEmuEnvelopeState::OplEmuEgAttack as usize - 1] >= 62 {
            self.m_env_attenuation = 0;
        }
    }

    fn opl_emu_fm_operator_clock_keystate(&mut self, keystate: u32) {
        if (keystate ^ self.m_key_state as u32) != 0 {
            self.m_key_state = keystate as u8;
            if keystate != 0 {
                self.opl_emu_fm_operator_start_attack();
            } else {
                self.opl_emu_fm_operator_start_release();
            }
        }
    }
}
