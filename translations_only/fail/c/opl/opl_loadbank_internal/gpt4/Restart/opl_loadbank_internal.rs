
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

#[derive(Clone, Copy)]
enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
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
    eg_rate: [u8; OplEmuEnvelopeState::States as usize],
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

struct OplTimbre {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS],
}

struct VoiceAlloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

struct Opl {
    notes2voices: [[[i8; 2]; 128]; 16],
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

impl Opl {
    fn opl_loadbank_internal(&mut self, file: &str, offset: usize) -> io::Result<i32> {
        self.is_op2 = false;
        let mut buff = [0u8; 16];
        let mut f = File::open(file)?;
        f.seek(SeekFrom::End(0))?;
        if f.stream_position()? != 3204 {
            return Ok(-2);
        }
        f.seek(SeekFrom::Start(0))?;
        f.read_exact(&mut buff[..4])?;
        if &buff[..4] != [b'I', b'B', b'K', 0x1A] {
            return Ok(-3);
        }
        for i in offset..(128 + offset) {
            f.read_exact(&mut buff)?;
            self.opl_gmtimbres[i].modulator_E862 = u32::from(buff[8]) << 24
                | u32::from(buff[6]) << 16
                | u32::from(buff[4]) << 8
                | u32::from(buff[0]);
            self.opl_gmtimbres[i].carrier_E862 = u32::from(buff[9]) << 24
                | u32::from(buff[7]) << 16
                | u32::from(buff[5]) << 8
                | u32::from(buff[1]);
            self.opl_gmtimbres[i].modulator_40 = buff[2];
            self.opl_gmtimbres[i].carrier_40 = buff[3];
            self.opl_gmtimbres[i].feedconn = buff[10];
            self.opl_gmtimbres[i].finetune = buff[12] as i8;
            self.opl_gmtimbres[i].notenum = 60;
            self.opl_gmtimbres[i].noteoffset = 0;
        }
        Ok(0)
    }
}
