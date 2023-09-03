pub const HQ_TABLE: [u32; 101] = [
	1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8,
	9, 9, 9, 10, 10, 10, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 16, 17,
	17, 17, 18, 18, 18, 19, 19, 20, 20, 21, 22, 23, 24, 26, 28, 31, 34, 38, 42, 47, 52, 58, 64, 68,
	71, 74, 76, 78, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 94, 96, 98, 100,
];

// TODO: clean this up, maybe types to represent Level and ILvl?
pub const fn level_to_ilevel(lvl: u32) -> u32 {
	if lvl <= 50 {
		lvl
	} else if lvl == 51 {
		120
	} else if lvl == 52 {
		125
	} else if lvl == 53 {
		130
	} else if lvl == 54 {
		133
	} else if lvl == 55 {
		136
	} else if lvl == 56 {
		139
	} else if lvl == 57 {
		142
	} else if lvl == 58 {
		145
	} else if lvl == 59 {
		148
	} else if lvl == 60 {
		150
	} else if lvl == 61 {
		260
	} else if lvl == 62 {
		265
	} else if lvl == 63 {
		270
	} else if lvl == 64 {
		273
	} else if lvl == 65 {
		276
	} else if lvl == 66 {
		279
	} else if lvl == 67 {
		282
	} else if lvl == 68 {
		285
	} else if lvl == 69 {
		288
	} else if lvl == 70 {
		290
	} else if lvl == 71 {
		390
	} else if lvl == 72 {
		395
	} else if lvl == 73 {
		400
	} else if lvl == 74 {
		403
	} else if lvl == 75 {
		406
	} else if lvl == 76 {
		409
	} else if lvl == 77 {
		412
	} else if lvl == 78 {
		415
	} else if lvl == 79 {
		418
	} else if lvl == 80 {
		420
	} else if lvl == 81 {
		517
	} else if lvl == 82 {
		520
	} else if lvl == 83 {
		525
	} else if lvl == 84 {
		530
	} else if lvl == 85 {
		535
	} else if lvl == 86 {
		540
	} else if lvl == 87 {
		545
	} else if lvl == 88 {
		550
	} else if lvl == 89 {
		555
	} else if lvl == 90 {
		560
	} else {
		0
	}
}
