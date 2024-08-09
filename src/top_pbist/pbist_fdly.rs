#[doc = "Register `PBIST_FDLY` reader"]
pub type R = crate::R<PbistFdlySpec>;
#[doc = "Register `PBIST_FDLY` writer"]
pub type W = crate::W<PbistFdlySpec>;
#[doc = "Field `PBIST_FDLY` reader - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistFdlyR = crate::FieldReader;
#[doc = "Field `PBIST_FDLY` writer - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistFdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn pbist_fdly(&self) -> PbistFdlyR {
        PbistFdlyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_fdly(&mut self) -> PbistFdlyW<PbistFdlySpec> {
        PbistFdlyW::new(self, 0)
    }
}
#[doc = "Fail Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistFdlySpec;
impl crate::RegisterSpec for PbistFdlySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pbist_fdly::R`](R) reader structure"]
impl crate::Readable for PbistFdlySpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_fdly::W`](W) writer structure"]
impl crate::Writable for PbistFdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PBIST_FDLY to value 0"]
impl crate::Resettable for PbistFdlySpec {
    const RESET_VALUE: u8 = 0;
}
