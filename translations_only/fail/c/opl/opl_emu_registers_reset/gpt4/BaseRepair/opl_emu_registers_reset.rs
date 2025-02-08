
use std::mem::MaybeUninit;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

impl OplEmuRegisters {
    fn new() -> OplEmuRegisters {
        OplEmuRegisters {
            m_lfo_am_counter: 0,
            m_lfo_pm_counter: 0,
            m_noise_lfsr: 0,
            m_lfo_am: 0,
            m_regdata: [0; OPL_EMU_REGISTERS_REGISTERS],
            m_waveform: Box::new(unsafe { MaybeUninit::uninit().assume_init() }),
        }
    }
}

fn opl_emu_registers_reset(regs: &mut OplEmuRegisters) {
    for i in 0..OPL_EMU_REGISTERS_REGISTERS {
        regs.m_regdata[i] = 0;
    }
}

fn main() {
    let mut regs = OplEmuRegisters::new();
    opl_emu_registers_reset(&mut regs);
}
