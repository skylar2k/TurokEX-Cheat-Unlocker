# Turok cheat unlocker
Turok saves unlocked cheats using an ConVar `cm_password`.
The value of `cm_password` is a XOR encrypted bitmask, this project creates valid codes to unlock specific cheats.
It also supports decrypting an existing `cm_password` value.