#[doc = "Register `ECCDIAG_STAT` reader"]
pub type R = crate::R<EccdiagStatSpec>;
#[doc = "Register `ECCDIAG_STAT` writer"]
pub type W = crate::W<EccdiagStatSpec>;
#[doc = "Field `SEFLG0` reader - 0:0\\]
Single bit error flag for TXRAM 1 - A Single bit Error is detected for TXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
pub type Seflg0R = crate::BitReader;
#[doc = "Field `SEFLG0` writer - 0:0\\]
Single bit error flag for TXRAM 1 - A Single bit Error is detected for TXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
pub type Seflg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEFLG1` reader - 1:1\\]
Single bit error flag for RXRAM 1 - A Single bit Error is detected for RXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
pub type Seflg1R = crate::BitReader;
#[doc = "Field `SEFLG1` writer - 1:1\\]
Single bit error flag for RXRAM 1 - A Single bit Error is detected for RXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
pub type Seflg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 15:2\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::FieldReader<u16>;
#[doc = "Field `NU1` writer - 15:2\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DEFLG0` reader - 16:16\\]
Double bit error flag for TXRAM 1 - A double bit Error is detected for TXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
pub type Deflg0R = crate::BitReader;
#[doc = "Field `DEFLG0` writer - 16:16\\]
Double bit error flag for TXRAM 1 - A double bit Error is detected for TXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
pub type Deflg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFLG1` reader - 17:17\\]
Double bit error flag for RXRAM 1 - A double bit Error is detected for RXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
pub type Deflg1R = crate::BitReader;
#[doc = "Field `DEFLG1` writer - 17:17\\]
Double bit error flag for RXRAM 1 - A double bit Error is detected for RXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
pub type Deflg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 31:18\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::FieldReader<u16>;
#[doc = "Field `NU2` writer - 31:18\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Single bit error flag for TXRAM 1 - A Single bit Error is detected for TXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
    #[inline(always)]
    pub fn seflg0(&self) -> Seflg0R {
        Seflg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Single bit error flag for RXRAM 1 - A Single bit Error is detected for RXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
    #[inline(always)]
    pub fn seflg1(&self) -> Seflg1R {
        Seflg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Double bit error flag for TXRAM 1 - A double bit Error is detected for TXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
    #[inline(always)]
    pub fn deflg0(&self) -> Deflg0R {
        Deflg0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Double bit error flag for RXRAM 1 - A double bit Error is detected for RXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
    #[inline(always)]
    pub fn deflg1(&self) -> Deflg1R {
        Deflg1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Single bit error flag for TXRAM 1 - A Single bit Error is detected for TXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
    #[inline(always)]
    #[must_use]
    pub fn seflg0(&mut self) -> Seflg0W<EccdiagStatSpec> {
        Seflg0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Single bit error flag for RXRAM 1 - A Single bit Error is detected for RXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
    #[inline(always)]
    #[must_use]
    pub fn seflg1(&mut self) -> Seflg1W<EccdiagStatSpec> {
        Seflg1W::new(self, 1)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<EccdiagStatSpec> {
        Nu1W::new(self, 2)
    }
    #[doc = "Bit 16 - 16:16\\]
Double bit error flag for TXRAM 1 - A double bit Error is detected for TXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
    #[inline(always)]
    #[must_use]
    pub fn deflg0(&mut self) -> Deflg0W<EccdiagStatSpec> {
        Deflg0W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Double bit error flag for RXRAM 1 - A double bit Error is detected for RXRAM bank during diagnostic mode tests. 0 - No error. A write ΓÇÿ1ΓÇÖ to this bit will clear the bit."]
    #[inline(always)]
    #[must_use]
    pub fn deflg1(&mut self) -> Deflg1W<EccdiagStatSpec> {
        Deflg1W::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<EccdiagStatSpec> {
        Nu2W::new(self, 18)
    }
}
#[doc = "ECC Diagnostic Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccdiag_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdiag_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccdiagStatSpec;
impl crate::RegisterSpec for EccdiagStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccdiag_stat::R`](R) reader structure"]
impl crate::Readable for EccdiagStatSpec {}
#[doc = "`write(|w| ..)` method takes [`eccdiag_stat::W`](W) writer structure"]
impl crate::Writable for EccdiagStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCDIAG_STAT to value 0"]
impl crate::Resettable for EccdiagStatSpec {
    const RESET_VALUE: u32 = 0;
}
