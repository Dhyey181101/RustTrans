









use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::boxed::Box;

const OPL_EMU_REGISTERS_OPERATORS: usize = 36;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

const OPL_EMU_EG_STATES_LEN: usize = OplEmuEnvelopeState::OPL_EMU_EG_STATES as usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op2Flags {
    OP2_FIXEDPITCH = 1,
    OP2_UNUSED = 2,
    OP2_DOUBLEVOICE = 4,
}

#[derive(Debug)]
struct Opl {
    notes2voices: [[[Option<usize>; 2]; 128]; 16],
    channelpitch: [u16; 16],
    channelvol: [u16; 16],
    voices2notes: [VoiceAlloc; 18],
    channelprog: [u8; 16],
    opl3: bool,
    opl_emu: OplEmu,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],
    is_op2: bool,
    op2_flags: [Op2Flags; 256],
}

#[derive(Debug)]
struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [bool; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS],
}

#[derive(Debug)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

#[derive(Debug)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
struct VoiceAlloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

#[derive(Debug)]
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

impl Opl {
    fn loadbank_ibk(&mut self, file: &str) -> io::Result<()> {
        let mut instruments = String::from(file);
        let mut percussion = String::new();
        let mut res = Ok(());

        let mut instruments = instruments.to_owned();
        for (i, c) in instruments.chars().enumerate() {
            if c == ',' {
                instruments.replace_range(i..i + 1, "");
                percussion = instruments[i + 1..].to_owned();
                break;
            }
        }

        res = self.loadbank_internal(instruments, 0);
        if res.is_ok() && !percussion.is_empty() {
            res = self.loadbank_internal(percussion, 128);
        }

        Ok(())
    }

    fn loadbank_internal(&mut self, mut instruments: String, offset: usize) -> io::Result<()> {
        self.is_op2 = false;

        let mut buff = [0u8; 16];
        for i in offset..128 + offset {
            instruments = format!("{}\n", instruments.trim_end());
            let mut f = File::open(&instruments)?;
            f.read_exact(&mut buff)?;

            let modulator_E862 = (buff[8] as u32) << 24
                | (buff[6] as u32) << 16
                | (buff[4] as u32) << 8
                | (buff[0] as u32);
            let carrier_E862 = (buff[9] as u32) << 24
                | (buff[7] as u32) << 16
                | (buff[5] as u32) << 8
                | (buff[1] as u32);
            let modulator_40 = buff[2];
            let carrier_40 = buff[3];
            let feedconn = buff[10];
            let finetune = (buff[12].wrapping_sub(128)) as i8;
            let notenum = 60;
            let noteoffset = 0;

            self.opl_gmtimbres[i] = OplTimbre {
                modulator_E862,
                modulator_40,
                carrier_E862,
                carrier_40,
                feedconn,
                finetune,
                notenum,
                noteoffset,
            };
        }

        Ok(())
    }
}

fn main() {}









