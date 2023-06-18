use bitflags::bitflags;

bitflags! {
    // There are a total of 26 cheat codes, saved in a bitmask.
    // All cheat codes unlocked equals 0x3FFFFFF.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct CheatCodes: u32 {
        const INVINCIBILITY = 0b00_00000000_00000000_00000001;
        const SPIRIT_MODE = 0b00_00000000_00000000_00000010;
        const ALL_WEAPONS = 0b00_00000000_00000000_00000100;
        const UNLIMITED_AMMO = 0b00_00000000_00000000_00001000;
        const INFINITE_LIVES = 0b00_00000000_00000000_00010000;
        const ALL_KEYS = 0b00_00000000_00000000_00100000;
        const ALL_MAP = 0b00_00000000_00000000_01000000;
        const BIG_HEAD_SESH = 0b00_00000000_00000000_10000000;
        const TINY_ENEMIES = 0b00_00000000_00000001_00000000;
        const PURDY_COLORS = 0b00_00000000_00000010_00000000;
        const DISCO_MODE = 0b00_00000000_00000100_00000000;
        const WARP_LEVEL1 = 0b00_00000000_00001000_00000000;
        const WARP_LEVEL2 = 0b00_00000000_00010000_00000000;
        const WARP_LEVEL3 = 0b00_00000000_00100000_00000000;
        const WARP_LEVEL4 = 0b00_00000000_01000000_00000000;
        const WARP_LEVEL5 = 0b00_00000000_10000000_00000000;
        const WARP_LEVEL6 = 0b00_00000001_00000000_00000000;
        const WARP_LEVEL7 = 0b00_00000010_00000000_00000000;
        const WARP_LEVEL8 = 0b00_00000100_00000000_00000000;
        const WARP_LONGHUNTER = 0b00_00001000_00000000_00000000;
        const WARP_MANTIS = 0b00_00010000_00000000_00000000;
        const WARP_TREX = 0b00_00100000_00000000_00000000;
        const WARP_CAMPAIGNER = 0b00_01000000_00000000_00000000;
        const SHOW_CREDITS = 0b00_10000000_00000000_00000000;
        const FLY_MODE = 0b01_00000000_00000000_00000000;
        const SHOW_ENEMIES = 0b10_00000000_00000000_00000000;
    }
}

fn encrypt(bitmask: CheatCodes) -> u32 {
    // Let's say all cheat codes are unlocked.
    // Then the bitmask will have 26 bits set to 1, this equals 0x3FFFFFF.
    // mov edx, 0x3FFFFFF
    // mov esi, edx
    // The game will extract the most significant byte by shifting the bits 0x18 to the right.
    // shr esi, 0x18
    // And then shifts the result to the second least significant byte.
    // shl esi, 0x8
    let esi = (bitmask.bits() >> 0x18) << 0x8;
    // The game gets the second most significant byte and shifts it to the most significant byte.
    // and edx, 0xFF0000
    // shl edx, 0x8
    let edx = (bitmask.bits() & 0xFF0000) << 0x8;
    // The game then moves DH into eax, and zero extends it.
    // movzx eax, dh
    let dh = (bitmask.bits() & 0xFF00) >> 0x8;
    // The game then moves DL into eax, and zero extends it.
    // movzx eax, dl
    // shl eax, 0x10
    let dl = (bitmask.bits() & 0xFF) << 0x10;
    // The game then ORs the result of the previous operations.
    let result = esi | edx | dh | dl;
    // The resulting value is XORed with 0xA5B4C3D2.
    result ^ 0xA5B4C3D2
}

// Test case: All cheat codes unlocked.
#[test]
fn test_all_cheats() {
    let cm_password = encrypt(CheatCodes::all());
    assert_eq!(cm_password, 1514913837);
}
// Test case: No cheat codes unlocked.
#[test]
fn test_no_cheats() {
    let cm_password = encrypt(CheatCodes::empty());
    assert_eq!(cm_password, 2780087250);
}

fn main() {
    let cm_password = encrypt(CheatCodes::all());
    println!("cm_password: {cm_password}");
}
