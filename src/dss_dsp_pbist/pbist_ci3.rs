#[doc = "Register `PBIST_CI3` reader"]
pub type R = crate::R<PbistCi3Spec>;
#[doc = "Register `PBIST_CI3` writer"]
pub type W = crate::W<PbistCi3Spec>;
#[doc = "Field `PBIST_CI3` reader - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistCi3R = crate::FieldReader<u16>;
#[doc = "Field `PBIST_CI3` writer - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistCi3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn pbist_ci3(&self) -> PbistCi3R {
        PbistCi3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_ci3(&mut self) -> PbistCi3W<PbistCi3Spec> {
        PbistCi3W::new(self, 0)
    }
}
#[doc = "Constant Increment Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ci3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ci3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistCi3Spec;
impl crate::RegisterSpec for PbistCi3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbist_ci3::R`](R) reader structure"]
impl crate::Readable for PbistCi3Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_ci3::W`](W) writer structure"]
impl crate::Writable for PbistCi3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBIST_CI3 to value 0"]
impl crate::Resettable for PbistCi3Spec {
    const RESET_VALUE: u16 = 0;
}
