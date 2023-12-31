use bitflags::bitflags;

bitflags! {
    // There are a total of 26 cheat codes, saved in a bitmask.
    // All cheat codes unlocked equals 0x3FFFFFF.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct CheatCodes: u32 {
        const INVINCIBILITY = 0x00000001;
        const SPIRIT_MODE = 0x00000002;
        const ALL_WEAPONS = 0x00000004;
        const UNLIMITED_AMMO = 0x00000008;
        const INFINITE_LIVES = 0x00000010;
        const ALL_KEYS = 0x00000020;
        const ALL_MAP = 0x00000040;
        const BIG_HEAD_SESH = 0x00000080;
        const TINY_ENEMIES = 0x00000100;
        const PURDY_COLORS = 0x00000200;
        const DISCO_MODE = 0x00000400;
        const WARP_LEVEL1 = 0x00000800;
        const WARP_LEVEL2 = 0x00001000;
        const WARP_LEVEL3 = 0x00002000;
        const WARP_LEVEL4 = 0x00004000;
        const WARP_LEVEL5 = 0x00008000;
        const WARP_LEVEL6 = 0x00010000;
        const WARP_LEVEL7 = 0x00020000;
        const WARP_LEVEL8 = 0x00040000;
        const WARP_LONGHUNTER = 0x00080000;
        const WARP_MANTIS = 0x00100000;
        const WARP_TREX = 0x00200000;
        const WARP_CAMPAIGNER = 0x00400000;
        const SHOW_CREDITS = 0x00800000;
        const FLY_MODE = 0x01000000;
        const SHOW_ENEMIES = 0x02000000;
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

// Decrypt the cheat codes from the password.
// This is the reverse of the encrypt function.
fn decrypt(bitmask: u32) -> CheatCodes {
    let bitmask = bitmask ^ 0xA5B4C3D2;
    let esi = (bitmask >> 0x8) << 0x18;
    let edx = (bitmask >> 0x8) & 0xFF0000;
    let dh = (bitmask << 0x8) & 0xFF00;
    let dl = (bitmask >> 0x10) & 0xFF;
    let result = esi | edx | dh | dl;
    CheatCodes::from_bits_truncate(result)
}

fn main() {
    let cm_password = encrypt(CheatCodes::all());
    let cheat_codes = decrypt(cm_password);
    println!("cheat_codes: {:?}", cheat_codes);
    println!("cm_password: {cm_password}");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case: All cheat codes unlocked.
    #[test]
    fn test_encrypt_all_cheats() {
        let cm_password = encrypt(CheatCodes::all());
        assert_eq!(cm_password, 1514913837);
    }

    // Test case: No cheat codes unlocked.
    #[test]
    fn test_encrypt_no_cheats() {
        let cm_password = encrypt(CheatCodes::empty());
        assert_eq!(cm_password, 2780087250);
    }

    // Test case: All warp cheat codes unlocked.
    #[test]
    fn test_encrypt_all_warp_cheats() {
        let cm_password = encrypt(
            CheatCodes::WARP_LEVEL1
                | CheatCodes::WARP_LEVEL2
                | CheatCodes::WARP_LEVEL3
                | CheatCodes::WARP_LEVEL4
                | CheatCodes::WARP_LEVEL5
                | CheatCodes::WARP_LEVEL6
                | CheatCodes::WARP_LEVEL7
                | CheatCodes::WARP_LEVEL8
                | CheatCodes::WARP_LONGHUNTER
                | CheatCodes::WARP_MANTIS
                | CheatCodes::WARP_TREX
                | CheatCodes::WARP_CAMPAIGNER,
        );
        assert_eq!(cm_password, 3669279530);
    }

    // Test case: All cheat codes unlocked.
    #[test]
    fn test_decrypt_all_cheats() {
        let cheat_codes = decrypt(1514913837);
        assert_eq!(cheat_codes, CheatCodes::all());
    }

    // Test case: No cheat codes unlocked.
    #[test]
    fn test_decrypt_no_cheats() {
        let cheat_codes = decrypt(2780087250);
        assert_eq!(cheat_codes, CheatCodes::empty());
    }

    // Test case: All warp cheat codes unlocked.
    #[test]
    fn test_decrypt_all_warp_cheats() {
        let cheat_codes = decrypt(3669279530);
        assert_eq!(
            cheat_codes,
            CheatCodes::WARP_LEVEL1
                | CheatCodes::WARP_LEVEL2
                | CheatCodes::WARP_LEVEL3
                | CheatCodes::WARP_LEVEL4
                | CheatCodes::WARP_LEVEL5
                | CheatCodes::WARP_LEVEL6
                | CheatCodes::WARP_LEVEL7
                | CheatCodes::WARP_LEVEL8
                | CheatCodes::WARP_LONGHUNTER
                | CheatCodes::WARP_MANTIS
                | CheatCodes::WARP_TREX
                | CheatCodes::WARP_CAMPAIGNER
        );
    }
}
