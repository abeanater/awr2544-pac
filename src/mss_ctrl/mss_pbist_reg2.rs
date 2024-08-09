#[doc = "Register `MSS_PBIST_REG2` reader"]
pub type R = crate::R<MssPbistReg2Spec>;
#[doc = "Register `MSS_PBIST_REG2` writer"]
pub type W = crate::W<MssPbistReg2Spec>;
#[doc = "Field `pbist_reg` reader - "]
pub type PbistRegR = crate::FieldReader<u32>;
#[doc = "Field `pbist_reg` writer - "]
pub type PbistRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pbist_reg(&self) -> PbistRegR {
        PbistRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_reg(&mut self) -> PbistRegW<MssPbistReg2Spec> {
        PbistRegW::new(self, 0)
    }
}
#[doc = "MSS_PBIST_REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_pbist_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_pbist_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssPbistReg2Spec;
impl crate::RegisterSpec for MssPbistReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_pbist_reg2::R`](R) reader structure"]
impl crate::Readable for MssPbistReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_pbist_reg2::W`](W) writer structure"]
impl crate::Writable for MssPbistReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_PBIST_REG2 to value 0"]
impl crate::Resettable for MssPbistReg2Spec {
    const RESET_VALUE: u32 = 0;
}
