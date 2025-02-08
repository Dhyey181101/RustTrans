package test

const (
	geo_s2_MaxLevel = 30
)

func geo_s2_areSiblings(a, b, c, d geo_s2_CellID) bool {
	// A necessary (but not sufficient) condition is that the XOR of the
	// four cell IDs must be zero. This is also very fast to test.
	if (a ^ b ^ c) != d {
		return false
	}

	// Now we do a slightly more expensive but exact test. First, compute a
	// mask that blocks out the two bits that encode the child position of
	// "id" with respect to its parent, then check that the other three
	// children all agree with "mask".
	mask := d.lsb() << 1
	mask = ^(mask + (mask << 1))
	idMasked := (uint64(d) & mask)
	return ((uint64(a)&mask) == idMasked &&
		(uint64(b)&mask) == idMasked &&
		(uint64(c)&mask) == idMasked &&
		!d.isFace())
}

func (ci geo_s2_CellID) lsb() uint64 { return uint64(ci) & -uint64(ci) }

func (ci geo_s2_CellID) isFace() bool { return uint64(ci)&(geo_s2_lsbForLevel(0)-1) == 0 }

func geo_s2_lsbForLevel(level int) uint64 { return 1 << uint64(2*(geo_s2_MaxLevel-level)) }

type geo_s2_CellID uint64
