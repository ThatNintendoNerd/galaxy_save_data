`GameData.bin` is the name of Super Mario Galaxy 2's save file. On Wii, it is stored in `/title/00010000/534234##/data`, where `##` is the plaintext hexadecimal value for the letter that corresponds to the game's region (either SB4**J**, SB4**E**, SB4**P**, SB4**W**, or SB4**K**).

# File Format

The save file has a fixed size of 0x30A0 (or 12,448 in decimal) bytes but can technically support sizes under 0x8000 (or 32,768 in decimal) bytes. While there is no required order for user files or their contained data blocks for reading, the vanilla game will establish its own order when writing. This document outlines ordering based on how the vanilla game writes the save file.

## Header

The header contains the supplementary information of the save file.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u32 | The precomputed checksum derived from the file data, excluding the checksum itself. |
| 0x04 | u32 | The version number of the save data. |
| 0x08 | u32 | The number of stored user file descriptors. |
| 0x0C | u32 | The size of the file, in bytes. |

## User File Info

Each file is split into two distinct user files: one for gameplay data and one for what was once shared data between all associated files.

- The gameplay data of the file is named `user#`, where `#` is the plaintext positive number of the file.
- The shared data of the file is named `config#`, where `#` is the plaintext positive number of the file.

The shared data between all files is its own user file named `sysconf`.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | char[12] | The null-terminated name of the user file. |
| 0x0C | u32 | The offset to the user file data in bytes, relative to the start of the file data. |

## user

A `user` user file stores gameplay data.

