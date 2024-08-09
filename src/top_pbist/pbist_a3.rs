#[doc = "Register `PBIST_A3` reader"]
pub type R = crate::R<PbistA3Spec>;
#[doc = "Register `PBIST_A3` writer"]
pub type W = crate::W<PbistA3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Variable Address Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_a3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_a3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistA3Spec;
impl crate::RegisterSpec for PbistA3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_a3::R`](R) reader structure"]
impl crate::Readable for PbistA3Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_a3::W`](W) writer structure"]
impl crate::Writable for PbistA3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_A3 to value 0"]
impl crate::Resettable for PbistA3Spec {
    const RESET_VALUE: u32 = 0;
}
