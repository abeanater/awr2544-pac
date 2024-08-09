#[doc = "Register `EFUSE_UID2` reader"]
pub type R = crate::R<EfuseUid2Spec>;
#[doc = "Register `EFUSE_UID2` writer"]
pub type W = crate::W<EfuseUid2Spec>;
#[doc = "Field `val` reader - 31:0\\]
EFUSE UID\\[95:64\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - 31:0\\]
EFUSE UID\\[95:64\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
EFUSE UID\\[95:64\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
EFUSE UID\\[95:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<EfuseUid2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "EFUSE_UID2\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_uid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_uid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseUid2Spec;
impl crate::RegisterSpec for EfuseUid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_uid2::R`](R) reader structure"]
impl crate::Readable for EfuseUid2Spec {}
#[doc = "`write(|w| ..)` method takes [`efuse_uid2::W`](W) writer structure"]
impl crate::Writable for EfuseUid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_UID2 to value 0"]
impl crate::Resettable for EfuseUid2Spec {
    const RESET_VALUE: u32 = 0;
}
