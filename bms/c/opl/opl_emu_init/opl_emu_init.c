#include <stdio.h>

#include <stdint.h>

#include <stdlib.h> /* calloc() */

#include <string.h> /* strdup() */

#define OPL_EMU_REGISTERS_OPERATORS ( OPL_EMU_REGISTERS_CHANNELS * 2 )

#define OPL_EMU_REGISTERS_ALL_CHANNELS ( (1 << OPL_EMU_REGISTERS_CHANNELS) - 1 )

#define OPL_EMU_REGISTERS_WAVEFORMS 8

#define OPL_EMU_REGISTERS_CHANNELS 18

#define OPL_EMU_REGISTERS_REGISTERS 0x200

#define OPL_EMU_REGISTERS_WAVEFORM_LENGTH 0x400

enum opl_emu_envelope_state
{
	OPL_EMU_EG_ATTACK = 1,
	OPL_EMU_EG_DECAY = 2,
	OPL_EMU_EG_SUSTAIN = 3,
	OPL_EMU_EG_RELEASE = 4,
	OPL_EMU_EG_STATES = 6
};


struct opl_emu_registers
{
	// internal state
	uint16_t m_lfo_am_counter;            // LFO AM counter
	uint16_t m_lfo_pm_counter;            // LFO PM counter
	uint32_t m_noise_lfsr;                // noise LFSR state
	uint8_t m_lfo_am;                     // current LFO AM value
	uint8_t m_regdata[OPL_EMU_REGISTERS_REGISTERS];         // register data
	uint16_t m_waveform[OPL_EMU_REGISTERS_WAVEFORMS][OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; // waveforms
};
struct opl_emu_registers_operator_mapping { uint32_t chan[OPL_EMU_REGISTERS_CHANNELS]; };
struct opl_emu_opdata_cache
{
	// set phase_step to this value to recalculate it each sample; needed
	// in the case of PM LFO changes

