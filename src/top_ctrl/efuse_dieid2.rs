#[doc = "Register `EFUSE_DIEID2` reader"]
pub type R = crate::R<EfuseDieid2Spec>;
#[doc = "Register `EFUSE_DIEID2` writer"]
pub type W = crate::W<EfuseDieid2Spec>;
#[doc = "Field `val` reader - 31:0\\]
EFUSE DieID\\[95:64\\]"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - 31:0\\]
EFUSE DieID\\[95:64\\]"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
EFUSE DieID\\[95:64\\]"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
EFUSE DieID\\[95:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<EfuseDieid2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "EFUSE_DIEID2\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_dieid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_dieid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseDieid2Spec;
impl crate::RegisterSpec for EfuseDieid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_dieid2::R`](R) reader structure"]
impl crate::Readable for EfuseDieid2Spec {}
#[doc = "`write(|w| ..)` method takes [`efuse_dieid2::W`](W) writer structure"]
impl crate::Writable for EfuseDieid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_DIEID2 to value 0"]
impl crate::Resettable for EfuseDieid2Spec {
    const RESET_VALUE: u32 = 0;
}
