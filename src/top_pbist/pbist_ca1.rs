#[doc = "Register `PBIST_CA1` reader"]
pub type R = crate::R<PbistCa1Spec>;
#[doc = "Register `PBIST_CA1` writer"]
pub type W = crate::W<PbistCa1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Constant Address Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ca1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ca1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistCa1Spec;
impl crate::RegisterSpec for PbistCa1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_ca1::R`](R) reader structure"]
impl crate::Readable for PbistCa1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_ca1::W`](W) writer structure"]
impl crate::Writable for PbistCa1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_CA1 to value 0"]
impl crate::Resettable for PbistCa1Spec {
    const RESET_VALUE: u32 = 0;
}
