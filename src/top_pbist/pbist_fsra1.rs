#[doc = "Register `PBIST_FSRA1` reader"]
pub type R = crate::R<PbistFsra1Spec>;
#[doc = "Register `PBIST_FSRA1` writer"]
pub type W = crate::W<PbistFsra1Spec>;
#[doc = "Field `PBIST_FSRA1` reader - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistFsra1R = crate::FieldReader<u16>;
#[doc = "Field `PBIST_FSRA1` writer - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistFsra1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn pbist_fsra1(&self) -> PbistFsra1R {
        PbistFsra1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_fsra1(&mut self) -> PbistFsra1W<PbistFsra1Spec> {
        PbistFsra1W::new(self, 0)
    }
}
#[doc = "Fail status address - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsra1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsra1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistFsra1Spec;
impl crate::RegisterSpec for PbistFsra1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbist_fsra1::R`](R) reader structure"]
impl crate::Readable for PbistFsra1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_fsra1::W`](W) writer structure"]
impl crate::Writable for PbistFsra1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBIST_FSRA1 to value 0"]
impl crate::Resettable for PbistFsra1Spec {
    const RESET_VALUE: u16 = 0;
}
