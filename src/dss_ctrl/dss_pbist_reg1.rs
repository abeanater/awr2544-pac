#[doc = "Register `DSS_PBIST_REG1` reader"]
pub type R = crate::R<DssPbistReg1Spec>;
#[doc = "Register `DSS_PBIST_REG1` writer"]
pub type W = crate::W<DssPbistReg1Spec>;
#[doc = "Field `dss_pbist_reg1` reader - 31:0\\]
DSP PBIST registers"]
pub type DssPbistReg1R = crate::FieldReader<u32>;
#[doc = "Field `dss_pbist_reg1` writer - 31:0\\]
DSP PBIST registers"]
pub type DssPbistReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
DSP PBIST registers"]
    #[inline(always)]
    pub fn dss_pbist_reg1(&self) -> DssPbistReg1R {
        DssPbistReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
DSP PBIST registers"]
    #[inline(always)]
    #[must_use]
    pub fn dss_pbist_reg1(&mut self) -> DssPbistReg1W<DssPbistReg1Spec> {
        DssPbistReg1W::new(self, 0)
    }
}
#[doc = "DSS_PBIST_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_pbist_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_pbist_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssPbistReg1Spec;
impl crate::RegisterSpec for DssPbistReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_pbist_reg1::R`](R) reader structure"]
impl crate::Readable for DssPbistReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_pbist_reg1::W`](W) writer structure"]
impl crate::Writable for DssPbistReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_PBIST_REG1 to value 0"]
impl crate::Resettable for DssPbistReg1Spec {
    const RESET_VALUE: u32 = 0;
}
