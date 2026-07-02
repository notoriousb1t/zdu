#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum Location {
    /// NES: $02 $B88F
    StartingInventoryData = 0xB89F,
    /// NES: $06 $81FE
    CaveDataLocationStart = 0x1820E,
    /// NES: $06 $847E
    FirstQuestDungeonDataEarly = 0x1848E,
    /// NES: $06 $877E
    FirstQuestDungeonDataLate = 0x1878E,
    /// NES: $06 $867E
    FirstQuestDungeonItemsEarly = 0x1868E,
    /// NES: $06 $897E
    FirstQuestDungeonItemsLate = 0x1898E,
    /// NES: $06 $8000
    LevelAddresses = 0x18010,
    /// NES: $06 $8397
    OwItemLetterCave = 0x183A7,
    /// NES: $06 $8388
    OwItemMagicalSwordGrave = 0x18398,
    /// NES: $06 $837F
    OwItemStartingSwordCave = 0x1838F,
    /// NES: $06 $8381
    OwItemTakeAnyLeft = 0x18391,
    /// NES: $06 $8382
    OwItemTakeAnyMiddle = 0x18392,
    /// NES: $06 $8383
    OwItemTakeAnyRight = 0x18393,
    /// NES: $06 $8385
    OwItemWhiteSwordPond = 0x18395,
    /// NES: $06 $83E1
    PriceArrowLeft = 0x183F1,
    /// NES: $06 $83E2
    PriceArrowMiddle = 0x183F2,
    /// NES: $06 $83E3
    PriceArrowRight = 0x183F3,
    /// NES: $06 $83E4
    PriceCandleLeft = 0x183F4,
    /// NES: $06 $83E5
    PriceCandleMiddle = 0x183F5,
    /// NES: $06 $83E6
    PriceCandleRight = 0x183F6,
    /// NES: $06 $83D8
    PricePotionLeft = 0x183E8,
    /// NES: $06 $83D9
    PricePotionMiddle = 0x183E9,
    /// NES: $06 $83DA
    PricePotionRight = 0x183EA,
    /// NES: $06 $83EA
    PriceRingLeft = 0x183FA,
    /// NES: $06 $83EB
    PriceRingMiddle = 0x183FB,
    /// NES: $06 $83EC
    PriceRingRight = 0x183FC,
    /// NES: $06 $83E7
    PriceShieldLeft = 0x183F7,
    /// NES: $06 $83E8
    PriceShieldMiddle = 0x183F8,
    /// NES: $06 $83E9
    PriceShieldRight = 0x183F9,
    /// NES: $06 $8A7E
    SecondQuestDungeonDataEarly = 0x18A8E,
    /// NES: $06 $8D7E
    SecondQuestDungeonDataLate = 0x18D8E,
    /// NES: $06 $8C7E
    SecondQuestDungeonItemsEarly = 0x18C8E,
    /// NES: $06 $8F7E
    SecondQuestDungeonItemsLate = 0x18F8E,
    /// NES: $06 $83EE
    SecretMoney1 = 0x183FE,
    /// NES: $06 $83F1
    SecretMoney2 = 0x18401,
    /// NES: $06 $83F4
    SecretMoney3 = 0x18404,
    /// NES: $06 $83A5
    ShopArrowLeft = 0x183B5,
    /// NES: $06 $83A6
    ShopArrowMiddle = 0x183B6,
    /// NES: $06 $83A7
    ShopArrowRight = 0x183B7,
    /// NES: $06 $83A8
    ShopCandleLeft = 0x183B8,
    /// NES: $06 $83A9
    ShopCandleMiddle = 0x183B9,
    /// NES: $06 $83AA
    ShopCandleRight = 0x183BA,
    /// NES: $06 $839C
    ShopPotionLeft = 0x183AC,
    /// NES: $06 $839D
    ShopPotionMiddle = 0x183AD,
    /// NES: $06 $839E
    ShopPotionRight = 0x183AE,
    /// NES: $06 $83AE
    ShopRingLeft = 0x183BE,
    /// NES: $06 $83AF
    ShopRingMiddle = 0x183BF,
    /// NES: $06 $83B0
    ShopRingRight = 0x183C0,
    /// NES: $06 $83AB
    ShopShieldLeft = 0x183BB,
    /// NES: $06 $83AC
    ShopShieldMiddle = 0x183BC,
    /// NES: $06 $83AD
    ShopShieldRight = 0x183BD,
    /// NES: $06 $90B2
    WarpCaveOffset = 0x190C2,
    /// NES: $07 $FFAC
    ShowSecretOwTiles = 0x1FFBC,
}
