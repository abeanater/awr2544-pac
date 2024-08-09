#[doc = "Register `PMICCLKOUT_DCDC_CTRL` reader"]
pub type R = crate::R<PmicclkoutDcdcCtrlSpec>;
#[doc = "Register `PMICCLKOUT_DCDC_CTRL` writer"]
pub type W = crate::W<PmicclkoutDcdcCtrlSpec>;
#[doc = "Field `dcdc_clk_en` reader - 0:0\\]
PMIC Clockout DCDC Clock Enable"]
pub type DcdcClkEnR = crate::BitReader;
#[doc = "Field `dcdc_clk_en` writer - 0:0\\]
PMIC Clockout DCDC Clock Enable"]
pub type DcdcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dither_en` reader - 1:1\\]
PMIC Clockout DCDC Clock Dither Enable"]
pub type DitherEnR = crate::BitReader;
#[doc = "Field `dither_en` writer - 1:1\\]
PMIC Clockout DCDC Clock Dither Enable"]
pub type DitherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `freq_acc_mode` reader - 2:2\\]
PMIC Clockout DCDC Freq Acc Enable"]
pub type FreqAccModeR = crate::BitReader;
#[doc = "Field `freq_acc_mode` writer - 2:2\\]
PMIC Clockout DCDC Freq Acc Enable"]
pub type FreqAccModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_assert` reader - 6:4\\]
Reset control for PMIC DCDC Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
pub type ResetAssertR = crate::FieldReader;
#[doc = "Field `reset_assert` writer - 6:4\\]
Reset control for PMIC DCDC Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
pub type ResetAssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `min_freq_thr` reader - 15:8\\]
PMIC Clockout DCDC Minimum Frequency Threshold"]
pub type MinFreqThrR = crate::FieldReader;
#[doc = "Field `min_freq_thr` writer - 15:8\\]
PMIC Clockout DCDC Minimum Frequency Threshold"]
pub type MinFreqThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `max_freq_thr` reader - 23:16\\]
PMIC Clockout DCDC Maximum Frequency Threshold"]
pub type MaxFreqThrR = crate::FieldReader;
#[doc = "Field `max_freq_thr` writer - 23:16\\]
PMIC Clockout DCDC Maximum Frequency Threshold"]
pub type MaxFreqThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PMIC Clockout DCDC Clock Enable"]
    #[inline(always)]
    pub fn dcdc_clk_en(&self) -> DcdcClkEnR {
        DcdcClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PMIC Clockout DCDC Clock Dither Enable"]
    #[inline(always)]
    pub fn dither_en(&self) -> DitherEnR {
        DitherEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
PMIC Clockout DCDC Freq Acc Enable"]
    #[inline(always)]
    pub fn freq_acc_mode(&self) -> FreqAccModeR {
        FreqAccModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Reset control for PMIC DCDC Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
    #[inline(always)]
    pub fn reset_assert(&self) -> ResetAssertR {
        ResetAssertR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
PMIC Clockout DCDC Minimum Frequency Threshold"]
    #[inline(always)]
    pub fn min_freq_thr(&self) -> MinFreqThrR {
        MinFreqThrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
PMIC Clockout DCDC Maximum Frequency Threshold"]
    #[inline(always)]
    pub fn max_freq_thr(&self) -> MaxFreqThrR {
        MaxFreqThrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PMIC Clockout DCDC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_clk_en(&mut self) -> DcdcClkEnW<PmicclkoutDcdcCtrlSpec> {
        DcdcClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
PMIC Clockout DCDC Clock Dither Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dither_en(&mut self) -> DitherEnW<PmicclkoutDcdcCtrlSpec> {
        DitherEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
PMIC Clockout DCDC Freq Acc Enable"]
    #[inline(always)]
    #[must_use]
    pub fn freq_acc_mode(&mut self) -> FreqAccModeW<PmicclkoutDcdcCtrlSpec> {
        FreqAccModeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Reset control for PMIC DCDC Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn reset_assert(&mut self) -> ResetAssertW<PmicclkoutDcdcCtrlSpec> {
        ResetAssertW::new(self, 4)
    }
    #[doc = "Bits 8:15 - 15:8\\]
PMIC Clockout DCDC Minimum Frequency Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn min_freq_thr(&mut self) -> MinFreqThrW<PmicclkoutDcdcCtrlSpec> {
        MinFreqThrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
PMIC Clockout DCDC Maximum Frequency Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn max_freq_thr(&mut self) -> MaxFreqThrW<PmicclkoutDcdcCtrlSpec> {
        MaxFreqThrW::new(self, 16)
    }
}
#[doc = "PMICCLKOUT_DCDC_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pmicclkout_dcdc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmicclkout_dcdc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmicclkoutDcdcCtrlSpec;
impl crate::RegisterSpec for PmicclkoutDcdcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmicclkout_dcdc_ctrl::R`](R) reader structure"]
impl crate::Readable for PmicclkoutDcdcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pmicclkout_dcdc_ctrl::W`](W) writer structure"]
impl crate::Writable for PmicclkoutDcdcCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMICCLKOUT_DCDC_CTRL to value 0"]
impl crate::Resettable for PmicclkoutDcdcCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
