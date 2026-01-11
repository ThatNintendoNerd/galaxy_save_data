`GameData.bin` is the name of Super Mario Galaxy's save file. On Wii, it is stored in `/title/00010000/524d47##/data`, where `##` is the plaintext hexadecimal value for the letter that corresponds to the game's region (either RMG**J**, RMG**E**, RMG**P**, or RMG**K**).

# Platform Differences

Super Mario Galaxy was originally released for Wii in 2007, later rereleased for NVIDIA Shield TV in 2018 and for Nintendo Switch in 2020 as part of Super Mario 3D All-Stars. Consequently, changes in CPU architecture warrant changes in data format.

## Endianness

The Wii is built around the PowerPC architecture in big-endian mode.

The Nintendo Switch is built around the ARM architecture in little-endian mode.

Assume the suggested endianness for the corresponding platform when reading the documentation.

## Alignment

On Wii, all data is packed with little to no effort made towards data alignment.

On Nintendo Switch, each data block is now padded to the next four-byte boundary.

## Time

On Wii, time is tracked using a register which increments every few CPU cycles, allowing for both incredible precision and accuracy. See [OSTime](#ostime) for more information.

On Nintendo Switch, time is now tracked on a per-second basis. See [PosixTime](#posixtime) for more information.

# File Format

The save file has a fixed size of 0xBE00 (or 48,640 in decimal) bytes but can technically support sizes under 0x10000 (or 65,536 in decimal) bytes. While there is no required order for user files or their contained data blocks for reading, the vanilla game will establish its own order when writing. This document outlines ordering based on how the vanilla game writes the save file.

## Header

The header contains the supplementary information of the save file.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u32 | The precomputed checksum derived from the file data, excluding the checksum itself. |
| 0x04 | u32 | The version number of the save data. |
| 0x08 | u32 | The number of stored user file descriptors. |
| 0x0C | u32 | The size of the file, in bytes. |

## User File Info

Each file is split into three distinct user files: one for Mario's part of the file, one for Luigi's part of the file, and one for shared data between them both.

- Mario's part of the file is named `mario#`, where `#` is the plaintext positive number of the file.
- Luigi's part of the file is named `luigi#`, where `#` is the plaintext positive number of the file.
- The shared data of the file is named `config#`, where `#` is the plaintext positive number of the file.

The shared data between all files is its own user file named `sysconf`.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | char[12] | The null-terminated name of the user file. |
| 0x0C | u32 | The offset to the user file data in bytes, relative to the start of the file data. |

## mario/luigi

A `mario` or `luigi` user file stores gameplay data.

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
| 0x0C | u8 | The value of sequential progression through the story. |
| 0x0D | u32 | The number of stashed Star Bits. |
| 0x12 | u16 | The number of remaining lives. |

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

### PCE1

A data block dedicated to preserving Hungry Luma state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | u16[2][8] | The matrix of Star Bit counters, where each row corresponds to individual galaxies and the Comet Observatory, respectively, and each column corresponds to an individual Hungry Luma's consumed number of Star Bits. |

### SPN1

A data block dedicated to preserving Launch Star path state.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | [BinaryDataChunkHolderChunkData](#binarydatachunkholderchunkdata) | The supplementary information for the data block. |
| 0x0C | u8 | The number of stored galaxies. |
| 0x0D | [SpinDriverPathStorageGalaxy](#spindriverpathstoragegalaxy)[] | The array of associated galaxies. |

#### SpinDriverPathStorageGalaxy

A container dedicated to preserving Launch Star path state in a galaxy.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u16 | The hashed internal name of the galaxy, truncated to the least significant 16 bits. |
| 0x02 | u16 | The size of the serialized struct, in bytes. |
| 0x04 | u8 | The number of stored base missions. |
| 0x05 | u8 | **Unused**. |
| 0x06 | [SpinDriverPathStorageScenario](#spindriverpathstoragescenario)[] | The array of associated base missions. |

#### SpinDriverPathStorageScenario

A container dedicated to preserving Launch Star path state in a base mission.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u16 | The size of the serialized struct, in bytes. |
| 0x02 | u8[] | TODO, terminated by a value of `0xFF`. |

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
| 0x12 | [BinaryDataContentAttribute](#binarydatacontentattribute)[4] | The array of galaxy field descriptors for use with the dynamic reader/writer. |
| 0x22 | [GameDataSomeGalaxyStorage](#gamedatasomegalaxystorage)[] | The array of galaxy states. |

#### GameDataSomeGalaxyStorage

A container dedicated to preserving the state of a galaxy.

| Offset | Type | Description |
| --- | --- | --- |
| 0x00 | u16 | The hashed internal name of the galaxy, truncated to the least significant 16 bits. |
| 0x02 | u8 | The flags representing the Star collection status for each mission. |
| 0x03 | u8 | The flags representing the selection status for each base mission. |
| 0x04 | u16[8] | The greatest number of collected coins for each mission. |

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
| 0x0C.0 | 1 bit | **Unused**. Never set or cleared; only tested if the last field is absent. |
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
| 0x0C.0 | 1 bit | Determines if Mario was the most recently played character in the file. |
| 0x0C.1 | 1 bit | Determines if Mario has completed his part of the file. |
| 0x0C.2 | 1 bit | Determines if Luigi has completed his part of the file. |
| 0x0C.3 | 5 bits | **Unused**. |
| 0x0D | [OSTime](#ostime) (Wii)<br>[PosixTime](#posixtime) (Nintendo Switch) | The timestamp representing when the user file was most recently saved. |

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
| 0x10 | [BinaryDataContentAttribute](#binarydatacontentattribute)[3] | The array of field descriptors for use with the dynamic reader/writer. |
| 0x1C | [OSTime](#ostime) (Wii)<br>[PosixTime](#posixtime) (Nintendo Switch) | The timestamp representing when the player was encouraged to change their TV Type from 50 Hz to 60 Hz. |
| 0x24 | [OSTime](#ostime) (Wii)<br>[PosixTime](#posixtime) (Nintendo Switch) | The timestamp representing when the most recent message was sent to the Wii Message Board. |
| 0x2C | u32 | The number of bytes sent to the Wii Message Board from the date represented in the previous field. |

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

> [!NOTE]
> The following information is only applicable to Wii.

A type alias for an `s64` representing the number of ticks since the Revolution OS epoch; 2000-01-01 00:00:00.

This data type originates from the OS library in the Revolution SDK.

### PosixTime

> [!NOTE]
> The following information is only applicable to Nintendo Switch.

A wrapper for an `int64_t` representing the number of non-leap seconds since the Unix epoch; 1970-01-01 00:00:00 UTC.

This data type originates from the time library in `nn`.
