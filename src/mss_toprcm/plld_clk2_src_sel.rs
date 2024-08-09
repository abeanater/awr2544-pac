#[doc = "Register `PLLD_CLK2_SRC_SEL` reader"]
pub type R = crate::R<PlldClk2SrcSelSpec>;
#[doc = "Register `PLLD_CLK2_SRC_SEL` writer"]
pub type W = crate::W<PlldClk2SrcSelSpec>;
#[doc = "Field `clksrcsel` reader - 11:0\\]
Select line for selecting source clock for PLLDSP_HSDIV_CLKOUT2_MUXED. Data should be loaded as multibit. For example: if Clock source 0x5 should be selected then 0x555 should be configured to the register. Refer to TPR12 clock spec for source clock reference"]
pub type ClksrcselR = crate::FieldReader<u16>;
#[doc = "Field `clksrcsel` writer - 11:0\\]
Select line for selecting source clock for PLLDSP_HSDIV_CLKOUT2_MUXED. Data should be loaded as multibit. For example: if Clock source 0x5 should be selected then 0x555 should be configured to the register. Refer to TPR12 clock spec for source clock reference"]
pub type ClksrcselW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Select line for selecting source clock for PLLDSP_HSDIV_CLKOUT2_MUXED. Data should be loaded as multibit. For example: if Clock source 0x5 should be selected then 0x555 should be configured to the register. Refer to TPR12 clock spec for source clock reference"]
    #[inline(always)]
    pub fn clksrcsel(&self) -> ClksrcselR {
        ClksrcselR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Select line for selecting source clock for PLLDSP_HSDIV_CLKOUT2_MUXED. Data should be loaded as multibit. For example: if Clock source 0x5 should be selected then 0x555 should be configured to the register. Refer to TPR12 clock spec for source clock reference"]
    #[inline(always)]
    #[must_use]
    pub fn clksrcsel(&mut self) -> ClksrcselW<PlldClk2SrcSelSpec> {
        ClksrcselW::new(self, 0)
    }
}
#[doc = "PLLD_CLK2_SRC_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`plld_clk2_src_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plld_clk2_src_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldClk2SrcSelSpec;
impl crate::RegisterSpec for PlldClk2SrcSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plld_clk2_src_sel::R`](R) reader structure"]
impl crate::Readable for PlldClk2SrcSelSpec {}
#[doc = "`write(|w| ..)` method takes [`plld_clk2_src_sel::W`](W) writer structure"]
impl crate::Writable for PlldClk2SrcSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLD_CLK2_SRC_SEL to value 0"]
impl crate::Resettable for PlldClk2SrcSelSpec {
    const RESET_VALUE: u32 = 0;
}
