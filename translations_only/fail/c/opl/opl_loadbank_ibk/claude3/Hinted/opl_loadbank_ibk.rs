

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_EG_STATES: usize = 6;

#[derive(Copy, Clone)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

#[derive(Copy, Clone)]
enum Op2FlagsT {
    Op2Fixedpitch = 1,
    Op2Unused = 2,
    Op2Doublevoice = 4,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: Box<[u8; OPL_EMU_EG_STATES]>,
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
    m_feedback: Box<[[i16; 2]; 2]>,
    m_feedback_in: i16,
}

struct OplTimbreT {
    modulator_e862: u32,
    carrier_e862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

struct OplEmuT {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: Box<[u8; 2]>,
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: Box<[OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS]>,
    m_operator: Box<[OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS]>,
}

struct VoiceallocT {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

struct OplT {
    notes2voices: Box<[[[i8; 2]; 128]; 16]>,
    channelpitch: Box<[u16; 16]>,
    channelvol: Box<[u16; 16]>,
    voices2notes: Box<[VoiceallocT; 18]>,
    channelprog: Box<[u8; 16]>,
    opl3: i8,
    opl_emu: OplEmuT,
    opl_gmtimbres: Box<[OplTimbreT; 256]>,
    opl_gmtimbres_voice2: Box<[OplTimbreT; 256]>,
    is_op2: i32,
    op2_flags: Box<[Op2FlagsT; 256]>,
}

fn opl_loadbank_internal(opl: &mut OplT, file: &Path, offset: i32) -> i32 {
    opl.is_op2 = 0;
    let mut buff = [0u8; 16];
    let mut f = match File::open(file) {
        Ok(file) => file,
        Err(_) => return -1,
    };

    if f.seek(SeekFrom::End(0)).unwrap() != 3204 {
        return -2;
    }

    f.seek(SeekFrom::Start(0)).unwrap();

    if f.read(&mut buff).unwrap() != 4
        || buff[0] != b'I'
        || buff[1] != b'B'
        || buff[2] != b'K'
        || buff[3] != 0x1A
    {
        return -3;
    }

    for i in offset..offset + 128 {
        if f.read(&mut buff).unwrap() != 16 {
            return -4;
        }

        opl.opl_gmtimbres[i as usize].modulator_e862 = u32::from(buff[8]); // wave select
        opl.opl_gmtimbres[i as usize].modulator_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].modulator_e862 |= u32::from(buff[6]); // sust/release
        opl.opl_gmtimbres[i as usize].modulator_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].modulator_e862 |= u32::from(buff[4]); // attack/decay
        opl.opl_gmtimbres[i as usize].modulator_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].modulator_e862 |= u32::from(buff[0]); // AM/VIB... flags

        opl.opl_gmtimbres[i as usize].carrier_e862 = u32::from(buff[9]); // wave select
        opl.opl_gmtimbres[i as usize].carrier_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].carrier_e862 |= u32::from(buff[7]); // sust/release
        opl.opl_gmtimbres[i as usize].carrier_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].carrier_e862 |= u32::from(buff[5]); // attack/decay
        opl.opl_gmtimbres[i as usize].carrier_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].carrier_e862 |= u32::from(buff[1]); // AM/VIB... flags

        opl.opl_gmtimbres[i as usize].modulator_40 = buff[2];
        opl.opl_gmtimbres[i as usize].carrier_40 = buff[3];

        opl.opl_gmtimbres[i as usize].feedconn = buff[10];
        opl.opl_gmtimbres[i as usize].finetune = buff[12] as i8; // used only in some IBK files
        opl.opl_gmtimbres[i as usize].notenum = 60;
        opl.opl_gmtimbres[i as usize].noteoffset = 0;
    }

    0
}

fn opl_loadbank_ibk(opl: &mut OplT, file: &Path) -> i32 {
    let mut instruments = None;
    let mut percussion = None;
    let mut res;

    if let Some(instruments_str) = file.to_str() {
        instruments = Some(instruments_str.to_owned());
    } else {
        return -64;
    }

    if let Some(instruments_str) = instruments.as_mut() {
        if let Some(comma_pos) = instruments_str.find(',') {
            percussion = Some(instruments_str.split_off(comma_pos + 1));
            instruments_str.truncate(comma_pos);
        }
    }

    res = opl_loadbank_internal(opl, Path::new(&instruments.unwrap()), 0);

    if res == 0 && percussion.is_some() {
        res = opl_loadbank_internal(opl, Path::new(&percussion.unwrap()), 128);
    }

    res
}

