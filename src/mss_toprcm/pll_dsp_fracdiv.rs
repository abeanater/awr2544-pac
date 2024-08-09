#[doc = "Register `PLL_DSP_FRACDIV` reader"]
pub type R = crate::R<PllDspFracdivSpec>;
#[doc = "Register `PLL_DSP_FRACDIV` writer"]
pub type W = crate::W<PllDspFracdivSpec>;
#[doc = "Field `FRACTIONALM` reader - 17:0\\]
Do not use. TI Reserved."]
pub type FractionalmR = crate::FieldReader<u32>;
#[doc = "Field `FRACTIONALM` writer - 17:0\\]
Do not use. TI Reserved."]
pub type FractionalmW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `REGSD` reader - 31:24\\]
Do not use. TI Reserved."]
pub type RegsdR = crate::FieldReader;
#[doc = "Field `REGSD` writer - 31:24\\]
Do not use. TI Reserved."]
pub type RegsdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:17 - 17:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn fractionalm(&self) -> FractionalmR {
        FractionalmR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn regsd(&self) -> RegsdR {
        RegsdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - 17:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn fractionalm(&mut self) -> FractionalmW<PllDspFracdivSpec> {
        FractionalmW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn regsd(&mut self) -> RegsdW<PllDspFracdivSpec> {
        RegsdW::new(self, 24)
    }
}
#[doc = "PLL_DSP_FRACDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_fracdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_fracdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDspFracdivSpec;
impl crate::RegisterSpec for PllDspFracdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_dsp_fracdiv::R`](R) reader structure"]
impl crate::Readable for PllDspFracdivSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_dsp_fracdiv::W`](W) writer structure"]
impl crate::Writable for PllDspFracdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_DSP_FRACDIV to value 0"]
impl crate::Resettable for PllDspFracdivSpec {
    const RESET_VALUE: u32 = 0;
}
