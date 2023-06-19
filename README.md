# Turok cheat unlocker
Turok saves unlocked cheats using an ConVar `cm_password`.
The value of `cm_password` is a XOR encrypted bitmask, this project creates valid codes to unlock specific cheats.
It also supports decrypting an existing `cm_password` value.

TODO:
- Look at maybe replacing some bitwise operations with stuff like [u32.to_be_bytes()](https://doc.rust-lang.org/std/primitive.u32.html#method.to_be_bytes)
