#[doc = "Register `PBIST_CI2` reader"]
pub type R = crate::R<PbistCi2Spec>;
#[doc = "Register `PBIST_CI2` writer"]
pub type W = crate::W<PbistCi2Spec>;
#[doc = "Field `PBIST_CI2` reader - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistCi2R = crate::FieldReader<u16>;
#[doc = "Field `PBIST_CI2` writer - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistCi2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn pbist_ci2(&self) -> PbistCi2R {
        PbistCi2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_ci2(&mut self) -> PbistCi2W<PbistCi2Spec> {
        PbistCi2W::new(self, 0)
    }
}
#[doc = "Constant Increment Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ci2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ci2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistCi2Spec;
impl crate::RegisterSpec for PbistCi2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbist_ci2::R`](R) reader structure"]
impl crate::Readable for PbistCi2Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_ci2::W`](W) writer structure"]
impl crate::Writable for PbistCi2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBIST_CI2 to value 0"]
impl crate::Resettable for PbistCi2Spec {
    const RESET_VALUE: u16 = 0;
}
