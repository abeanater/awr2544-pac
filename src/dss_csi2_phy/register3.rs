#[doc = "Register `REGISTER3` reader"]
pub type R = crate::R<Register3Spec>;
#[doc = "Register `REGISTER3` writer"]
pub type W = crate::W<Register3Spec>;
#[doc = "Field `REG_TXTRIGGERESC0` reader - 7:0\\]
Default: 10100000"]
pub type RegTxtriggeresc0R = crate::FieldReader;
#[doc = "Field `REG_TXTRIGGERESC0` writer - 7:0\\]
Default: 10100000"]
pub type RegTxtriggeresc0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_TXTRIGGERESC1` reader - 15:8\\]
Default: 00100001"]
pub type RegTxtriggeresc1R = crate::FieldReader;
#[doc = "Field `REG_TXTRIGGERESC1` writer - 15:8\\]
Default: 00100001"]
pub type RegTxtriggeresc1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_TXTRIGGERESC2` reader - 23:16\\]
Default: 01011101"]
pub type RegTxtriggeresc2R = crate::FieldReader;
#[doc = "Field `REG_TXTRIGGERESC2` writer - 23:16\\]
Default: 01011101"]
pub type RegTxtriggeresc2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_TXTRIGGERESC3` reader - 31:24\\]
Transmitted pattern when REG_TXTRIGGERESC3 is asserted (first bit transmitted to last bit transmitted). Default: 01100010"]
pub type RegTxtriggeresc3R = crate::FieldReader;
#[doc = "Field `REG_TXTRIGGERESC3` writer - 31:24\\]
Transmitted pattern when REG_TXTRIGGERESC3 is asserted (first bit transmitted to last bit transmitted). Default: 01100010"]
pub type RegTxtriggeresc3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Default: 10100000"]
    #[inline(always)]
    pub fn reg_txtriggeresc0(&self) -> RegTxtriggeresc0R {
        RegTxtriggeresc0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Default: 00100001"]
    #[inline(always)]
    pub fn reg_txtriggeresc1(&self) -> RegTxtriggeresc1R {
        RegTxtriggeresc1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Default: 01011101"]
    #[inline(always)]
    pub fn reg_txtriggeresc2(&self) -> RegTxtriggeresc2R {
        RegTxtriggeresc2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Transmitted pattern when REG_TXTRIGGERESC3 is asserted (first bit transmitted to last bit transmitted). Default: 01100010"]
    #[inline(always)]
    pub fn reg_txtriggeresc3(&self) -> RegTxtriggeresc3R {
        RegTxtriggeresc3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Default: 10100000"]
    #[inline(always)]
    #[must_use]
    pub fn reg_txtriggeresc0(&mut self) -> RegTxtriggeresc0W<Register3Spec> {
        RegTxtriggeresc0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Default: 00100001"]
    #[inline(always)]
    #[must_use]
    pub fn reg_txtriggeresc1(&mut self) -> RegTxtriggeresc1W<Register3Spec> {
        RegTxtriggeresc1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Default: 01011101"]
    #[inline(always)]
    #[must_use]
    pub fn reg_txtriggeresc2(&mut self) -> RegTxtriggeresc2W<Register3Spec> {
        RegTxtriggeresc2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Transmitted pattern when REG_TXTRIGGERESC3 is asserted (first bit transmitted to last bit transmitted). Default: 01100010"]
    #[inline(always)]
    #[must_use]
    pub fn reg_txtriggeresc3(&mut self) -> RegTxtriggeresc3W<Register3Spec> {
        RegTxtriggeresc3W::new(self, 24)
    }
}
#[doc = "REGISTER3\n\nYou can [`read`](crate::Reg::read) this register and get [`register3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register3Spec;
impl crate::RegisterSpec for Register3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register3::R`](R) reader structure"]
impl crate::Readable for Register3Spec {}
#[doc = "`write(|w| ..)` method takes [`register3::W`](W) writer structure"]
impl crate::Writable for Register3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER3 to value 0"]
impl crate::Resettable for Register3Spec {
    const RESET_VALUE: u32 = 0;
}