This data block container is assigned a data buffer of 0xF80 (or 3,968 in decimal) bytes.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderHeader](#binarydatachunkholderheader) | The supplementary information for the data block container. |
| 0x04 | [BinaryDataChunkBase](#binarydatachunkbase)[] | The array of associated data blocks. |

### PLAY

A data block dedicated to preserving player state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | [BinaryDataContentHeader](#binarydatacontentheader) | The supplementary information for the dynamic reader/writer. |
| 0x10 | [BinaryDataContentAttribute](#binarydatacontentattribute)[5] | The array of field descriptors for use with the dynamic reader/writer. |
| 0x24 | u8 | The number of remaining lives. |
| 0x25 | u16 | The number of stashed Star Bits. |
| 0x27 | u16 | The number of stashed coins. |
| 0x29 | u16 | The most recent number of stashed coins to have awarded the player an extra life. |
| 0x2B.0 | 1 bit | Determines if Luigi is the current player character. |
| 0x2B.1 | 7 bits | **Unused**. |

### FLG1

A data block dedicated to preserving key-value pair state, where each value is a Boolean.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | [GameEventFlag](#gameeventflag)[] | The array of key-value pairs. |

#### GameEventFlag

A key-value pair for a Boolean.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00.0 | 15 bits | The hashed key, truncated to the least significant 15 bits. |
| 0x00.15 | 1 bit | The associated value. |

### STF1

A data block dedicated to preserving Hungry Luma state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | u16[8][6] | The matrix of Star Bit counters, where each row corresponds to a world and each column corresponds to an individual Hungry Luma's consumed number of Star Bits. |
| 0x6C | u16[16] | The array of hashed internal galaxy names with satisfied coin-dependent Hungry Lumas, truncated to the least significant 16 bits. |

### VLE1

A data block dedicated to preserving key-value pair state, where each value is a 16-bit unsigned integer.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | [GameEventValue](#gameeventvalue)[] | The array of key-value pairs. |

#### GameEventValue

A key-value pair for a 16-bit unsigned integer.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u16 | The hashed key, truncated to the least significant 16 bits. |
| 0x02 | u16 | The associated value. |

### GALA

A data block dedicated to preserving galaxy state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | u16 | The number of stored galaxy states. |
| 0x0E | [BinaryDataContentHeader](#binarydatacontentheader) | The supplementary information for the dynamic reader/writer. |
| 0x12 | [BinaryDataContentAttribute](#binarydatacontentattribute)[5] | The array of galaxy field descriptors for use with the dynamic reader/writer. |
| 0x26 | [BinaryDataContentHeader](#binarydatacontentheader) | The supplementary information for the dynamic reader/writer. |
| 0x2A | [BinaryDataContentAttribute](#binarydatacontentattribute)[3] | The array of mission field descriptors for use with the dynamic reader/writer. |
| 0x36 | [SaveDataStorageGalaxyStage](#savedatastoragegalaxystage)[] | The array of galaxy states. |

#### SaveDataStorageGalaxyStage

A container dedicated to preserving the state of a galaxy.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u16 | The hashed internal name of the galaxy, truncated to the least significant 16 bits. |
| 0x02 | u16 | The size of the serialized struct, in bytes. |
| 0x04 | u8 | The number of stored missions. |
| 0x05 | u8 | The unit state on the World Map. |
| 0x06.0 | 1 bit | Determines if the Comet Medal has been collected. |
| 0x06.1 | 1 bit | Determines if a Prankster Comet is in orbit. |
| 0x06.2 | 6 bits | **Unused**. |
| 0x07 | [SaveDataStorageGalaxyScenario](#savedatastoragegalaxyscenario)[] | The array of mission states. |

#### SaveDataStorageGalaxyScenario

A container dedicated to preserving the state of a mission.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u8 | The number of lives lost. |
| 0x01 | u32 | The best clear time, in frames. |
| 0x02.0 | 1 bit | Determines if the Star has been collected. |
| 0x02.1 | 1 bit | Determines if the Bronze Star has been collected. |
| 0x02.2 | 1 bit | Determines if the mission has been selected before. |
| 0x02.3 | 1 bit | Determines if a ghost can appear. |
| 0x02.4 | 1 bit | Determines if Luigi has ever appeared on standby. |
| 0x02.5 | 3 bits | **Unused**. |

### SSWM

A data block dedicated to preserving World Map state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | u8[8] | The flags representing the Star Barrier passage status for each world. |
| 0x14 | u8 | The positive world number currently being navigated. |

## config

A `config` user file stores shared data between all associated user files.

This data block container is assigned a data buffer of 0x60 (or 96 in decimal) bytes.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderHeader](#binarydatachunkholderheader) | The supplementary information for the data block container. |
| 0x04 | [BinaryDataChunkBase](#binarydatachunkbase)[] | The array of associated data blocks. |

### CONF

A data block dedicated to preserving user file creation state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | u8 | Determines if the user file currently exists. |

### MII

A data block dedicated to preserving user file icon state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C.0 | 1 bit | **Unused**. Removed in Super Mario Galaxy 2. |
| 0x0C.1 | 1 bit | **Unused**. Never tested or cleared; only set if the icon represents a Mii. |
| 0x0C.2 | 6 bits | **Unused**. |
| 0x0D | [RFLCreateID](#rflcreateid) | The unique identifier of the Mii. |
| 0x15 | u8 | The icon of the user file. |

#### RFLCreateID

A unique identifier for a Mii.

This data type originates from the Revolution Face Library.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00.7 | 1 bit | Determines if the Mii was not distributed by Nintendo. |
| 0x00.6 | 1 bit | Determines if the Mii was created on another system. |
| 0x00.5 | 1 bit | **Unknown**. |
| 0x00.4 | 1 bit | **Unused**. |
| 0x00.3 | 28 bits | The timestamp representing when the Mii character's gender was selected. |
| 0x04 | u8 | The checksum derived from the organizationally unique identifier of the source console's MAC address. |
| 0x05 | u8[3] | The device identifier from the source console's MAC address. |

### MISC

A data block dedicated to preserving miscellaneous user file state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | [OSTime](#ostime) | The timestamp representing when the user file was most recently saved. |

## sysconf

A `sysconf` user file stores shared data between all user files.

This data block container is assigned a data buffer of 0x80 (or 128 in decimal) bytes.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderHeader](#binarydatachunkholderheader) | The supplementary information for the data block container. |
| 0x04 | [BinaryDataChunkBase](#binarydatachunkbase)[] | The array of associated data blocks. |

### SYSC

A data block dedicated to preserving shared state between all user files.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | [BinaryDataContentHeader](#binarydatacontentheader) | The supplementary information for the dynamic reader/writer. |
| 0x10 | [BinaryDataContentAttribute](#binarydatacontentattribute)[7] | The array of field descriptors for use with the dynamic reader/writer. |
| 0x2C | bool | Determines if the player was encouraged to change their TV Type from 50 Hz to 60 Hz. |
| 0x2D | [OSTime](#ostime) | The timestamp representing when the most recent message was sent to the Wii Message Board. |
| 0x35 | u32 | The number of bytes sent to the Wii Message Board from the date represented in the previous field. |
| 0x39 | u16 | The number of banked Star Bits. |
| 0x3B | u16 | The greatest number of banked Star Bits. |
| 0x3D | u8 | The number of extra lives from another user file attached to a letter from Rosalina.
| 0x3E | u16 | The sender of extra lives' hashed user file name, truncated to the least significant 16 bits. |

# Common Data Types

## Data Parsing

### BinaryDataChunkBase

The abstract class for types to be represented as a serializable and deserializable block of data.

### BinaryDataChunkHolderHeader

The supplementary information for a container of data blocks.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u8 | The version number of the data block container. |
| 0x01 | u8 | The number of stored data blocks. |
| 0x02 | u8[2] | **Unused**. |

### BinaryDataChunkHolderChunkData

The supplementary information for a block of data.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u32 | The magic number identifying the data block. |
| 0x04 | u32 | The hash digest identifying the data block. |
| 0x08 | u32 | The size of the data block, in bytes. |

### BinaryDataContentHeader

The supplementary information for a container of field descriptors.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u16 | The number of stored field descriptors. |
| 0x02 | u16 | The size of the serialized field data, in bytes. |

### BinaryDataContentAttribute

The descriptor for a field stored in a data block.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u16 | The hashed name of the field, truncated to the least significant 16 bits. |
| 0x02 | u16 | The offset to the field in bytes, relative to the start of the serialized field data. |

## Time

### OSTime

A type alias for an `s64` representing the number of ticks since the Revolution OS epoch; 2000-01-01 00:00:00.

This data type originates from the OS library in the Revolution SDK.
