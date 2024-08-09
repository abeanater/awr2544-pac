#[doc = "Register `PLLD_CLK1_STATUS` reader"]
pub type R = crate::R<PlldClk1StatusSpec>;
#[doc = "Register `PLLD_CLK1_STATUS` writer"]
pub type W = crate::W<PlldClk1StatusSpec>;
#[doc = "Field `clkinuse` reader - 7:0\\]
Status shows the source clock slected for GCM switch for PLLCORE_HSDIV_CLKOUT2_MUXED"]
pub type ClkinuseR = crate::FieldReader;
#[doc = "Field `clkinuse` writer - 7:0\\]
Status shows the source clock slected for GCM switch for PLLCORE_HSDIV_CLKOUT2_MUXED"]
pub type ClkinuseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for GCM switch for PLLCORE_HSDIV_CLKOUT2_MUXED"]
    #[inline(always)]
    pub fn clkinuse(&self) -> ClkinuseR {
        ClkinuseR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for GCM switch for PLLCORE_HSDIV_CLKOUT2_MUXED"]
    #[inline(always)]
    #[must_use]
    pub fn clkinuse(&mut self) -> ClkinuseW<PlldClk1StatusSpec> {
        ClkinuseW::new(self, 0)
    }
}
#[doc = "PLLD_CLK1_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`plld_clk1_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plld_clk1_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldClk1StatusSpec;
impl crate::RegisterSpec for PlldClk1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plld_clk1_status::R`](R) reader structure"]
impl crate::Readable for PlldClk1StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`plld_clk1_status::W`](W) writer structure"]
impl crate::Writable for PlldClk1StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLD_CLK1_STATUS to value 0"]
impl crate::Resettable for PlldClk1StatusSpec {
    const RESET_VALUE: u32 = 0;
}
