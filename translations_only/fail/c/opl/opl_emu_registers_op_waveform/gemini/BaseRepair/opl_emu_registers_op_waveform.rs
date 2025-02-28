
use std::boxed::Box;

#[derive(Clone)]
pub struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; 0x200]>,
    m_waveform: Box<[[u16; 0x400]; 8]>,
}

impl OplEmuRegisters {
    pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
        ((value >> start) & ((1 << length) - 1)) as u32
    }

    pub fn opl_emu_registers_byte(
        &self,
        offset: u32,
        start: u32,
        count: u32,
        extra_offset: u32,
    ) -> u32 {
        Self::opl_emu_bitfield(
            self.m_regdata[offset as usize + extra_offset as usize] as u32,
            start as i32,
            count as i32,
        )
    }

    pub fn opl_emu_registers_newflag(&self) -> u32 {
        Self::opl_emu_registers_byte(self, 0x105, 0, 1, 0)
    }

    pub fn opl_emu_registers_op_waveform(&self, opoffs: u32) -> u32 {
        Self::opl_emu_registers_byte(
            self,
            0xe0,
            0,
            if self.opl_emu_registers_newflag() != 0 { 3 } else { 2 },
            opoffs,
        )
    }
}
