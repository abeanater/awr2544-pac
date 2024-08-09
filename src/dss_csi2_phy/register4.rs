#[doc = "Register `REGISTER4` reader"]
pub type R = crate::R<Register4Spec>;
#[doc = "Register `REGISTER4` writer"]
pub type W = crate::W<Register4Spec>;
#[doc = "Field `REG_RXTRIGGERESC0` reader - 7:0\\]
Default: 10100000"]
pub type RegRxtriggeresc0R = crate::FieldReader;
#[doc = "Field `REG_RXTRIGGERESC0` writer - 7:0\\]
Default: 10100000"]
pub type RegRxtriggeresc0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_RXTRIGGERESC1` reader - 15:8\\]
Default: 00100001"]
pub type RegRxtriggeresc1R = crate::FieldReader;
#[doc = "Field `REG_RXTRIGGERESC1` writer - 15:8\\]
Default: 00100001"]
pub type RegRxtriggeresc1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_RXTRIGGERESC2` reader - 23:16\\]
Default: 01011101"]
pub type RegRxtriggeresc2R = crate::FieldReader;
#[doc = "Field `REG_RXTRIGGERESC2` writer - 23:16\\]
Default: 01011101"]
pub type RegRxtriggeresc2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_RXTRIGGERESC3` reader - 31:24\\]
Received pattern for which REG_RXTRIGGERESC3 is asserted (first bit transmitted to last bit transmitted). Default: 01100010"]
pub type RegRxtriggeresc3R = crate::FieldReader;
#[doc = "Field `REG_RXTRIGGERESC3` writer - 31:24\\]
Received pattern for which REG_RXTRIGGERESC3 is asserted (first bit transmitted to last bit transmitted). Default: 01100010"]
pub type RegRxtriggeresc3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Default: 10100000"]
    #[inline(always)]
    pub fn reg_rxtriggeresc0(&self) -> RegRxtriggeresc0R {
        RegRxtriggeresc0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Default: 00100001"]
    #[inline(always)]
    pub fn reg_rxtriggeresc1(&self) -> RegRxtriggeresc1R {
        RegRxtriggeresc1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Default: 01011101"]
    #[inline(always)]
    pub fn reg_rxtriggeresc2(&self) -> RegRxtriggeresc2R {
        RegRxtriggeresc2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Received pattern for which REG_RXTRIGGERESC3 is asserted (first bit transmitted to last bit transmitted). Default: 01100010"]
    #[inline(always)]
    pub fn reg_rxtriggeresc3(&self) -> RegRxtriggeresc3R {
        RegRxtriggeresc3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Default: 10100000"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rxtriggeresc0(&mut self) -> RegRxtriggeresc0W<Register4Spec> {
        RegRxtriggeresc0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Default: 00100001"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rxtriggeresc1(&mut self) -> RegRxtriggeresc1W<Register4Spec> {
        RegRxtriggeresc1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Default: 01011101"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rxtriggeresc2(&mut self) -> RegRxtriggeresc2W<Register4Spec> {
        RegRxtriggeresc2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Received pattern for which REG_RXTRIGGERESC3 is asserted (first bit transmitted to last bit transmitted). Default: 01100010"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rxtriggeresc3(&mut self) -> RegRxtriggeresc3W<Register4Spec> {
        RegRxtriggeresc3W::new(self, 24)
    }
}
#[doc = "REGISTER4\n\nYou can [`read`](crate::Reg::read) this register and get [`register4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register4Spec;
impl crate::RegisterSpec for Register4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register4::R`](R) reader structure"]
impl crate::Readable for Register4Spec {}
#[doc = "`write(|w| ..)` method takes [`register4::W`](W) writer structure"]
impl crate::Writable for Register4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER4 to value 0"]
impl crate::Resettable for Register4Spec {
    const RESET_VALUE: u32 = 0;
}
