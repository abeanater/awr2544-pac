#[doc = "Register `EFUSE_UID3` reader"]
pub type R = crate::R<EfuseUid3Spec>;
#[doc = "Register `EFUSE_UID3` writer"]
pub type W = crate::W<EfuseUid3Spec>;
#[doc = "Field `val` reader - 23:0\\]
EFUSE UID\\[120:96\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - 23:0\\]
EFUSE UID\\[120:96\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
EFUSE UID\\[120:96\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
EFUSE UID\\[120:96\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<EfuseUid3Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "EFUSE_UID3\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_uid3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_uid3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseUid3Spec;
impl crate::RegisterSpec for EfuseUid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_uid3::R`](R) reader structure"]
impl crate::Readable for EfuseUid3Spec {}
#[doc = "`write(|w| ..)` method takes [`efuse_uid3::W`](W) writer structure"]
impl crate::Writable for EfuseUid3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_UID3 to value 0"]
impl crate::Resettable for EfuseUid3Spec {
    const RESET_VALUE: u32 = 0;
}
