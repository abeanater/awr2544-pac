#[doc = "Register `PBIST_FSRCR1` reader"]
pub type R = crate::R<PbistFsrcr1Spec>;
#[doc = "Register `PBIST_FSRCR1` writer"]
pub type W = crate::W<PbistFsrcr1Spec>;
#[doc = "Field `PBIST_FSRCR1` reader - 3:0\\]
Fail Status Count - Port 1 These registers keep count of the number of failures observed during the memory self-test. The PBIST controller stops executing the memory self-test whenever a failure occurs in any memory instance for any of the test algorithms. The value in gets incremented by one whenever a failure occurs"]
pub type PbistFsrcr1R = crate::FieldReader;
#[doc = "Field `PBIST_FSRCR1` writer - 3:0\\]
Fail Status Count - Port 1 These registers keep count of the number of failures observed during the memory self-test. The PBIST controller stops executing the memory self-test whenever a failure occurs in any memory instance for any of the test algorithms. The value in gets incremented by one whenever a failure occurs"]
pub type PbistFsrcr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Fail Status Count - Port 1 These registers keep count of the number of failures observed during the memory self-test. The PBIST controller stops executing the memory self-test whenever a failure occurs in any memory instance for any of the test algorithms. The value in gets incremented by one whenever a failure occurs"]
    #[inline(always)]
    pub fn pbist_fsrcr1(&self) -> PbistFsrcr1R {
        PbistFsrcr1R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Fail Status Count - Port 1 These registers keep count of the number of failures observed during the memory self-test. The PBIST controller stops executing the memory self-test whenever a failure occurs in any memory instance for any of the test algorithms. The value in gets incremented by one whenever a failure occurs"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_fsrcr1(&mut self) -> PbistFsrcr1W<PbistFsrcr1Spec> {
        PbistFsrcr1W::new(self, 0)
    }
}
#[doc = "Fail Count fail - port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_fsrcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_fsrcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistFsrcr1Spec;
impl crate::RegisterSpec for PbistFsrcr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pbist_fsrcr1::R`](R) reader structure"]
impl crate::Readable for PbistFsrcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbist_fsrcr1::W`](W) writer structure"]
impl crate::Writable for PbistFsrcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PBIST_FSRCR1 to value 0"]
impl crate::Resettable for PbistFsrcr1Spec {
    const RESET_VALUE: u8 = 0;
}
