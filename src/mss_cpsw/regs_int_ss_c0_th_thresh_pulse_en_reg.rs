#[doc = "Register `REGS_INT_SS_C0_TH_THRESH_PULSE_EN_REG` reader"]
pub type R = crate::R<RegsIntSsC0ThThreshPulseEnRegSpec>;
#[doc = "Register `REGS_INT_SS_C0_TH_THRESH_PULSE_EN_REG` writer"]
pub type W = crate::W<RegsIntSsC0ThThreshPulseEnRegSpec>;
#[doc = "Field `THOST_THRESHOLD_PULSE` reader - 7:0\\]
THost Threshold Pulse Interrupt Enable Register"]
pub type ThostThresholdPulseR = crate::FieldReader;
#[doc = "Field `THOST_THRESHOLD_PULSE` writer - 7:0\\]
THost Threshold Pulse Interrupt Enable Register"]
pub type ThostThresholdPulseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
THost Threshold Pulse Interrupt Enable Register"]
    #[inline(always)]
    pub fn thost_threshold_pulse(&self) -> ThostThresholdPulseR {
        ThostThresholdPulseR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
THost Threshold Pulse Interrupt Enable Register"]
    #[inline(always)]
    #[must_use]
    pub fn thost_threshold_pulse(
        &mut self,
    ) -> ThostThresholdPulseW<RegsIntSsC0ThThreshPulseEnRegSpec> {
        ThostThresholdPulseW::new(self, 0)
    }
}
#[doc = "Core 0 THost Threshold Pulse Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_th_thresh_pulse_en_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_th_thresh_pulse_en_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsIntSsC0ThThreshPulseEnRegSpec;
impl crate::RegisterSpec for RegsIntSsC0ThThreshPulseEnRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_int_ss_c0_th_thresh_pulse_en_reg::R`](R) reader structure"]
impl crate::Readable for RegsIntSsC0ThThreshPulseEnRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs_int_ss_c0_th_thresh_pulse_en_reg::W`](W) writer structure"]
impl crate::Writable for RegsIntSsC0ThThreshPulseEnRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_INT_SS_C0_TH_THRESH_PULSE_EN_REG to value 0"]
impl crate::Resettable for RegsIntSsC0ThThreshPulseEnRegSpec {
    const RESET_VALUE: u32 = 0;
}
