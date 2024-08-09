#[doc = "Register `PLL_DSP_HSDIVIDER_RSTCTRL` reader"]
pub type R = crate::R<PllDspHsdividerRstctrlSpec>;
#[doc = "Register `PLL_DSP_HSDIVIDER_RSTCTRL` writer"]
pub type W = crate::W<PllDspHsdividerRstctrlSpec>;
#[doc = "Field `assert` reader - 2:0\\]
SW Reset override for the HSDIVIDER Write 3'b111 : Override is enabled and Reset is asserted"]
pub type AssertR = crate::FieldReader;
#[doc = "Field `assert` writer - 2:0\\]
SW Reset override for the HSDIVIDER Write 3'b111 : Override is enabled and Reset is asserted"]
pub type AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
SW Reset override for the HSDIVIDER Write 3'b111 : Override is enabled and Reset is asserted"]
    #[inline(always)]
    pub fn assert(&self) -> AssertR {
        AssertR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
SW Reset override for the HSDIVIDER Write 3'b111 : Override is enabled and Reset is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn assert(&mut self) -> AssertW<PllDspHsdividerRstctrlSpec> {
        AssertW::new(self, 0)
    }
}
#[doc = "PLL_DSP_HSDIVIDER_RSTCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_hsdivider_rstctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_hsdivider_rstctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDspHsdividerRstctrlSpec;
impl crate::RegisterSpec for PllDspHsdividerRstctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_dsp_hsdivider_rstctrl::R`](R) reader structure"]
impl crate::Readable for PllDspHsdividerRstctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_dsp_hsdivider_rstctrl::W`](W) writer structure"]
impl crate::Writable for PllDspHsdividerRstctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_DSP_HSDIVIDER_RSTCTRL to value 0"]
impl crate::Resettable for PllDspHsdividerRstctrlSpec {
    const RESET_VALUE: u32 = 0;
}
