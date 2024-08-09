#[doc = "Register `PBIST_DE10` reader"]
pub type R = crate::R<PbistDe10Spec>;
#[doc = "Register `PBIST_DE10` writer"]
pub type W = crate::W<PbistDe10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DE0 Data Register 16 (D0)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_de10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_de10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistDe10Spec;
impl crate::RegisterSpec for PbistDe10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_de10::R`](R) reader structure"]
impl crate::Readable for PbistDe10Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_de10::W`](W) writer structure"]
impl crate::Writable for PbistDe10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_DE10 to value 0"]
impl crate::Resettable for PbistDe10Spec {
    const RESET_VALUE: u32 = 0;
}
