#[doc = "Register `CCMSR2` reader"]
pub type R = crate::R<Ccmsr2Spec>;
#[doc = "Register `CCMSR2` writer"]
pub type W = crate::W<Ccmsr2Spec>;
#[doc = "Field `STE2` reader - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
pub type Ste2R = crate::BitReader;
#[doc = "Field `STE2` writer - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
pub type Ste2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STET2` reader - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
pub type Stet2R = crate::BitReader;
#[doc = "Field `STET2` writer - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
pub type Stet2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU4` reader - 7:2\\]
Reserved"]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 7:2\\]
Reserved"]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `STC2` reader - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
pub type Stc2R = crate::BitReader;
#[doc = "Field `STC2` writer - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
pub type Stc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU5` reader - 15:9\\]
Reserved"]
pub type Nu5R = crate::FieldReader;
#[doc = "Field `NU5` writer - 15:9\\]
Reserved"]
pub type Nu5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CMPE2` reader - 16:16\\]
Compare Error 0 = VIM signals are identical 1= VIM signal compare mismatch Writes '1' to clear this bit"]
pub type Cmpe2R = crate::BitReader;
#[doc = "Field `CMPE2` writer - 16:16\\]
Compare Error 0 = VIM signals are identical 1= VIM signal compare mismatch Writes '1' to clear this bit"]
pub type Cmpe2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU6` reader - 31:17\\]
Reserved"]
pub type Nu6R = crate::FieldReader<u16>;
#[doc = "Field `NU6` writer - 31:17\\]
Reserved"]
pub type Nu6W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
    #[inline(always)]
    pub fn ste2(&self) -> Ste2R {
        Ste2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
    #[inline(always)]
    pub fn stet2(&self) -> Stet2R {
        Stet2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
    #[inline(always)]
    pub fn stc2(&self) -> Stc2R {
        Stc2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    pub fn nu5(&self) -> Nu5R {
        Nu5R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Compare Error 0 = VIM signals are identical 1= VIM signal compare mismatch Writes '1' to clear this bit"]
    #[inline(always)]
    pub fn cmpe2(&self) -> Cmpe2R {
        Cmpe2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved"]
    #[inline(always)]
    pub fn nu6(&self) -> Nu6R {
        Nu6R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Self Test Error 0 = self test passed 1 = self test failed Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ste2(&mut self) -> Ste2W<Ccmsr2Spec> {
        Ste2W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Self Test Error Type 0 = self test failed during Compare Match test 1 = self test failed during Compare mismatch test Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn stet2(&mut self) -> Stet2W<Ccmsr2Spec> {
        Stet2W::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<Ccmsr2Spec> {
        Nu4W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Self Test Complete 0 = self test on-going if self test mode asserted 1 = self test is complete Writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn stc2(&mut self) -> Stc2W<Ccmsr2Spec> {
        Stc2W::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu5(&mut self) -> Nu5W<Ccmsr2Spec> {
        Nu5W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Compare Error 0 = VIM signals are identical 1= VIM signal compare mismatch Writes '1' to clear this bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpe2(&mut self) -> Cmpe2W<Ccmsr2Spec> {
        Cmpe2W::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu6(&mut self) -> Nu6W<Ccmsr2Spec> {
        Nu6W::new(self, 17)
    }
}
#[doc = "VIM Compare Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmsr2Spec;
impl crate::RegisterSpec for Ccmsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmsr2::R`](R) reader structure"]
impl crate::Readable for Ccmsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmsr2::W`](W) writer structure"]
impl crate::Writable for Ccmsr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMSR2 to value 0"]
impl crate::Resettable for Ccmsr2Spec {
    const RESET_VALUE: u32 = 0;
}
