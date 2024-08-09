#[doc = "Register `DSS_HWA_CLK_SRC_SEL` reader"]
pub type R = crate::R<DssHwaClkSrcSelSpec>;
#[doc = "Register `DSS_HWA_CLK_SRC_SEL` writer"]
pub type W = crate::W<DssHwaClkSrcSelSpec>;
#[doc = "Field `clksrcsel` reader - 2:0\\]
Select line for selecting source clock for DSS HWA. Data should be loaded as multibit. Write 3'b000 : TOPRCM_CR5_CLK Write 3'b111 : TOPRCM_SYS_CLK"]
pub type ClksrcselR = crate::FieldReader;
#[doc = "Field `clksrcsel` writer - 2:0\\]
Select line for selecting source clock for DSS HWA. Data should be loaded as multibit. Write 3'b000 : TOPRCM_CR5_CLK Write 3'b111 : TOPRCM_SYS_CLK"]
pub type ClksrcselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Select line for selecting source clock for DSS HWA. Data should be loaded as multibit. Write 3'b000 : TOPRCM_CR5_CLK Write 3'b111 : TOPRCM_SYS_CLK"]
    #[inline(always)]
    pub fn clksrcsel(&self) -> ClksrcselR {
        ClksrcselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Select line for selecting source clock for DSS HWA. Data should be loaded as multibit. Write 3'b000 : TOPRCM_CR5_CLK Write 3'b111 : TOPRCM_SYS_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn clksrcsel(&mut self) -> ClksrcselW<DssHwaClkSrcSelSpec> {
        ClksrcselW::new(self, 0)
    }
}
#[doc = "DSS_HWA_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_clk_src_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_clk_src_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssHwaClkSrcSelSpec;
impl crate::RegisterSpec for DssHwaClkSrcSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_hwa_clk_src_sel::R`](R) reader structure"]
impl crate::Readable for DssHwaClkSrcSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_hwa_clk_src_sel::W`](W) writer structure"]
impl crate::Writable for DssHwaClkSrcSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_HWA_CLK_SRC_SEL to value 0"]
impl crate::Resettable for DssHwaClkSrcSelSpec {
    const RESET_VALUE: u32 = 0;
}
