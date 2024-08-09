#[doc = "Register `PBIST_FSRDL1` reader"]
pub type R = crate::R<PbistFsrdl1Spec>;
#[doc = "Register `PBIST_FSRDL1` writer"]
pub type W = crate::W<PbistFsrdl1Spec>;
#[doc = "Field `PBIST_FSRDL1` reader - 31:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistFsrdl1R = crate::FieldReader<u32>;
#[doc = "Field `PBIST_FSRDL1` writer - 31:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistFsrdl1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn pbist_fsrdl1(&self) -> PbistFsrdl1R {
        PbistFsrdl1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_fsrdl1(&mut self) -> PbistFsrdl1W<PbistFsrdl1Spec> {
        PbistFsrdl1W::new(self, 0)
    }
}
#[doc = "Fail status Data - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsrdl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsrdl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistFsrdl1Spec;
impl crate::RegisterSpec for PbistFsrdl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_fsrdl1::R`](R) reader structure"]
impl crate::Readable for PbistFsrdl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_fsrdl1::W`](W) writer structure"]
impl crate::Writable for PbistFsrdl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_FSRDL1 to value 0"]
impl crate::Resettable for PbistFsrdl1Spec {
    const RESET_VALUE: u32 = 0;
}
