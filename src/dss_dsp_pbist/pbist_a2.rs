#[doc = "Register `PBIST_A2` reader"]
pub type R = crate::R<PbistA2Spec>;
#[doc = "Register `PBIST_A2` writer"]
pub type W = crate::W<PbistA2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Variable Address Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_a2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_a2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistA2Spec;
impl crate::RegisterSpec for PbistA2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_a2::R`](R) reader structure"]
impl crate::Readable for PbistA2Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_a2::W`](W) writer structure"]
impl crate::Writable for PbistA2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_A2 to value 0"]
impl crate::Resettable for PbistA2Spec {
    const RESET_VALUE: u32 = 0;
}
