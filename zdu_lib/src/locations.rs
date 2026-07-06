#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum Location {
    /// NES: $02 $B88A
    StartingInventoryData = 0xB89A,
    /// NES: $06 $819B
    CaveDataLocationStart = 0x181AB,
    /// NES: $06 $841B
    FirstQuestDungeonDataEarly = 0x1842B,
    /// NES: $06 $871B
    FirstQuestDungeonDataLate = 0x1872B,
    /// NES: $06 $861B
    FirstQuestDungeonItemsEarly = 0x1862B,
    /// NES: $06 $891B
    FirstQuestDungeonItemsLate = 0x1892B,
    /// NES: $06 $8000
    LevelAddresses = 0x18010,
    /// NES: $06 $8334
    OwItemLetterCave = 0x18344,
    /// NES: $06 $8325
    OwItemMagicalSwordGrave = 0x18335,
    /// NES: $06 $831C
    OwItemStartingSwordCave = 0x1832C,
    /// NES: $06 $831E
    OwItemTakeAnyLeft = 0x1832E,
    /// NES: $06 $831F
    OwItemTakeAnyMiddle = 0x1832F,
    /// NES: $06 $8320
    OwItemTakeAnyRight = 0x18330,
    /// NES: $06 $8322
    OwItemWhiteSwordPond = 0x18332,
    /// NES: $06 $837E
    PriceArrowLeft = 0x1838E,
    /// NES: $06 $837F
    PriceArrowMiddle = 0x1838F,
    /// NES: $06 $8380
    PriceArrowRight = 0x18390,
    /// NES: $06 $8381
    PriceCandleLeft = 0x18391,
    /// NES: $06 $8382
    PriceCandleMiddle = 0x18392,
    /// NES: $06 $8383
    PriceCandleRight = 0x18393,
    /// NES: $06 $8375
    PricePotionLeft = 0x18385,
    /// NES: $06 $8376
    PricePotionMiddle = 0x18386,
    /// NES: $06 $8377
    PricePotionRight = 0x18387,
    /// NES: $06 $8387
    PriceRingLeft = 0x18397,
    /// NES: $06 $8388
    PriceRingMiddle = 0x18398,
    /// NES: $06 $8389
    PriceRingRight = 0x18399,
    /// NES: $06 $8384
    PriceShieldLeft = 0x18394,
    /// NES: $06 $8385
    PriceShieldMiddle = 0x18395,
    /// NES: $06 $8386
    PriceShieldRight = 0x18396,
    /// NES: $06 $838B
    SecretMoney1 = 0x1839B,
    /// NES: $06 $838E
    SecretMoney2 = 0x1839E,
    /// NES: $06 $8391
    SecretMoney3 = 0x183A1,
    /// NES: $06 $8342
    ShopArrowLeft = 0x18352,
    /// NES: $06 $8343
    ShopArrowMiddle = 0x18353,
    /// NES: $06 $8344
    ShopArrowRight = 0x18354,
    /// NES: $06 $8345
    ShopCandleLeft = 0x18355,
    /// NES: $06 $8346
    ShopCandleMiddle = 0x18356,
    /// NES: $06 $8347
    ShopCandleRight = 0x18357,
    /// NES: $06 $8339
    ShopPotionLeft = 0x18349,
    /// NES: $06 $833A
    ShopPotionMiddle = 0x1834A,
    /// NES: $06 $833B
    ShopPotionRight = 0x1834B,
    /// NES: $06 $834B
    ShopRingLeft = 0x1835B,
    /// NES: $06 $834C
    ShopRingMiddle = 0x1835C,
    /// NES: $06 $834D
    ShopRingRight = 0x1835D,
    /// NES: $06 $8348
    ShopShieldLeft = 0x18358,
    /// NES: $06 $8349
    ShopShieldMiddle = 0x18359,
    /// NES: $06 $834A
    ShopShieldRight = 0x1835A,
    /// NES: $06 $904F
    WarpCaveOffset = 0x1905F,
    /// NES: $07 $FFAC
    ShowSecretOwTiles = 0x1FFBC,
}
