
use std::mem::MaybeUninit;

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
    opl3: i32,
    opl_emu: Box<OplEmu>,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],
    is_op2: i32,
    op2_flags: [Op2Flags; 256],
}

impl Opl {
    fn new() -> Self {
        Self {
            notes2voices: [[[0; 2]; 128]; 16],
            channelpitch: [0; 16],
            channelvol: [0; 16],
            voices2notes: unsafe { MaybeUninit::zeroed().assume_init() },
            channelprog: [0; 16],
            opl3: 0,
            opl_emu: Box::new(OplEmu {
                m_env_counter: 0,
                m_status: 0,
                m_timer_running: [0; 2],
                m_active_channels: 0,
                m_modified_channels: 0,
                m_prepare_count: 0,
                m_regs: OplEmuRegisters {
                    m_lfo_am_counter: 0,
                    m_lfo_pm_counter: 0,
                    m_noise_lfsr: 0,
                    m_lfo_am: 0,
                    m_regdata: [0; OPL_EMU_REGISTERS_REGISTERS],
                    m_waveform: [[0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
                },
                m_channel: unsafe { MaybeUninit::zeroed().assume_init() },
                m_operator: unsafe { MaybeUninit::zeroed().assume_init() },
            }),
            opl_gmtimbres: unsafe { MaybeUninit::zeroed().assume_init() },
            opl_gmtimbres_voice2: unsafe { MaybeUninit::zeroed().assume_init() },
            is_op2: 0,
            op2_flags: unsafe { MaybeUninit::zeroed().assume_init() },
        }
    }

    fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
        timbre.modulator_E862 = ((buff[3] as u32) << 24) | ((buff[2] as u32) << 16) | ((buff[1] as u32) << 8) | buff[0] as u32;
        timbre.carrier_E862 = ((buff[10] as u32) << 24) | ((buff[9] as u32) << 16) | ((buff[8] as u32) << 8) | buff[7] as u32;
        timbre.modulator_40 = (buff[5] & 0x3f) | (buff[4] & 0xc0);
        timbre.carrier_40 = (buff[12] & 0x3f) | (buff[11] & 0xc0);
        timbre.feedconn = buff[6];
        timbre.finetune = 0;
        timbre.noteoffset = ((buff[14] as i16) | ((buff[15] as i16) << 8));
    }

    fn opl_loadbank_op2(&mut self, data: &[u8]) -> i32 {
        if data.len() < 8 + 36 * 175 {
            return -3;
        }
        let mut buff = &data[8..];

        self.is_op2 = 1;

        for i in 0..175 {
            let flags = (buff[0] as u16) | ((buff[1] as u16) << 8);
            self.op2_flags[i] = match flags {
                1 => Op2Flags::FixedPitch,
                2 => Op2Flags::Unused,
                4 => Op2Flags::DoubleVoice,
                _ => continue,
            };
            let finetune = buff[2];
            let fixednote = buff[3];
            buff = &buff[4..];

            Opl::opl_load_op2_voice(&mut self.opl_gmtimbres[i], &buff[..16]);
            self.opl_gmtimbres[i].notenum = fixednote;
            buff = &buff[16..];

            Opl::opl_load_op2_voice(&mut self.opl_gmtimbres_voice2[i], &buff[..16]);
            self.opl_gmtimbres_voice2[i].notenum = fixednote;
            self.opl_gmtimbres_voice2[i].finetune += finetune.wrapping_sub(128) as i8;
            buff = &buff[16..];
        }

        0
    }
}

fn main() {
    let mut opl = Opl::new();
    let data: Vec<u8> = vec![];
    let _ = opl.opl_loadbank_op2(&data);
}
