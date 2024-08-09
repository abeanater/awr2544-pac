#[doc = "Register `PBIST_L3` reader"]
pub type R = crate::R<PbistL3Spec>;
#[doc = "Register `PBIST_L3` writer"]
pub type W = crate::W<PbistL3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Variable Loop Count Register L3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_l3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_l3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistL3Spec;
impl crate::RegisterSpec for PbistL3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_l3::R`](R) reader structure"]
impl crate::Readable for PbistL3Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_l3::W`](W) writer structure"]
impl crate::Writable for PbistL3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_L3 to value 0"]
impl crate::Resettable for PbistL3Spec {
    const RESET_VALUE: u32 = 0;
}
