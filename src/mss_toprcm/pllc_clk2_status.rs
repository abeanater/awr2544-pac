#[doc = "Register `PLLC_CLK2_STATUS` reader"]
pub type R = crate::R<PllcClk2StatusSpec>;
#[doc = "Register `PLLC_CLK2_STATUS` writer"]
pub type W = crate::W<PllcClk2StatusSpec>;
#[doc = "Field `clkinuse` reader - 7:0\\]
Status shows the source clock slected for GCM switch for PLLCORE_HSDIV_CLK2"]
pub type ClkinuseR = crate::FieldReader;
#[doc = "Field `clkinuse` writer - 7:0\\]
Status shows the source clock slected for GCM switch for PLLCORE_HSDIV_CLK2"]
pub type ClkinuseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for GCM switch for PLLCORE_HSDIV_CLK2"]
    #[inline(always)]
    pub fn clkinuse(&self) -> ClkinuseR {
        ClkinuseR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for GCM switch for PLLCORE_HSDIV_CLK2"]
    #[inline(always)]
    #[must_use]
    pub fn clkinuse(&mut self) -> ClkinuseW<PllcClk2StatusSpec> {
        ClkinuseW::new(self, 0)
    }
}
#[doc = "PLLC_CLK2_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`pllc_clk2_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllc_clk2_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcClk2StatusSpec;
impl crate::RegisterSpec for PllcClk2StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllc_clk2_status::R`](R) reader structure"]
impl crate::Readable for PllcClk2StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pllc_clk2_status::W`](W) writer structure"]
impl crate::Writable for PllcClk2StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLC_CLK2_STATUS to value 0"]
impl crate::Resettable for PllcClk2StatusSpec {
    const RESET_VALUE: u32 = 0;
}