	uint32_t phase_step;              // phase step, or OPL_EMU_PHASE_STEP_DYNAMIC if PM is active
	uint32_t total_level;             // total level * 8 + KSL
	uint32_t block_freq;              // raw block frequency value (used to compute phase_step)
	int32_t detune;                   // detuning value (used to compute phase_step)
	uint32_t multiple;                // multiple value (x.1, used to compute phase_step)
	uint32_t eg_sustain;              // sustain level, shifted up to envelope values
	uint8_t eg_rate[OPL_EMU_EG_STATES];       // envelope rate, including KSR
	uint8_t eg_shift;                 // envelope shift amount
};
struct opl_emu_fm_operator
{
	// internal state
	uint32_t m_choffs;                     // channel offset in registers
	uint32_t m_opoffs;                     // operator offset in registers
	uint32_t m_phase;                      // current phase value (10.10 format)
	uint16_t m_env_attenuation;            // computed envelope attenuation (4.6 format)
	enum opl_emu_envelope_state m_env_state;            // current envelope state
	uint8_t m_key_state;                   // current key state: on or off (bit 0)
	uint8_t m_keyon_live;                  // live key on state (bit 0 = direct, bit 1 = rhythm, bit 2 = CSM)
	struct opl_emu_opdata_cache m_cache;                  // cached values for performance
	struct opl_emu_registers* m_regs;                  // direct reference to registers
};
struct opl_emu_fm_channel
{
	// internal state
	uint32_t m_choffs;                     // channel offset in registers
	int16_t m_feedback[2];                 // feedback memory for operator 1
	int16_t m_feedback_in;         // next input value for op 1 feedback (set in output)
	struct opl_emu_fm_operator *m_op[4];    // up to 4 operators
	struct opl_emu_registers* m_regs;                  // direct reference to registers
};
struct opl_emu_t
{
	uint32_t m_env_counter;          // envelope counter; low 2 bits are sub-counter
	uint8_t m_status;                // current status register
	uint8_t m_timer_running[2];      // current timer running state
	uint32_t m_active_channels;      // mask of active channels (computed by prepare)
	uint32_t m_modified_channels;    // mask of channels that have been modified
	uint32_t m_prepare_count;        // counter to do periodic prepare sweeps
	struct opl_emu_registers m_regs;             // register accessor
	struct opl_emu_fm_channel m_channel[OPL_EMU_REGISTERS_CHANNELS]; // channel pointers
	struct opl_emu_fm_operator m_operator[OPL_EMU_REGISTERS_OPERATORS]; // operator pointers
};
uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
;
uint32_t opl_emu_registers_byte(struct opl_emu_registers* regs,uint32_t offset, uint32_t start, uint32_t count, uint32_t extra_offset/* = 0*/) 
;
void opl_emu_fm_operator_set_choffs(struct opl_emu_fm_operator* fmop,uint32_t choffs) ;
uint32_t opl_emu_registers_operator_list(uint8_t o1, uint8_t o2, uint8_t o3, uint8_t o4)
;
uint32_t opl_emu_registers_fourop_enable(struct opl_emu_registers* regs)                    ;
void opl_emu_fm_channel_assign(struct opl_emu_fm_channel* fmch,uint32_t index, struct opl_emu_fm_operator *op)
;
void opl_emu_registers_operator_map(struct opl_emu_registers* regs, struct opl_emu_registers_operator_mapping* dest)
;
uint32_t opl_emu_abs_sin_attenuation(uint32_t input)
;
void opl_emu_assign_operators( struct opl_emu_t* emu)
;
void opl_emu_fm_operator_init(struct opl_emu_fm_operator* fmop, struct opl_emu_registers* regs, uint32_t opoffs) ;
uint32_t opl_emu_registers_operator_offset(uint32_t opnum)
;
void opl_emu_fm_channel_init(struct opl_emu_fm_channel* fmch,struct opl_emu_registers* regs, uint32_t choffs) ;
uint32_t opl_emu_registers_channel_offset(uint32_t chnum)
;
void opl_emu_registers_init(struct opl_emu_registers* regs) 
;
void opl_emu_init( struct opl_emu_t* emu ) 
;
uint32_t opl_emu_bitfield(uint32_t value, int start, int length )
{
	return (value >> start) & ((1 << length) - 1);
}
uint32_t opl_emu_registers_byte(struct opl_emu_registers* regs,uint32_t offset, uint32_t start, uint32_t count, uint32_t extra_offset/* = 0*/) 
{
	return opl_emu_bitfield(regs->m_regdata[offset + extra_offset], start, count);
}
void opl_emu_fm_operator_set_choffs(struct opl_emu_fm_operator* fmop,uint32_t choffs) { fmop->m_choffs = choffs; }
uint32_t opl_emu_registers_operator_list(uint8_t o1, uint8_t o2, uint8_t o3, uint8_t o4)
{
	return o1 | (o2 << 8) | (o3 << 16) | (o4 << 24);
}
uint32_t opl_emu_registers_fourop_enable(struct opl_emu_registers* regs)                    { return opl_emu_registers_byte(regs,0x104, 0, 6, 0); }
void opl_emu_fm_channel_assign(struct opl_emu_fm_channel* fmch,uint32_t index, struct opl_emu_fm_operator *op)
{
	fmch->m_op[index] = op;
	if (op != NULL)
		opl_emu_fm_operator_set_choffs(op, fmch->m_choffs);
}
void opl_emu_registers_operator_map(struct opl_emu_registers* regs, struct opl_emu_registers_operator_mapping* dest)
{
    // OPL3/OPL4 can be configured for 2 or 4 operators
    uint32_t fourop = opl_emu_registers_fourop_enable(regs);

    dest->chan[ 0] = opl_emu_bitfield(fourop, 0,1) ? opl_emu_registers_operator_list(  0,  3,  6,  9 ) : opl_emu_registers_operator_list(  0,  3, 0xff, 0xff );
    dest->chan[ 1] = opl_emu_bitfield(fourop, 1,1) ? opl_emu_registers_operator_list(  1,  4,  7, 10 ) : opl_emu_registers_operator_list(  1,  4, 0xff, 0xff );
    dest->chan[ 2] = opl_emu_bitfield(fourop, 2,1) ? opl_emu_registers_operator_list(  2,  5,  8, 11 ) : opl_emu_registers_operator_list(  2,  5, 0xff, 0xff );
    dest->chan[ 3] = opl_emu_bitfield(fourop, 0,1) ? opl_emu_registers_operator_list( 0xff, 0xff, 0xff, 0xff) : opl_emu_registers_operator_list(  6,  9, 0xff, 0xff );
    dest->chan[ 4] = opl_emu_bitfield(fourop, 1,1) ? opl_emu_registers_operator_list( 0xff, 0xff, 0xff, 0xff) : opl_emu_registers_operator_list(  7, 10, 0xff, 0xff );
    dest->chan[ 5] = opl_emu_bitfield(fourop, 2,1) ? opl_emu_registers_operator_list( 0xff, 0xff, 0xff, 0xff) : opl_emu_registers_operator_list(  8, 11, 0xff, 0xff );
    dest->chan[ 6] = opl_emu_registers_operator_list( 12, 15, 0xff, 0xff );
    dest->chan[ 7] = opl_emu_registers_operator_list( 13, 16, 0xff, 0xff );
    dest->chan[ 8] = opl_emu_registers_operator_list( 14, 17, 0xff, 0xff );

    dest->chan[ 9] = opl_emu_bitfield(fourop, 3,1) ? opl_emu_registers_operator_list( 18, 21, 24, 27 ) : opl_emu_registers_operator_list( 18, 21, 0xff, 0xff );
    dest->chan[10] = opl_emu_bitfield(fourop, 4,1) ? opl_emu_registers_operator_list( 19, 22, 25, 28 ) : opl_emu_registers_operator_list( 19, 22, 0xff, 0xff );
    dest->chan[11] = opl_emu_bitfield(fourop, 5,1) ? opl_emu_registers_operator_list( 20, 23, 26, 29 ) : opl_emu_registers_operator_list( 20, 23, 0xff, 0xff );
    dest->chan[12] = opl_emu_bitfield(fourop, 3,1) ? opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff) : opl_emu_registers_operator_list( 24, 27, 0xff, 0xff );
    dest->chan[13] = opl_emu_bitfield(fourop, 4,1) ? opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff) : opl_emu_registers_operator_list( 25, 28, 0xff, 0xff );
    dest->chan[14] = opl_emu_bitfield(fourop, 5,1) ? opl_emu_registers_operator_list(0xff, 0xff, 0xff, 0xff) : opl_emu_registers_operator_list( 26, 29, 0xff, 0xff );
    dest->chan[15] = opl_emu_registers_operator_list( 30, 33, 0xff, 0xff );
    dest->chan[16] = opl_emu_registers_operator_list( 31, 34, 0xff, 0xff );
    dest->chan[17] = opl_emu_registers_operator_list( 32, 35, 0xff, 0xff );
}
uint32_t opl_emu_abs_sin_attenuation(uint32_t input)
{
	// the values here are stored as 4.8 logarithmic values for 1/4 phase
	// this matches the internal format of the OPN chip, extracted from the die
	static uint16_t const s_sin_table[256] =
	{
		0x859,0x6c3,0x607,0x58b,0x52e,0x4e4,0x4a6,0x471,0x443,0x41a,0x3f5,0x3d3,0x3b5,0x398,0x37e,0x365,
		0x34e,0x339,0x324,0x311,0x2ff,0x2ed,0x2dc,0x2cd,0x2bd,0x2af,0x2a0,0x293,0x286,0x279,0x26d,0x261,
		0x256,0x24b,0x240,0x236,0x22c,0x222,0x218,0x20f,0x206,0x1fd,0x1f5,0x1ec,0x1e4,0x1dc,0x1d4,0x1cd,
		0x1c5,0x1be,0x1b7,0x1b0,0x1a9,0x1a2,0x19b,0x195,0x18f,0x188,0x182,0x17c,0x177,0x171,0x16b,0x166,
		0x160,0x15b,0x155,0x150,0x14b,0x146,0x141,0x13c,0x137,0x133,0x12e,0x129,0x125,0x121,0x11c,0x118,
		0x114,0x10f,0x10b,0x107,0x103,0x0ff,0x0fb,0x0f8,0x0f4,0x0f0,0x0ec,0x0e9,0x0e5,0x0e2,0x0de,0x0db,
		0x0d7,0x0d4,0x0d1,0x0cd,0x0ca,0x0c7,0x0c4,0x0c1,0x0be,0x0bb,0x0b8,0x0b5,0x0b2,0x0af,0x0ac,0x0a9,
		0x0a7,0x0a4,0x0a1,0x09f,0x09c,0x099,0x097,0x094,0x092,0x08f,0x08d,0x08a,0x088,0x086,0x083,0x081,
		0x07f,0x07d,0x07a,0x078,0x076,0x074,0x072,0x070,0x06e,0x06c,0x06a,0x068,0x066,0x064,0x062,0x060,
		0x05e,0x05c,0x05b,0x059,0x057,0x055,0x053,0x052,0x050,0x04e,0x04d,0x04b,0x04a,0x048,0x046,0x045,
		0x043,0x042,0x040,0x03f,0x03e,0x03c,0x03b,0x039,0x038,0x037,0x035,0x034,0x033,0x031,0x030,0x02f,
		0x02e,0x02d,0x02b,0x02a,0x029,0x028,0x027,0x026,0x025,0x024,0x023,0x022,0x021,0x020,0x01f,0x01e,
		0x01d,0x01c,0x01b,0x01a,0x019,0x018,0x017,0x017,0x016,0x015,0x014,0x014,0x013,0x012,0x011,0x011,
		0x010,0x00f,0x00f,0x00e,0x00d,0x00d,0x00c,0x00c,0x00b,0x00a,0x00a,0x009,0x009,0x008,0x008,0x007,
		0x007,0x007,0x006,0x006,0x005,0x005,0x005,0x004,0x004,0x004,0x003,0x003,0x003,0x002,0x002,0x002,
		0x002,0x001,0x001,0x001,0x001,0x001,0x001,0x001,0x000,0x000,0x000,0x000,0x000,0x000,0x000,0x000
	};

	// if the top bit is set, we're in the second half of the curve
	// which is a mirror image, so invert the index
	if (opl_emu_bitfield(input, 8,1))
		input = ~input;

	// return the value from the table
	return s_sin_table[input & 0xff];
}
void opl_emu_assign_operators( struct opl_emu_t* emu)
{
	struct opl_emu_registers_operator_mapping map;
	opl_emu_registers_operator_map(&emu->m_regs, &map);

	for (uint32_t chnum = 0; chnum < OPL_EMU_REGISTERS_CHANNELS; chnum++)
		for (uint32_t index = 0; index < 4; index++)
		{
			uint32_t opnum = opl_emu_bitfield(map.chan[chnum], 8 * index, 8);
			opl_emu_fm_channel_assign(&emu->m_channel[chnum],index, (opnum == 0xff) ? NULL : &emu->m_operator[opnum]);
		}
}
void opl_emu_fm_operator_init(struct opl_emu_fm_operator* fmop, struct opl_emu_registers* regs, uint32_t opoffs) {
	fmop->m_choffs = 0;
	fmop->m_opoffs = opoffs;
	fmop->m_phase = 0;
	fmop->m_env_attenuation = 0x3ff;
	fmop->m_env_state = OPL_EMU_EG_RELEASE;
	fmop->m_key_state = 0;
	fmop->m_keyon_live = 0;
	fmop->m_regs = regs;
	fmop->m_cache.eg_shift = 0;
}
uint32_t opl_emu_registers_operator_offset(uint32_t opnum)
{
    return (opnum % 18) + 2 * ((opnum % 18) / 6) + 0x100 * (opnum / 18);
}
void opl_emu_fm_channel_init(struct opl_emu_fm_channel* fmch,struct opl_emu_registers* regs, uint32_t choffs) {
	fmch->m_choffs = choffs;
	fmch->m_feedback[ 0 ] = 0;
    fmch->m_feedback[ 1 ] = 0;
	fmch->m_feedback_in = 0;
	fmch->m_op[ 0 ] = NULL;
	fmch->m_op[ 1 ] = NULL;
	fmch->m_op[ 2 ] = NULL;
	fmch->m_op[ 3 ] = NULL;
	fmch->m_regs = regs;
}
uint32_t opl_emu_registers_channel_offset(uint32_t chnum)
{
    return (chnum % 9) + 0x100 * (chnum / 9);
}
void opl_emu_registers_init(struct opl_emu_registers* regs) 
{
	regs->m_lfo_am_counter = 0;
	regs->m_lfo_pm_counter = 0;
	regs->m_noise_lfsr = 1;
	regs->m_lfo_am = 0;

	// create these pointers to appease overzealous compilers checking array
	// bounds in unreachable code (looking at you, clang)
	uint16_t *wf0 = &regs->m_waveform[0][0];
	uint16_t *wf1 = &regs->m_waveform[1 % OPL_EMU_REGISTERS_WAVEFORMS][0];
	uint16_t *wf2 = &regs->m_waveform[2 % OPL_EMU_REGISTERS_WAVEFORMS][0];
	uint16_t *wf3 = &regs->m_waveform[3 % OPL_EMU_REGISTERS_WAVEFORMS][0];
	uint16_t *wf4 = &regs->m_waveform[4 % OPL_EMU_REGISTERS_WAVEFORMS][0];
	uint16_t *wf5 = &regs->m_waveform[5 % OPL_EMU_REGISTERS_WAVEFORMS][0];
	uint16_t *wf6 = &regs->m_waveform[6 % OPL_EMU_REGISTERS_WAVEFORMS][0];
	uint16_t *wf7 = &regs->m_waveform[7 % OPL_EMU_REGISTERS_WAVEFORMS][0];

	// create the waveforms
	for (uint32_t index = 0; index < OPL_EMU_REGISTERS_WAVEFORM_LENGTH; index++)
		wf0[index] = opl_emu_abs_sin_attenuation(index) | (opl_emu_bitfield(index, 9,1) << 15);

	if (OPL_EMU_REGISTERS_WAVEFORMS >= 4)
	{
		uint16_t zeroval = wf0[0];
		for (uint32_t index = 0; index < OPL_EMU_REGISTERS_WAVEFORM_LENGTH; index++)
		{
			wf1[index] = opl_emu_bitfield(index, 9,1) ? zeroval : wf0[index];
			wf2[index] = wf0[index] & 0x7fff;
			wf3[index] = opl_emu_bitfield(index, 8,1) ? zeroval : (wf0[index] & 0x7fff);
			if (OPL_EMU_REGISTERS_WAVEFORMS >= 8)
			{
				wf4[index] = opl_emu_bitfield(index, 9,1) ? zeroval : wf0[index * 2];
				wf5[index] = opl_emu_bitfield(index, 9,1) ? zeroval : wf0[(index * 2) & 0x1ff];
				wf6[index] = opl_emu_bitfield(index, 9,1) << 15;
				wf7[index] = (opl_emu_bitfield(index, 9,1) ? (index ^ 0x13ff) : index) << 3;
			}
		}
	}
}
void opl_emu_init( struct opl_emu_t* emu ) 
{
	emu->m_env_counter = 0;
	emu->m_status = 0;
	emu->m_timer_running[ 0 ] = 0;
	emu->m_timer_running[ 1 ] = 0;
	emu->m_active_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;
	emu->m_modified_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;
	emu->m_prepare_count = 0;

	opl_emu_registers_init( &emu->m_regs );

	// create the channels
	for (uint32_t chnum = 0; chnum < OPL_EMU_REGISTERS_CHANNELS; chnum++)
		opl_emu_fm_channel_init(&emu->m_channel[chnum], &emu->m_regs, opl_emu_registers_channel_offset(chnum));

	// create the operators
	for (uint32_t opnum = 0; opnum < OPL_EMU_REGISTERS_OPERATORS; opnum++)
		opl_emu_fm_operator_init(&emu->m_operator[opnum],&emu->m_regs, opl_emu_registers_operator_offset(opnum));

	// do the initial operator assignment
	opl_emu_assign_operators( emu );
}