# Game Event Values

A Game Event Value is a key-value pair for a 16-bit unsigned integer.

The default value for all values is `0`.

## グライダー[ジャングル]/hi

> Glider [Jungle]/hi

Represents the most significant bytes for the best race time in the Wild Glide Galaxy, in frames.

## グライダー[ジャングル]/lo

> Glider [Jungle]/lo

Represents the least significant bytes for the best race time in the Wild Glide Galaxy, in frames.

## グライダー[チャレンジ]/hi

> Glider [Challenge]/hi

Represents the most significant bytes for the best race time in the Fleet Glide Galaxy, in frames.

## グライダー[チャレンジ]/lo

> Glider [Challenge]/lo

Represents the least significant bytes for the best race time in the Fleet Glide Galaxy, in frames.

## ベストスコア[MokumokuValleyGalaxy]/lo

> Best Score [MokumokuValleyGalaxy]/lo

Represents the least significant bytes for the high score in the second mission of the Fluffy Bluff Galaxy.

## ベストスコア[MokumokuValleyGalaxy]/hi

> Best Score [MokumokuValleyGalaxy]/hi

Represents the most significant bytes for the high score in the second mission of the Fluffy Bluff Galaxy.

## ベストスコア[HoneyBeeVillageGalaxy]/lo

> Best Score [HoneyBeeVillageGalaxy]/lo

Represents the least significant bytes for the high score in the second mission of the Honeyhop Galaxy.

## ベストスコア[HoneyBeeVillageGalaxy]/hi

> Best Score [HoneyBeeVillageGalaxy]/hi

Represents the most significant bytes for the high score in the second mission of the Honeyhop Galaxy.

## ベストスコア[UnderGroundDangeonGalaxy]/lo

> Best Score [UnderGroundDangeonGalaxy]/lo

Represents the least significant bytes for the high score in the second mission of the Slimy Spring Galaxy.

## ベストスコア[UnderGroundDangeonGalaxy]/hi

> Best Score [UnderGroundDangeonGalaxy]/hi

Represents the most significant bytes for the high score in the second mission of the Slimy Spring Galaxy.

## ベストスコア[TwisterTowerGalaxy]/lo

> Best Score [TwisterTowerGalaxy]/lo

Represents the least significant bytes for the high score in the third mission of the Melty Monster Galaxy.

## ベストスコア[TwisterTowerGalaxy]/hi

> Best Score [TwisterTowerGalaxy]/hi

Represents the most significant bytes for the high score in the third mission of the Melty Monster Galaxy.

## ベストスコア[KachikochiLavaGalaxy]/lo

> Best Score [KachikochiLavaGalaxy]/lo

Represents the least significant bytes for the high score in the third mission of the Shiverburn Galaxy.

## ベストスコア[KachikochiLavaGalaxy]/hi

> Best Score [KachikochiLavaGalaxy]/hi

Represents the most significant bytes for the high score in the third mission of the Shiverburn Galaxy.

## ベストスコア[WhiteSnowGalaxy]/lo

> Best Score [WhiteSnowGalaxy]/lo

Represents the least significant bytes for the high score in the third mission of the Freezy Flake Galaxy.

## ベストスコア[WhiteSnowGalaxy]/hi

> Best Score [WhiteSnowGalaxy]/hi

Represents the most significant bytes for the high score in the third mission of the Freezy Flake Galaxy.

## 郵便屋[タスク手紙既読フラグ]/0

> Postman [Task Letter Read Flag]/0

Represents the least significant bytes for a bit field determining if a letter informing the player about a conditionally unlocked mission was received by the mailtoad.

```
0b00000000_01000000 => PostmanLetterCaretaker1
0b00000000_10000000 => PostmanLetterCaretaker2
0b00000001_00000000 => PostmanLetterPichan1
0b00000010_00000000 => PostmanLetterPichan2
0b00000100_00000000 => PostmanLetterScoreAttack1
0b00001000_00000000 => PostmanLetterScoreAttack2
0b00010000_00000000 => PostmanLetterScoreAttack3
```

