#[doc = "Register `PBIST_CI1` reader"]
pub type R = crate::R<PbistCi1Spec>;
#[doc = "Register `PBIST_CI1` writer"]
pub type W = crate::W<PbistCi1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Constant Increment Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_ci1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_ci1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistCi1Spec;
impl crate::RegisterSpec for PbistCi1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_ci1::R`](R) reader structure"]
impl crate::Readable for PbistCi1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_ci1::W`](W) writer structure"]
impl crate::Writable for PbistCi1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_CI1 to value 0"]
impl crate::Resettable for PbistCi1Spec {
    const RESET_VALUE: u32 = 0;
}
