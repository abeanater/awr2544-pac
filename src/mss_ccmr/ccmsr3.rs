#[doc = "Register `CCMSR3` reader"]
pub type R = crate::R<Ccmsr3Spec>;
#[doc = "Register `CCMSR3` writer"]
pub type W = crate::W<Ccmsr3Spec>;
#[doc = "Field `STE3` reader - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
pub type Ste3R = crate::BitReader;
#[doc = "Field `STE3` writer - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
pub type Ste3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STET3` reader - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
pub type Stet3R = crate::BitReader;
#[doc = "Field `STET3` writer - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
pub type Stet3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU8` reader - 7:2\\]
Reserved"]
pub type Nu8R = crate::FieldReader;
#[doc = "Field `NU8` writer - 7:2\\]
Reserved"]
pub type Nu8W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `STC3` reader - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
pub type Stc3R = crate::BitReader;
#[doc = "Field `STC3` writer - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
pub type Stc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU9` reader - 15:9\\]
Reserved"]
pub type Nu9R = crate::FieldReader;
#[doc = "Field `NU9` writer - 15:9\\]
Reserved"]
pub type Nu9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CMPE3` reader - 16:16\\]
Compare Error 0 = Inactivity monitor signals are identical 1= Inactivity monitor signal compare mismatch Writes '1' to clear this bit"]
pub type Cmpe3R = crate::BitReader;
#[doc = "Field `CMPE3` writer - 16:16\\]
Compare Error 0 = Inactivity monitor signals are identical 1= Inactivity monitor signal compare mismatch Writes '1' to clear this bit"]
pub type Cmpe3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU10` reader - 31:17\\]
Reserved"]
pub type Nu10R = crate::FieldReader<u16>;
#[doc = "Field `NU10` writer - 31:17\\]
Reserved"]
pub type Nu10W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
    #[inline(always)]
    pub fn ste3(&self) -> Ste3R {
        Ste3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
    #[inline(always)]
    pub fn stet3(&self) -> Stet3R {
        Stet3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu8(&self) -> Nu8R {
        Nu8R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
    #[inline(always)]
    pub fn stc3(&self) -> Stc3R {
        Stc3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    pub fn nu9(&self) -> Nu9R {
        Nu9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Compare Error 0 = Inactivity monitor signals are identical 1= Inactivity monitor signal compare mismatch Writes '1' to clear this bit"]
    #[inline(always)]
    pub fn cmpe3(&self) -> Cmpe3R {
        Cmpe3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved"]
    #[inline(always)]
    pub fn nu10(&self) -> Nu10R {
        Nu10R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ste3(&mut self) -> Ste3W<Ccmsr3Spec> {
        Ste3W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn stet3(&mut self) -> Stet3W<Ccmsr3Spec> {
        Stet3W::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu8(&mut self) -> Nu8W<Ccmsr3Spec> {
        Nu8W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn stc3(&mut self) -> Stc3W<Ccmsr3Spec> {
        Stc3W::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu9(&mut self) -> Nu9W<Ccmsr3Spec> {
        Nu9W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Compare Error 0 = Inactivity monitor signals are identical 1= Inactivity monitor signal compare mismatch Writes '1' to clear this bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpe3(&mut self) -> Cmpe3W<Ccmsr3Spec> {
        Cmpe3W::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu10(&mut self) -> Nu10W<Ccmsr3Spec> {
        Nu10W::new(self, 17)
    }
}
#[doc = "Inactivity Monitor Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmsr3Spec;
impl crate::RegisterSpec for Ccmsr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmsr3::R`](R) reader structure"]
impl crate::Readable for Ccmsr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmsr3::W`](W) writer structure"]
impl crate::Writable for Ccmsr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMSR3 to value 0"]
impl crate::Resettable for Ccmsr3Spec {
    const RESET_VALUE: u32 = 0;
}
