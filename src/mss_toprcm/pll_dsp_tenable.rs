#[doc = "Register `PLL_DSP_TENABLE` reader"]
pub type R = crate::R<PllDspTenableSpec>;
#[doc = "Register `PLL_DSP_TENABLE` writer"]
pub type W = crate::W<PllDspTenableSpec>;
#[doc = "Field `TENABLE` reader - 0:0\\]
Do not use. TI Reserved."]
pub type TenableR = crate::BitReader;
#[doc = "Field `TENABLE` writer - 0:0\\]
Do not use. TI Reserved."]
pub type TenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn tenable(&self) -> TenableR {
        TenableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tenable(&mut self) -> TenableW<PllDspTenableSpec> {
        TenableW::new(self, 0)
    }
}
#[doc = "PLL_DSP_TENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_tenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_tenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDspTenableSpec;
impl crate::RegisterSpec for PllDspTenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_dsp_tenable::R`](R) reader structure"]
impl crate::Readable for PllDspTenableSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_dsp_tenable::W`](W) writer structure"]
impl crate::Writable for PllDspTenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_DSP_TENABLE to value 0"]
impl crate::Resettable for PllDspTenableSpec {
    const RESET_VALUE: u32 = 0;
}
