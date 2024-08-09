#[doc = "Register `CCMSR1` reader"]
pub type R = crate::R<Ccmsr1Spec>;
#[doc = "Register `CCMSR1` writer"]
pub type W = crate::W<Ccmsr1Spec>;
#[doc = "Field `STE1` reader - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
pub type Ste1R = crate::BitReader;
#[doc = "Field `STE1` writer - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
pub type Ste1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STET1` reader - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
pub type Stet1R = crate::BitReader;
#[doc = "Field `STET1` writer - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
pub type Stet1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU0` reader - 7:2\\]
Reserved"]
pub type Nu0R = crate::FieldReader;
#[doc = "Field `NU0` writer - 7:2\\]
Reserved"]
pub type Nu0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `STC1` reader - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
pub type Stc1R = crate::BitReader;
#[doc = "Field `STC1` writer - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
pub type Stc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 15:9\\]
Reserved"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 15:9\\]
Reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CMPE1` reader - 16:16\\]
Compare Error 0 = CPU signals are identical 1= CPU signal compare mismatch Writes '1' to clear this bit"]
pub type Cmpe1R = crate::BitReader;
#[doc = "Field `CMPE1` writer - 16:16\\]
Compare Error 0 = CPU signals are identical 1= CPU signal compare mismatch Writes '1' to clear this bit"]
pub type Cmpe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 31:17\\]
Reserved"]
pub type Nu2R = crate::FieldReader<u16>;
#[doc = "Field `NU2` writer - 31:17\\]
Reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
    #[inline(always)]
    pub fn ste1(&self) -> Ste1R {
        Ste1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
    #[inline(always)]
    pub fn stet1(&self) -> Stet1R {
        Stet1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu0(&self) -> Nu0R {
        Nu0R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
    #[inline(always)]
    pub fn stc1(&self) -> Stc1R {
        Stc1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Compare Error 0 = CPU signals are identical 1= CPU signal compare mismatch Writes '1' to clear this bit"]
    #[inline(always)]
    pub fn cmpe1(&self) -> Cmpe1R {
        Cmpe1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ste1(&mut self) -> Ste1W<Ccmsr1Spec> {
        Ste1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn stet1(&mut self) -> Stet1W<Ccmsr1Spec> {
        Stet1W::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu0(&mut self) -> Nu0W<Ccmsr1Spec> {
        Nu0W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn stc1(&mut self) -> Stc1W<Ccmsr1Spec> {
        Stc1W::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Ccmsr1Spec> {
        Nu1W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Compare Error 0 = CPU signals are identical 1= CPU signal compare mismatch Writes '1' to clear this bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpe1(&mut self) -> Cmpe1W<Ccmsr1Spec> {
        Cmpe1W::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Ccmsr1Spec> {
        Nu2W::new(self, 17)
    }
}
#[doc = "CPU Compare Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmsr1Spec;
impl crate::RegisterSpec for Ccmsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmsr1::R`](R) reader structure"]
impl crate::Readable for Ccmsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmsr1::W`](W) writer structure"]
impl crate::Writable for Ccmsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMSR1 to value 0"]
impl crate::Resettable for Ccmsr1Spec {
    const RESET_VALUE: u32 = 0;
}
