#[doc = "Register `SPILVL` reader"]
pub type R = crate::R<SpilvlSpec>;
#[doc = "Register `SPILVL` writer"]
pub type W = crate::W<SpilvlSpec>;
#[doc = "Field `DLENERRLVL` reader - 0:0\\]
Data Length Error interrupt Enable Level. 1 = An interrupt on Data Length Error is mapped to interrupt line INT1. 0 = An interrupt on Data Length Error is mapped to interrupt line INT0."]
pub type DlenerrlvlR = crate::BitReader;
#[doc = "Field `DLENERRLVL` writer - 0:0\\]
Data Length Error interrupt Enable Level. 1 = An interrupt on Data Length Error is mapped to interrupt line INT1. 0 = An interrupt on Data Length Error is mapped to interrupt line INT0."]
pub type DlenerrlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTLVL` reader - 1:1\\]
SPIENA pin Time-out interrupt level. 1 =An interrupt on a time-out of the ENA signal (TIMEOUT = 1) is mapped to interrupt line INT1. 0 =An interrupt on a time-out of the ENA signal (TIMEOUT = 1) is mapped to interrupt line INT0."]
pub type TimeoutlvlR = crate::BitReader;
#[doc = "Field `TIMEOUTLVL` writer - 1:1\\]
SPIENA pin Time-out interrupt level. 1 =An interrupt on a time-out of the ENA signal (TIMEOUT = 1) is mapped to interrupt line INT1. 0 =An interrupt on a time-out of the ENA signal (TIMEOUT = 1) is mapped to interrupt line INT0."]
pub type TimeoutlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARERRLVL` reader - 2:2\\]
Parity error interrupt level. 1 =A parity error interrupt (PARITYERR = 1) is mapped to interrupt line INT1. 0 =A parity error interrupt (PARITYERR = 1) is mapped to interrupt line INT0."]
pub type ParerrlvlR = crate::BitReader;
#[doc = "Field `PARERRLVL` writer - 2:2\\]
Parity error interrupt level. 1 =A parity error interrupt (PARITYERR = 1) is mapped to interrupt line INT1. 0 =A parity error interrupt (PARITYERR = 1) is mapped to interrupt line INT0."]
pub type ParerrlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESYNCLVL` reader - 3:3\\]
De-synchronized slave interrupt level. DESYNCLVL is used in master mode only. 1 =An interrupt due to de-synchronization of the slave (DESYNC = 1) is mapped to interrupt line INT1. 0 =An interrupt due to de-synchronization of the slave (DESYNC = 1) is mapped to interrupt line INT0."]
pub type DesynclvlR = crate::BitReader;
#[doc = "Field `DESYNCLVL` writer - 3:3\\]
De-synchronized slave interrupt level. DESYNCLVL is used in master mode only. 1 =An interrupt due to de-synchronization of the slave (DESYNC = 1) is mapped to interrupt line INT1. 0 =An interrupt due to de-synchronization of the slave (DESYNC = 1) is mapped to interrupt line INT0."]
pub type DesynclvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITERRLVL` reader - 4:4\\]
Bit error interrupt level. 1 =bit error interrupt is mapped to interrupt line INT1. 0 =bit error interrupt is mapped to interrupt line INT0."]
pub type BiterrlvlR = crate::BitReader;
#[doc = "Field `BITERRLVL` writer - 4:4\\]
Bit error interrupt level. 1 =bit error interrupt is mapped to interrupt line INT1. 0 =bit error interrupt is mapped to interrupt line INT0."]
pub type BiterrlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRNINTLVL` reader - 6:6\\]
Receive Overrun interrupt level. 1 =Receive Overrun interrupt is mapped to interrupt line INT1. 0 =Receive Overrun interrupt is mapped to interrupt line INT0."]
pub type OvrnintlvlR = crate::BitReader;
#[doc = "Field `OVRNINTLVL` writer - 6:6\\]
Receive Overrun interrupt level. 1 =Receive Overrun interrupt is mapped to interrupt line INT1. 0 =Receive Overrun interrupt is mapped to interrupt line INT0."]
pub type OvrnintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::BitReader;
#[doc = "Field `NU2` writer - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINTLVL` reader - 8:8\\]
Receive interrupt level. 1 =Receive interrupt is mapped to interrupt line INT1. 0 =Receive interrupt is mapped to interrupt line INT0."]
pub type RxintlvlR = crate::BitReader;
#[doc = "Field `RXINTLVL` writer - 8:8\\]
Receive interrupt level. 1 =Receive interrupt is mapped to interrupt line INT1. 0 =Receive interrupt is mapped to interrupt line INT0."]
pub type RxintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINTLVL` reader - 9:9\\]
Transmit Interrupt Level. 1 =Transmit interrupt is mapped to interrupt line INT1. 0 =Transmit interrupt is mapped to interrupt line INT0."]
pub type TxintlvlR = crate::BitReader;
#[doc = "Field `TXINTLVL` writer - 9:9\\]
Transmit Interrupt Level. 1 =Transmit interrupt is mapped to interrupt line INT1. 0 =Transmit interrupt is mapped to interrupt line INT0."]
pub type TxintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 31:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::FieldReader<u32>;
#[doc = "Field `NU3` writer - 31:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data Length Error interrupt Enable Level. 1 = An interrupt on Data Length Error is mapped to interrupt line INT1. 0 = An interrupt on Data Length Error is mapped to interrupt line INT0."]
    #[inline(always)]
    pub fn dlenerrlvl(&self) -> DlenerrlvlR {
        DlenerrlvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SPIENA pin Time-out interrupt level. 1 =An interrupt on a time-out of the ENA signal (TIMEOUT = 1) is mapped to interrupt line INT1. 0 =An interrupt on a time-out of the ENA signal (TIMEOUT = 1) is mapped to interrupt line INT0."]
    #[inline(always)]
    pub fn timeoutlvl(&self) -> TimeoutlvlR {
        TimeoutlvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Parity error interrupt level. 1 =A parity error interrupt (PARITYERR = 1) is mapped to interrupt line INT1. 0 =A parity error interrupt (PARITYERR = 1) is mapped to interrupt line INT0."]
    #[inline(always)]
    pub fn parerrlvl(&self) -> ParerrlvlR {
        ParerrlvlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
De-synchronized slave interrupt level. DESYNCLVL is used in master mode only. 1 =An interrupt due to de-synchronization of the slave (DESYNC = 1) is mapped to interrupt line INT1. 0 =An interrupt due to de-synchronization of the slave (DESYNC = 1) is mapped to interrupt line INT0."]
    #[inline(always)]
    pub fn desynclvl(&self) -> DesynclvlR {
        DesynclvlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Bit error interrupt level. 1 =bit error interrupt is mapped to interrupt line INT1. 0 =bit error interrupt is mapped to interrupt line INT0."]
    #[inline(always)]
    pub fn biterrlvl(&self) -> BiterrlvlR {
        BiterrlvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive Overrun interrupt level. 1 =Receive Overrun interrupt is mapped to interrupt line INT1. 0 =Receive Overrun interrupt is mapped to interrupt line INT0."]
    #[inline(always)]
    pub fn ovrnintlvl(&self) -> OvrnintlvlR {
        OvrnintlvlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Receive interrupt level. 1 =Receive interrupt is mapped to interrupt line INT1. 0 =Receive interrupt is mapped to interrupt line INT0."]
    #[inline(always)]
    pub fn rxintlvl(&self) -> RxintlvlR {
        RxintlvlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmit Interrupt Level. 1 =Transmit interrupt is mapped to interrupt line INT1. 0 =Transmit interrupt is mapped to interrupt line INT0."]
    #[inline(always)]
    pub fn txintlvl(&self) -> TxintlvlR {
        TxintlvlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data Length Error interrupt Enable Level. 1 = An interrupt on Data Length Error is mapped to interrupt line INT1. 0 = An interrupt on Data Length Error is mapped to interrupt line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn dlenerrlvl(&mut self) -> DlenerrlvlW<SpilvlSpec> {
        DlenerrlvlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SPIENA pin Time-out interrupt level. 1 =An interrupt on a time-out of the ENA signal (TIMEOUT = 1) is mapped to interrupt line INT1. 0 =An interrupt on a time-out of the ENA signal (TIMEOUT = 1) is mapped to interrupt line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn timeoutlvl(&mut self) -> TimeoutlvlW<SpilvlSpec> {
        TimeoutlvlW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Parity error interrupt level. 1 =A parity error interrupt (PARITYERR = 1) is mapped to interrupt line INT1. 0 =A parity error interrupt (PARITYERR = 1) is mapped to interrupt line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn parerrlvl(&mut self) -> ParerrlvlW<SpilvlSpec> {
        ParerrlvlW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
De-synchronized slave interrupt level. DESYNCLVL is used in master mode only. 1 =An interrupt due to de-synchronization of the slave (DESYNC = 1) is mapped to interrupt line INT1. 0 =An interrupt due to de-synchronization of the slave (DESYNC = 1) is mapped to interrupt line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn desynclvl(&mut self) -> DesynclvlW<SpilvlSpec> {
        DesynclvlW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Bit error interrupt level. 1 =bit error interrupt is mapped to interrupt line INT1. 0 =bit error interrupt is mapped to interrupt line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn biterrlvl(&mut self) -> BiterrlvlW<SpilvlSpec> {
        BiterrlvlW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<SpilvlSpec> {
        Nu1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive Overrun interrupt level. 1 =Receive Overrun interrupt is mapped to interrupt line INT1. 0 =Receive Overrun interrupt is mapped to interrupt line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn ovrnintlvl(&mut self) -> OvrnintlvlW<SpilvlSpec> {
        OvrnintlvlW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<SpilvlSpec> {
        Nu2W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Receive interrupt level. 1 =Receive interrupt is mapped to interrupt line INT1. 0 =Receive interrupt is mapped to interrupt line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn rxintlvl(&mut self) -> RxintlvlW<SpilvlSpec> {
        RxintlvlW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmit Interrupt Level. 1 =Transmit interrupt is mapped to interrupt line INT1. 0 =Transmit interrupt is mapped to interrupt line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn txintlvl(&mut self) -> TxintlvlW<SpilvlSpec> {
        TxintlvlW::new(self, 9)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<SpilvlSpec> {
        Nu3W::new(self, 10)
    }
}
#[doc = "SPI / MibSPI Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spilvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spilvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpilvlSpec;
impl crate::RegisterSpec for SpilvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spilvl::R`](R) reader structure"]
impl crate::Readable for SpilvlSpec {}
#[doc = "`write(|w| ..)` method takes [`spilvl::W`](W) writer structure"]
impl crate::Writable for SpilvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPILVL to value 0"]
impl crate::Resettable for SpilvlSpec {
    const RESET_VALUE: u32 = 0;
}
