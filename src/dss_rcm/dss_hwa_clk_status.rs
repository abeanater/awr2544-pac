#[doc = "Register `DSS_HWA_CLK_STATUS` reader"]
pub type R = crate::R<DssHwaClkStatusSpec>;
#[doc = "Register `DSS_HWA_CLK_STATUS` writer"]
pub type W = crate::W<DssHwaClkStatusSpec>;
#[doc = "Field `clkinuse` reader - 1:0\\]
Status shows the source clock slected for DSS HWA Clock"]
pub type ClkinuseR = crate::FieldReader;
#[doc = "Field `clkinuse` writer - 1:0\\]
Status shows the source clock slected for DSS HWA Clock"]
pub type ClkinuseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Status shows the source clock slected for DSS HWA Clock"]
    #[inline(always)]
    pub fn clkinuse(&self) -> ClkinuseR {
        ClkinuseR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Status shows the source clock slected for DSS HWA Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clkinuse(&mut self) -> ClkinuseW<DssHwaClkStatusSpec> {
        ClkinuseW::new(self, 0)
    }
}
#[doc = "DSS_HWA_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_clk_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_clk_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssHwaClkStatusSpec;
impl crate::RegisterSpec for DssHwaClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_hwa_clk_status::R`](R) reader structure"]
impl crate::Readable for DssHwaClkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_hwa_clk_status::W`](W) writer structure"]
impl crate::Writable for DssHwaClkStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_HWA_CLK_STATUS to value 0"]
impl crate::Resettable for DssHwaClkStatusSpec {
    const RESET_VALUE: u32 = 0;
}
