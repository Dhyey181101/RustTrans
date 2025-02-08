

use std::mem;

const POWER_TABLE: [u32; 256] = [
    0x3fa00, 0x3f500, 0x3ef00, 0x3ea00, 0x3e400, 0x3df00, 0x3da00, 0x3d400, 0x3cf00, 0x3c900,
    0x3c400, 0x3bf00, 0x3b900, 0x3b400, 0x3ae00, 0x3a900, 0x3a400, 0x39f00, 0x39900, 0x39400,
    0x38f00, 0x38a00, 0x38400, 0x37f00, 0x37a00, 0x37500, 0x37000, 0x36a00, 0x36500, 0x36000,
    0x35b00, 0x35600, 0x35100, 0x34c00, 0x34700, 0x34200, 0x33d00, 0x33800, 0x33300, 0x32e00,
    0x32900, 0x32400, 0x31f00, 0x31a00, 0x31500, 0x31000, 0x30b00, 0x30600, 0x30200, 0x2fd00,
    0x2f800, 0x2f300, 0x2ee00, 0x2e900, 0x2e500, 0x2e000, 0x2db00, 0x2d600, 0x2d200, 0x2cd00,
    0x2c800, 0x2c400, 0x2bf00, 0x2ba00, 0x2b500, 0x2b100, 0x2ac00, 0x2a800, 0x2a300, 0x29e00,
    0x29a00, 0x29500, 0x29100, 0x28c00, 0x28800, 0x28300, 0x27f00, 0x27a00, 0x27600, 0x27100,
    0x26d00, 0x26800, 0x26400, 0x25f00, 0x25b00, 0x25700, 0x25200, 0x24e00, 0x24900, 0x24500,
    0x24100, 0x23c00, 0x23800, 0x23400, 0x23000, 0x22b00, 0x22700, 0x22300, 0x21e00, 0x21a00,
    0x21600, 0x21200, 0x20e00, 0x20900, 0x20500, 0x20100, 0x1fd00, 0x1f900, 0x1f500, 0x1f000,
    0x1ec00, 0x1e800, 0x1e400, 0x1e000, 0x1dc00, 0x1d800, 0x1d400, 0x1d000, 0x1cc00, 0x1c800,
    0x1c400, 0x1c000, 0x1bc00, 0x1b800, 0x1b400, 0x1b000, 0x1ac00, 0x1a800, 0x1a400, 0x1a000,
    0x19c00, 0x19900, 0x19500, 0x19100, 0x18d00, 0x18900, 0x18500, 0x18100, 0x17e00, 0x17a00,
    0x17600, 0x17200, 0x16f00, 0x16b00, 0x16700, 0x16300, 0x16000, 0x15c00, 0x15800, 0x15400,
    0x15100, 0x14d00, 0x14900, 0x14600, 0x14200, 0x13e00, 0x13b00, 0x13700, 0x13400, 0x13000,
    0x12c00, 0x12900, 0x12500, 0x12200, 0x11e00, 0x11b00, 0x11700, 0x11400, 0x11000, 0x10c00,
    0x10900, 0x10600, 0x10200, 0x0ff00, 0x0fb00, 0x0f800, 0x0f400, 0x0f100, 0x0ed00, 0x0ea00,
    0x0e700, 0x0e300, 0x0e000, 0x0dc00, 0x0d900, 0x0d600, 0x0d200, 0x0cf00, 0x0cc00, 0x0c800,
    0x0c500, 0x0c200, 0x0be00, 0x0bb00, 0x0b800, 0x0b500, 0x0b100, 0x0ae00, 0x0ab00, 0x0a800,
    0x0a400, 0x0a100, 0x09e00, 0x09b00, 0x09800, 0x09400, 0x09100, 0x08e00, 0x08b00, 0x08800,
    0x08500, 0x08200, 0x07e00, 0x07b00, 0x07800, 0x07500, 0x07200, 0x06f00, 0x06c00, 0x06900,
    0x06600, 0x06300, 0x06000, 0x05d00, 0x05a00, 0x05700, 0x05400, 0x05100, 0x04e00, 0x04b00,
    0x04800, 0x04500, 0x04200, 0x03f00, 0x03c00, 0x03900, 0x03600, 0x03300, 0x03000, 0x02d00,
    0x02a00, 0x02800, 0x02500, 0x02200, 0x01f00, 0x01c00, 0x01900, 0x01600, 0x01400, 0x01100,
    0x00e00, 0x00b00, 0x00800, 0x00600, 0x00300, 0x00000,
];

fn opl_emu_attenuation_to_volume(input: u32) -> u32 {
    let fractional = input & 0xff;
    let whole = (input >> 8) as usize;
    POWER_TABLE[fractional as usize] >> whole
}

