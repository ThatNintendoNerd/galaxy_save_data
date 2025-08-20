# Game Event Values

A Game Event Value is a key-value pair for a 16-bit unsigned integer.

## ペンギンレース[オーシャンリング]/hi

> Penguin Race [Ocean Ring]/hi

Represents the most significant bytes for the Sea Slide Galaxy's best race time, in frames.

The default value is `0`.

## ペンギンレース[オーシャンリング]/lo

> Penguin Race [Ocean Ring]/lo

Represents the least significant bytes for the Sea Slide Galaxy's best race time, in frames.

The default value is `5400`, or a time of `01:30:00`.

## テレサレース[ファントム]/hi

> Teresa Race [Phantom]/hi

Represents the most significant bytes for the Ghostly Galaxy's best race time, in frames.

The default value is `0`.

## テレサレース[ファントム]/lo

> Teresa Race [Phantom]/lo

Represents the least significant bytes for the Ghostly Galaxy's best race time, in frames.

The default value is `5400`, or a time of `01:30:00`.

## テレサレース[デスプロムナード]/hi

> Teresa Race [Death Promenade]/hi

Represents the most significant bytes for the Boo's Boneyard Galaxy's best race time, in frames.

The default value is `0`.

## テレサレース[デスプロムナード]/lo

> Teresa Race [Death Promenade]/lo

Represents the least significant bytes for the Boo's Boneyard Galaxy's best race time, in frames.

The default value is `5400`, or a time of `01:30:00`.

## サーフィン[トライアル]/hi

> Surfing [Trial]/hi

Represents the most significant bytes for the Loopdeeloop Galaxy's best race time, in frames.

The default value is `0`.

## サーフィン[トライアル]/lo

> Surfing [Trial]/lo

Represents the least significant bytes for the Loopdeeloop Galaxy's best race time, in frames.

The default value is `5400`, or a time of `01:30:00`.

## サーフィン[チャレンジ]/hi

> Surfing [Challenge]/hi

Represents the most significant bytes for the Loopdeeswoop Galaxy's best race time, in frames.

The default value is `0`.

## サーフィン[チャレンジ]/lo

> Surfing [Challenge]/lo

Represents the least significant bytes for the Loopdeeswoop Galaxy's best race time, in frames.

The default value is `5400`, or a time of `01:30:00`.

## LibraryOpenNewStarCount

Represents the remaining number of Stars to collect after completing Bowser's Star Reactor before Rosalina's library can open.

The default value is `1`.

## 絵本既読章

> Picture Book Chapters Already Read

Represents the number of chapters read in Rosalina's Story.

The default value is `0`.

## MsgLedPattern

> [!NOTE]
> The following Game Event Value only exists in the Korean version of the game.

Represents a Boolean determining if the disc slot should illuminate when the mailtoad receives a letter.

The default value is `1`.

## LuigiEventState

Represents the state of Luigi's game of hide-and-seek.

The default value is `0xFF00`.

## WarpPodSaveBits

Represents a bit field determining if an optional Warp Pad was activated.

The default value is `0b00000000_00000000`.

## TicoGalaxyAlreadyTalk

Represents a bit field determining if a Hungry Luma in the Comet Observatory was talked to.

Hungry Lumas are arbitrarily assigned a bit index based on their objects' `Obj_arg7` value. The bits are reserved in the vanilla game as follows:

```
0b00000000_00000001 => The entrance to the Terrace
0b00000000_00000010 => The entrance to the Fountain
0b00000000_00000100 => The entrance to the Kitchen
0b00000000_00001000 => The Gate
0b00000000_00010000 => The entrance to the Garden
0b00000000_00100000 => The entrance to the Engine Room
0b00000000_01000000 => The entrance to the Bedroom
```

The default value is `0b00000000_00000000`.

## MessageAlreadyRead

Represents a bit field determining if a message from an actor was read by the player.

Messages are arbitrarily assigned a bit index based on their last parameter. The bits are reserved in the vanilla game as follows:

```
0b00000000_00000001 => DiskGardenZone_Tico006 (Talking to the Yellow Luma on the starting planet of the Good Egg Galaxy's second mission)
0b00000000_00000010 => AstroGalaxy_Kinopio083 (Talking to the mailtoad after collecting the Grand Star from Bowser Jr.'s Robot Reactor)
0b00000000_00000010 => AstroGalaxy_Kinopio095 (Talking to the mailtoad when he first has a letter for the player character)
0b00000000_00000100 => AstroGalaxy_Kinopio008 (Talking to Captain Toad before collecting the Grand Star from Bowser's Star Reactor)
0b00000000_00001000 => AstroGalaxy_Kinopio081 (Talking to the mailtoad before collecting the Grand Star from Bowser Jr.'s Robot Reactor)
```

The default value is `0b00000000_00000000`.

## MissPointForLetter

Represents the number of points earned from losing lives and getting game overs which go towards receiving a letter from Princess Peach.

This value will increase by one when a life is lost and increases by three from a game over but is clamped to the interval [0, 20]. The mailtoad will receive a letter from Princess Peach when this value equals its maximum and resets to zero after speaking to him about it.

The default value is `0`.

## MissNum

Represents the number of lives lost.

This value is clamped to the interval [0, 9999].

The default value is `0`.

## Comet1Status

Represents the state of the Prankster Comet in the Terrace.

- The least significant bit determines if the Prankster Comet is in orbit.
- The most significant 15 bits represent the index into `sCometTimeTableGrandGalaxy1`.

The default value is `0`.

## Comet2Status

Represents the state of the Prankster Comet in the Fountain.

- The least significant bit determines if the Prankster Comet is in orbit.
- The most significant 15 bits represent the index into `sCometTimeTableGrandGalaxy2`.

The default value is `0`.

## Comet3Status

Represents the state of the Prankster Comet in the Kitchen.

- The least significant bit determines if the Prankster Comet is in orbit.
- The most significant 15 bits represent the index into `sCometTimeTableGrandGalaxy3`.

The default value is `0`.

## Comet4Status

Represents the state of the Prankster Comet in the Bedroom.

- The least significant bit determines if the Prankster Comet is in orbit.
- The most significant 15 bits represent the index into `sCometTimeTableGrandGalaxy4`.

The default value is `0`.

## Comet5Status

Represents the state of the Prankster Comet in the Engine Room.

- The least significant bit determines if the Prankster Comet is in orbit.
- The most significant 15 bits represent the index into `sCometTimeTableGrandGalaxy5`.

The default value is `0`.

## Comet6Status

Represents the state of the Prankster Comet in the Garden.

- The least significant bit determines if the Prankster Comet is in orbit.
- The most significant 15 bits represent the index into `sCometTimeTableGrandGalaxy6`.

The default value is `0`.
