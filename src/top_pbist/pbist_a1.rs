#[doc = "Register `PBIST_A1` reader"]
pub type R = crate::R<PbistA1Spec>;
#[doc = "Register `PBIST_A1` writer"]
pub type W = crate::W<PbistA1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Variable Address Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_a1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_a1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistA1Spec;
impl crate::RegisterSpec for PbistA1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_a1::R`](R) reader structure"]
impl crate::Readable for PbistA1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_a1::W`](W) writer structure"]
impl crate::Writable for PbistA1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_A1 to value 0"]
impl crate::Resettable for PbistA1Spec {
    const RESET_VALUE: u32 = 0;
}
