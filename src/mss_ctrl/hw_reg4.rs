#[doc = "Register `HW_REG4` reader"]
pub type R = crate::R<HwReg4Spec>;
#[doc = "Register `HW_REG4` writer"]
pub type W = crate::W<HwReg4Spec>;
#[doc = "Field `mss_pcrA_timeout_cfg` reader - 31:0\\]
Configures the timeout value for MSS_PCRA."]
pub type MssPcrATimeoutCfgR = crate::FieldReader<u32>;
#[doc = "Field `mss_pcrA_timeout_cfg` writer - 31:0\\]
Configures the timeout value for MSS_PCRA."]
pub type MssPcrATimeoutCfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Configures the timeout value for MSS_PCRA."]
    #[inline(always)]
    pub fn mss_pcr_a_timeout_cfg(&self) -> MssPcrATimeoutCfgR {
        MssPcrATimeoutCfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Configures the timeout value for MSS_PCRA."]
    #[inline(always)]
    #[must_use]
    pub fn mss_pcr_a_timeout_cfg(&mut self) -> MssPcrATimeoutCfgW<HwReg4Spec> {
        MssPcrATimeoutCfgW::new(self, 0)
    }
}
#[doc = "HW_REG4\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwReg4Spec;
impl crate::RegisterSpec for HwReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_reg4::R`](R) reader structure"]
impl crate::Readable for HwReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_reg4::W`](W) writer structure"]
impl crate::Writable for HwReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_REG4 to value 0"]
impl crate::Resettable for HwReg4Spec {
    const RESET_VALUE: u32 = 0;
}
