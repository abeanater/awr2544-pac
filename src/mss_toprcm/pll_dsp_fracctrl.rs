#[doc = "Register `PLL_DSP_FRACCTRL` reader"]
pub type R = crate::R<PllDspFracctrlSpec>;
#[doc = "Register `PLL_DSP_FRACCTRL` writer"]
pub type W = crate::W<PllDspFracctrlSpec>;
#[doc = "Field `DeltaMStepFraction` reader - 17:0\\]
Do not use. TI Reserved."]
pub type DeltaMstepFractionR = crate::FieldReader<u32>;
#[doc = "Field `DeltaMStepFraction` writer - 17:0\\]
Do not use. TI Reserved."]
pub type DeltaMstepFractionW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `DeltaMStepInteger` reader - 20:18\\]
Do not use. TI Reserved."]
pub type DeltaMstepIntegerR = crate::FieldReader;
#[doc = "Field `DeltaMStepInteger` writer - 20:18\\]
Do not use. TI Reserved."]
pub type DeltaMstepIntegerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ModFreqDividerMantissa` reader - 27:21\\]
Do not use. TI Reserved."]
pub type ModFreqDividerMantissaR = crate::FieldReader;
#[doc = "Field `ModFreqDividerMantissa` writer - 27:21\\]
Do not use. TI Reserved."]
pub type ModFreqDividerMantissaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ModFreqDividerExponent` reader - 30:28\\]
Do not use. TI Reserved."]
pub type ModFreqDividerExponentR = crate::FieldReader;
#[doc = "Field `ModFreqDividerExponent` writer - 30:28\\]
Do not use. TI Reserved."]
pub type ModFreqDividerExponentW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DOWNSPREAD` reader - 31:31\\]
Do not use. TI Reserved."]
pub type DownspreadR = crate::BitReader;
#[doc = "Field `DOWNSPREAD` writer - 31:31\\]
Do not use. TI Reserved."]
pub type DownspreadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - 17:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn delta_mstep_fraction(&self) -> DeltaMstepFractionR {
        DeltaMstepFractionR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn delta_mstep_integer(&self) -> DeltaMstepIntegerR {
        DeltaMstepIntegerR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:27 - 27:21\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn mod_freq_divider_mantissa(&self) -> ModFreqDividerMantissaR {
        ModFreqDividerMantissaR::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn mod_freq_divider_exponent(&self) -> ModFreqDividerExponentR {
        ModFreqDividerExponentR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn downspread(&self) -> DownspreadR {
        DownspreadR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - 17:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn delta_mstep_fraction(&mut self) -> DeltaMstepFractionW<PllDspFracctrlSpec> {
        DeltaMstepFractionW::new(self, 0)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn delta_mstep_integer(&mut self) -> DeltaMstepIntegerW<PllDspFracctrlSpec> {
        DeltaMstepIntegerW::new(self, 18)
    }
    #[doc = "Bits 21:27 - 27:21\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn mod_freq_divider_mantissa(&mut self) -> ModFreqDividerMantissaW<PllDspFracctrlSpec> {
        ModFreqDividerMantissaW::new(self, 21)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn mod_freq_divider_exponent(&mut self) -> ModFreqDividerExponentW<PllDspFracctrlSpec> {
        ModFreqDividerExponentW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn downspread(&mut self) -> DownspreadW<PllDspFracctrlSpec> {
        DownspreadW::new(self, 31)
    }
}
#[doc = "PLL_DSP_FRACCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_fracctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_fracctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDspFracctrlSpec;
impl crate::RegisterSpec for PllDspFracctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_dsp_fracctrl::R`](R) reader structure"]
impl crate::Readable for PllDspFracctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_dsp_fracctrl::W`](W) writer structure"]
impl crate::Writable for PllDspFracctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_DSP_FRACCTRL to value 0"]
impl crate::Resettable for PllDspFracctrlSpec {
    const RESET_VALUE: u32 = 0;
}
