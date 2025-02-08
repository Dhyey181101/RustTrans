

use std::fs::File;
use std::io::Read;
use std::mem;
use std::option::Option;
use std::ptr;

const OPL_EMU_REGISTERS_OPERATORS: usize = 36;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, PartialEq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

const OPL_EMU_EG_STATES_LEN: usize = OplEmuEnvelopeState::OPL_EMU_EG_STATES as usize;

#[derive(Clone)]
struct Opl {
    notes2voices: [[[Option<usize>; 2]; 128]; 16],
    channelpitch: [u16; 16],
    channelvol: [u16; 16],
    voices2notes: [Option<VoiceAlloc>; 18],
    channelprog: [u8; 16],
    opl3: bool,
    opl_emu: OplEmu,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],
    is_op2: bool,
    //op2_flags: [Op2Flags; 256],
}

#[derive(Clone)]
struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [bool; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [Box<OplEmuFmChannel>; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [Box<OplEmuFmOperator>; OPL_EMU_REGISTERS_OPERATORS],
}

#[derive(Clone)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

#[derive(Clone)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
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
    m_cache: OplEmuOpdataCache,
}

#[derive(Clone)]
struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OPL_EMU_EG_STATES_LEN],
    eg_shift: u8,
}

#[derive(Clone)]
struct OplTimbre {
    modulator_E862: u32,
    modulator_40: u8,
    carrier_E862: u32,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

#[derive(Clone)]
struct VoiceAlloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

impl Opl {
    fn opl_loadbank_internal(&mut self, file: &str, offset: i32) -> i32 {
        self.is_op2 = false;
        let mut buff: [u8; 16] = [0; 16];
        let mut i: i32 = offset;

        let mut f = File::open(file).unwrap();
        if f.read(&mut buff).unwrap() != 16 {
            return -4;
        }

        if &buff[0..4] != b"IBK\x1A" {
            return -3;
        }

        for i in offset..128 + offset {
            let timbre = &mut self.opl_gmtimbres[i as usize];

            timbre.modulator_E862 = (buff[6] as u32) << 8 | buff[8] as u32;
            timbre.modulator_E862 <<= 8;
            timbre.modulator_E862 |= (buff[4] as u32) << 8 | buff[0] as u32;

            timbre.carrier_E862 = (buff[7] as u32) << 8 | buff[9] as u32;
            timbre.carrier_E862 <<= 8;
            timbre.carrier_E862 |= (buff[5] as u32) << 8 | buff[1] as u32;

            timbre.modulator_40 = buff[2];
            timbre.carrier_40 = buff[3];

            timbre.feedconn = buff[10];
            timbre.finetune = buff[12] as i8;

            timbre.notenum = 60;
            timbre.noteoffset = 0;

            if i < 128 {
                self.notes2voices[0][i as usize][0] = Some(i as usize);
            }
            if i < 256 {
                self.notes2voices[1][i as usize][0] = Some(i as usize);
            }

            if (i - offset) % 2 == 1 {
                self.notes2voices[0][i as usize][1] = Some(i as usize);
            }

            if (i - offset) % 2 == 0 {
                self.notes2voices[1][i as usize][1] = Some(i as usize);
            }
        }

        0
    }
}

