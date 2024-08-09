#[doc = "Register `TRCCLKOUT_CLK_SRC_SEL` reader"]
pub type R = crate::R<TrcclkoutClkSrcSelSpec>;
#[doc = "Register `TRCCLKOUT_CLK_SRC_SEL` writer"]
pub type W = crate::W<TrcclkoutClkSrcSelSpec>;
#[doc = "Field `clksrcsel` reader - 11:0\\]
Select line for selecting source clock for TRC Clkout Data should be loaded as multibit. For example: if Clock source 0x5 should be selected then 0x555 should be configured to the register. Refer to TPR12 clock spec for source clock reference"]
pub type ClksrcselR = crate::FieldReader<u16>;
#[doc = "Field `clksrcsel` writer - 11:0\\]
Select line for selecting source clock for TRC Clkout Data should be loaded as multibit. For example: if Clock source 0x5 should be selected then 0x555 should be configured to the register. Refer to TPR12 clock spec for source clock reference"]
pub type ClksrcselW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Select line for selecting source clock for TRC Clkout Data should be loaded as multibit. For example: if Clock source 0x5 should be selected then 0x555 should be configured to the register. Refer to TPR12 clock spec for source clock reference"]
    #[inline(always)]
    pub fn clksrcsel(&self) -> ClksrcselR {
        ClksrcselR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Select line for selecting source clock for TRC Clkout Data should be loaded as multibit. For example: if Clock source 0x5 should be selected then 0x555 should be configured to the register. Refer to TPR12 clock spec for source clock reference"]
    #[inline(always)]
    #[must_use]
    pub fn clksrcsel(&mut self) -> ClksrcselW<TrcclkoutClkSrcSelSpec> {
        ClksrcselW::new(self, 0)
    }
}
#[doc = "TRCCLKOUT_CLK_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclkout_clk_src_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclkout_clk_src_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcclkoutClkSrcSelSpec;
impl crate::RegisterSpec for TrcclkoutClkSrcSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcclkout_clk_src_sel::R`](R) reader structure"]
impl crate::Readable for TrcclkoutClkSrcSelSpec {}
#[doc = "`write(|w| ..)` method takes [`trcclkout_clk_src_sel::W`](W) writer structure"]
impl crate::Writable for TrcclkoutClkSrcSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCLKOUT_CLK_SRC_SEL to value 0"]
impl crate::Resettable for TrcclkoutClkSrcSelSpec {
    const RESET_VALUE: u32 = 0;
}
