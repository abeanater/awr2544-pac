#[doc = "Register `DSS_PBIST_REG0` reader"]
pub type R = crate::R<DssPbistReg0Spec>;
#[doc = "Register `DSS_PBIST_REG0` writer"]
pub type W = crate::W<DssPbistReg0Spec>;
#[doc = "Field `dss_pbist_reg0` reader - 31:0\\]
DSP PBIST registers"]
pub type DssPbistReg0R = crate::FieldReader<u32>;
#[doc = "Field `dss_pbist_reg0` writer - 31:0\\]
DSP PBIST registers"]
pub type DssPbistReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
DSP PBIST registers"]
    #[inline(always)]
    pub fn dss_pbist_reg0(&self) -> DssPbistReg0R {
        DssPbistReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
DSP PBIST registers"]
    #[inline(always)]
    #[must_use]
    pub fn dss_pbist_reg0(&mut self) -> DssPbistReg0W<DssPbistReg0Spec> {
        DssPbistReg0W::new(self, 0)
    }
}
#[doc = "DSS_PBIST_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_pbist_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_pbist_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssPbistReg0Spec;
impl crate::RegisterSpec for DssPbistReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_pbist_reg0::R`](R) reader structure"]
impl crate::Readable for DssPbistReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_pbist_reg0::W`](W) writer structure"]
impl crate::Writable for DssPbistReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_PBIST_REG0 to value 0"]
impl crate::Resettable for DssPbistReg0Spec {
    const RESET_VALUE: u32 = 0;
}
