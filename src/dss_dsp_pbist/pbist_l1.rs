#[doc = "Register `PBIST_L1` reader"]
pub type R = crate::R<PbistL1Spec>;
#[doc = "Register `PBIST_L1` writer"]
pub type W = crate::W<PbistL1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Variable Loop Count Register L1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_l1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_l1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistL1Spec;
impl crate::RegisterSpec for PbistL1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_l1::R`](R) reader structure"]
impl crate::Readable for PbistL1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_l1::W`](W) writer structure"]
impl crate::Writable for PbistL1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_L1 to value 0"]
impl crate::Resettable for PbistL1Spec {
    const RESET_VALUE: u32 = 0;
}
