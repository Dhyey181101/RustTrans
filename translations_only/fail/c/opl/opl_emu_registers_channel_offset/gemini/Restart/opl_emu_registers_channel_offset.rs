
pub fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    (chnum % 9) + 0x100 * (chnum / 9)
}
