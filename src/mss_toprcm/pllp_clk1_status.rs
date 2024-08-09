#[doc = "Register `PLLP_CLK1_STATUS` reader"]
pub type R = crate::R<PllpClk1StatusSpec>;
#[doc = "Register `PLLP_CLK1_STATUS` writer"]
pub type W = crate::W<PllpClk1StatusSpec>;
#[doc = "Field `clkinuse` reader - 7:0\\]
Status shows the source clock slected for GCM switch for PLLPER_HSDIV_CLK1"]
pub type ClkinuseR = crate::FieldReader;
#[doc = "Field `clkinuse` writer - 7:0\\]
Status shows the source clock slected for GCM switch for PLLPER_HSDIV_CLK1"]
pub type ClkinuseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for GCM switch for PLLPER_HSDIV_CLK1"]
    #[inline(always)]
    pub fn clkinuse(&self) -> ClkinuseR {
        ClkinuseR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Status shows the source clock slected for GCM switch for PLLPER_HSDIV_CLK1"]
    #[inline(always)]
    #[must_use]
    pub fn clkinuse(&mut self) -> ClkinuseW<PllpClk1StatusSpec> {
        ClkinuseW::new(self, 0)
    }
}
#[doc = "PLLP_CLK1_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`pllp_clk1_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllp_clk1_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllpClk1StatusSpec;
impl crate::RegisterSpec for PllpClk1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllp_clk1_status::R`](R) reader structure"]
impl crate::Readable for PllpClk1StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pllp_clk1_status::W`](W) writer structure"]
impl crate::Writable for PllpClk1StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLP_CLK1_STATUS to value 0"]
impl crate::Resettable for PllpClk1StatusSpec {
    const RESET_VALUE: u32 = 0;
}