## 郵便屋[タスク手紙既読フラグ]/1

> Postman [Task Letter Read Flag]/1

Represents the most significant bytes for a bit field determining if a letter informing the player about a conditionally unlocked mission was received by the mailtoad.

## 郵便屋[重要手紙既読フラグ]/0

> Postman [Important Letter Read Flag]/0

Represents the least significant bytes for a bit field determining if a letter gifting the player was received by the mailtoad.

```
0b00000000_00000001 => PostmanLetterPeach
0b00000000_00000010 => PostmanLetterLuigi
0b00000000_00000100 => PostmanLetterPeach (unused?)
0b00000000_00001000 => PostmanLetterWitch
0b00000000_00010000 => PostmanLetterBee
0b00000000_00100000 => PostmanLetterRabbit
0b00100000_00000000 => PostmanLetterRosetta1
0b01000000_00000000 => PostmanLetterRosetta2
0b10000000_00000000 => PostmanLetterRosetta3
```

## 郵便屋[重要手紙既読フラグ]/1

> Postman [Important Letter Read Flag]/1

Represents the most significant bytes for a bit field determining if a letter gifting the player was received by the mailtoad.

```
0b00000000_00000001 => PostmanLetterRosetta4
0b00000000_00000010 => PostmanLetterRosetta5
```

## 郵便屋[最後に読んだタスク手紙インデックス]

> Postman [Last Read Task Letter Index]

Represents the bit index of the most recent letter received by the mailtoad informing the player about a conditionally unlocked mission.

## 郵便屋[ピーチ手紙を読んだ時の累積死亡回数]

> Postman [Cumulative Number of Deaths When Reading Peach's Letter]

Represents the number of lives lost as of reading Princess Peach's letter.

> [!NOTE]
> This value effectively goes unused as it is never read from; only written to with the value of [`累積死亡回数`](#累積死亡回数).

## メッセージ既読フラグ/0

> Message Already Read Flag/0

Represents the least significant bytes for a bit field determining if a message from an actor was read by the player.

Messages are assigned a bit index based on their last parameter.

## メッセージ既読フラグ/1

> Message Already Read Flag/1

Represents the most significant bytes for a bit field determining if a message from an actor was read by the player.

Messages are assigned a bit index based on their last parameter.

## 累積死亡回数

> Cumulative Number of Deaths

Represents the number of lives lost.

This value is clamped to the interval [0, 9999].

## 累積ゲームオーバー回数

> Cumulative Number of Game Overs

Represents the number of game overs.

## 累積プレイ時間/lo

> Cumulative Play Time/lo

Represents the least significant bytes for the amount of time spent actively playing the game, in frames.

## 累積プレイ時間/hi

> Cumulative Play Time/hi

Represents the most significant bytes for the amount of time spent actively playing the game, in frames.

## でしゃばりルイージ出現カウンタ

> Intrusive Luigi Appearance Counter

Represents the number of missions cleared and lives lost to allow Luigi to appear on standby.

This value is wrapped around the interval [0, 5].

## 銀行屋キノピオ[利子]

> Banker Kinopio [Interest]

Represents the number of Star Bits earned by the Banktoad to be added to the number of banked Star Bits.

This value is derived from the value of the ones place from the number of Star Bits collected after clearing a mission and will be deposited when the Banktoad is talked to next.

## 顔惑星イベント番号/0

> Face Planet Event Number/0

TBD

## 顔惑星イベント番号/1

> Face Planet Event Number/1

TBD

## 顔惑星イベント番号/2

> Face Planet Event Number/2

TBD

## 顔惑星イベント番号/3

> Face Planet Event Number/3

TBD

## 顔惑星イベントグランドスター番号

> Face Planet Event Grand Star Number

TBD

## 一定数死亡後のステージクリア回数

> Number of Stages Cleared After a Certain Number of Deaths

TBD

TODO: Under certain conditions, if this value exceeds 4, it will be reset to 0, and Lubba will inform the player about Co-Star Mode.
