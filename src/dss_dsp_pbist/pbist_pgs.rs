#[doc = "Register `PBIST_PGS` reader"]
pub type R = crate::R<PbistPgsSpec>;
#[doc = "Register `PBIST_PGS` writer"]
pub type W = crate::W<PbistPgsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PAGE/PGS\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_pgs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_pgs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistPgsSpec;
impl crate::RegisterSpec for PbistPgsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_pgs::R`](R) reader structure"]
impl crate::Readable for PbistPgsSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_pgs::W`](W) writer structure"]
impl crate::Writable for PbistPgsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_PGS to value 0"]
impl crate::Resettable for PbistPgsSpec {
    const RESET_VALUE: u32 = 0;
}
