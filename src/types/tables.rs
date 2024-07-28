use crate::types::structs::CraftingLevel;

pub const HQ_TABLE: [u32; 101] = [
	1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8,
	9, 9, 9, 10, 10, 10, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 16, 17,
	17, 17, 18, 18, 18, 19, 19, 20, 20, 21, 22, 23, 24, 26, 28, 31, 34, 38, 42, 47, 52, 58, 64, 68,
	71, 74, 76, 78, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 94, 96, 98, 100
];

pub fn level_to_ilevel(lvl: CraftingLevel) -> u32 {
	let arr = [
		0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
		25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
		48, 49, 50, 120, 125, 130, 133, 136, 139, 142, 145, 148, 150, 260, 265, 270, 273, 276, 279,
		282, 285, 288, 290, 390, 395, 400, 403, 406, 409, 412, 415, 418, 420, 517, 520, 525, 530,
		535, 540, 545, 550, 555, 560, 650, 653, 656, 660, 665, 670, 675, 680, 685, 690,
	];

	*arr.get::<usize>(Into::<u8>::into(lvl).into()).unwrap()
}
