#[doc = "Register `MSS_CPSW_MII_CLK_STATUS` reader"]
pub type R = crate::R<MssCpswMiiClkStatusSpec>;
#[doc = "Register `MSS_CPSW_MII_CLK_STATUS` writer"]
pub type W = crate::W<MssCpswMiiClkStatusSpec>;
#[doc = "Field `clkinuse` reader - 7:0\\]
Status shows the source clock slected for CPSW_MII"]
pub type ClkinuseR = crate::FieldReader;
#[doc = "Field `clkinuse` writer - 7:0\\]
Status shows the source clock slected for CPSW_MII"]
pub type ClkinuseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for CPSW_MII"]
    #[inline(always)]
    pub fn clkinuse(&self) -> ClkinuseR {
        ClkinuseR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for CPSW_MII"]
    #[inline(always)]
    #[must_use]
    pub fn clkinuse(&mut self) -> ClkinuseW<MssCpswMiiClkStatusSpec> {
        ClkinuseW::new(self, 0)
    }
}
#[doc = "MSS_CPSW_MII_CLK_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_cpsw_mii_clk_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_cpsw_mii_clk_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssCpswMiiClkStatusSpec;
impl crate::RegisterSpec for MssCpswMiiClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_cpsw_mii_clk_status::R`](R) reader structure"]
impl crate::Readable for MssCpswMiiClkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_cpsw_mii_clk_status::W`](W) writer structure"]
impl crate::Writable for MssCpswMiiClkStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_CPSW_MII_CLK_STATUS to value 0"]
impl crate::Resettable for MssCpswMiiClkStatusSpec {
    const RESET_VALUE: u32 = 0;
}
