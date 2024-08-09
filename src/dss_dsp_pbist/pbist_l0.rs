#[doc = "Register `PBIST_L0` reader"]
pub type R = crate::R<PbistL0Spec>;
#[doc = "Register `PBIST_L0` writer"]
pub type W = crate::W<PbistL0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Variable Loop Count Register L0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_l0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_l0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistL0Spec;
impl crate::RegisterSpec for PbistL0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_l0::R`](R) reader structure"]
impl crate::Readable for PbistL0Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_l0::W`](W) writer structure"]
impl crate::Writable for PbistL0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_L0 to value 0"]
impl crate::Resettable for PbistL0Spec {
    const RESET_VALUE: u32 = 0;
}
