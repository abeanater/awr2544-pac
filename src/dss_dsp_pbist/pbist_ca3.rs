#[doc = "Register `PBIST_CA3` reader"]
pub type R = crate::R<PbistCa3Spec>;
#[doc = "Register `PBIST_CA3` writer"]
pub type W = crate::W<PbistCa3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Constant Address Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ca3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ca3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistCa3Spec;
impl crate::RegisterSpec for PbistCa3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_ca3::R`](R) reader structure"]
impl crate::Readable for PbistCa3Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_ca3::W`](W) writer structure"]
impl crate::Writable for PbistCa3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_CA3 to value 0"]
impl crate::Resettable for PbistCa3Spec {
    const RESET_VALUE: u32 = 0;
}
