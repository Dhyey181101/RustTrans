

use std::boxed::Box;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OplEmuKeyonType {
    OplEmuKeyonNormal = 0,
    OplEmuKeyonRhythm = 1,
    OplEmuKeyCsm = 2,
}

const OPL_EMU_REGISTERS_OPERATORS: u32 = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = (1 << OPL_EMU_REGISTERS_CHANNELS) - 1;
const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u32 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_CHANNELS: u32 = 18;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u32 = 0x04;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize],
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
    m_keyon_live: u32,
    m_cache: OplEmuOpdataCache,
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
}

struct OplEmuT {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS as usize],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS as usize],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start as u32) & ((1 << length) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, type_: OplEmuKeyonType) {
    fmop.m_keyon_live = (fmop.m_keyon_live & !(1 << (type_ as u32))) | (opl_emu_bitfield(on, 0, 1) << (type_ as u32));
}

fn opl_emu_fm_channel_keyonoff(fmch: &mut OplEmuFmChannel, states: u32, type_: OplEmuKeyonType, chnum: u32) {
    for opnum in 0..fmch.m_op.len() {
        if let Some(op) = &mut fmch.m_op[opnum as usize] {
            opl_emu_fm_operator_keyonoff(op, opl_emu_bitfield(states, opnum as i32, 1), type_);
        }
    }
}

fn opl_emu_registers_write(regs: &mut OplEmuRegisters, index: u16, data: u8, channel: &mut u32, opmask: &mut u32) -> bool {
    if u32::from(index) == OPL_EMU_REGISTERS_REG_MODE && opl_emu_bitfield(data as u32, 7, 1) != 0 {
        regs.m_regdata[index as usize] |= 0x80;
    } else {
        regs.m_regdata[index as usize] = data;
    }

    if index == 0xbd {
        *channel = OPL_EMU_REGISTERS_RHYTHM_CHANNEL;
        *opmask = if opl_emu_bitfield(data as u32, 5, 1) != 0 {
            opl_emu_bitfield(data as u32, 0, 5)
        } else {
            0
        };
        return true;
    }

    if (index & 0xf0) == 0xb0 {
        *channel = (index & 0x0f) as u32;
        if *channel < 9 {
            let wrapping_add_result = (*channel).wrapping_add(9 * opl_emu_bitfield(index as u32, 8, 1));
            *channel = wrapping_add_result;
            *opmask = if opl_emu_bitfield(data as u32, 5, 1) != 0 {
                15
            } else {
                0
            };
            return true;
        }
    }

    false
}

fn opl_emu_write(emu: &mut OplEmuT, regnum: u16, data: u8) {
    if u32::from(regnum) == OPL_EMU_REGISTERS_REG_MODE {
        return;
    }

    emu.m_modified_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;

    let mut keyon_channel = 0;
    let mut keyon_opmask = 0;
    if opl_emu_registers_write(&mut emu.m_regs, regnum, data, &mut keyon_channel, &mut keyon_opmask) {
        if keyon_channel < OPL_EMU_REGISTERS_CHANNELS {
            opl_emu_fm_channel_keyonoff(&mut emu.m_channel[keyon_channel as usize], keyon_opmask, OplEmuKeyonType::OplEmuKeyonNormal, keyon_channel);
        } else if OPL_EMU_REGISTERS_CHANNELS >= 9 && keyon_channel == OPL_EMU_REGISTERS_RHYTHM_CHANNEL {
            opl_emu_fm_channel_keyonoff(&mut emu.m_channel[6], if opl_emu_bitfield(keyon_opmask, 4, 1) != 0 { 3 } else { 0 }, OplEmuKeyonType::OplEmuKeyonRhythm, 6);
            opl_emu_fm_channel_keyonoff(&mut emu.m_channel[7], opl_emu_bitfield(keyon_opmask, 0, 1) | (opl_emu_bitfield(keyon_opmask, 3, 1) << 1), OplEmuKeyonType::OplEmuKeyonRhythm, 7);
            opl_emu_fm_channel_keyonoff(&mut emu.m_channel[8], opl_emu_bitfield(keyon_opmask, 2, 1) | (opl_emu_bitfield(keyon_opmask, 1, 1) << 1), OplEmuKeyonType::OplEmuKeyonRhythm, 8);
        }
    }
}

