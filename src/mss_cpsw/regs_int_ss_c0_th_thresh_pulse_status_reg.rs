#[doc = "Register `REGS_INT_SS_C0_TH_THRESH_PULSE_STATUS_REG` reader"]
pub type R = crate::R<RegsIntSsC0ThThreshPulseStatusRegSpec>;
#[doc = "Register `REGS_INT_SS_C0_TH_THRESH_PULSE_STATUS_REG` writer"]
pub type W = crate::W<RegsIntSsC0ThThreshPulseStatusRegSpec>;
#[doc = "Field `CORE_0_THOST` reader - 7:0\\]
Core 0 THost Threshold Pulse Interrupt Status Register"]
pub type Core0ThostR = crate::FieldReader;
#[doc = "Field `CORE_0_THOST` writer - 7:0\\]
Core 0 THost Threshold Pulse Interrupt Status Register"]
pub type Core0ThostW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Core 0 THost Threshold Pulse Interrupt Status Register"]
    #[inline(always)]
    pub fn core_0_thost(&self) -> Core0ThostR {
        Core0ThostR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Core 0 THost Threshold Pulse Interrupt Status Register"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_thost(&mut self) -> Core0ThostW<RegsIntSsC0ThThreshPulseStatusRegSpec> {
        Core0ThostW::new(self, 0)
    }
}
#[doc = "THost Threshold Pulse Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_th_thresh_pulse_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_th_thresh_pulse_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsIntSsC0ThThreshPulseStatusRegSpec;
impl crate::RegisterSpec for RegsIntSsC0ThThreshPulseStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_int_ss_c0_th_thresh_pulse_status_reg::R`](R) reader structure"]
impl crate::Readable for RegsIntSsC0ThThreshPulseStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs_int_ss_c0_th_thresh_pulse_status_reg::W`](W) writer structure"]
impl crate::Writable for RegsIntSsC0ThThreshPulseStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_INT_SS_C0_TH_THRESH_PULSE_STATUS_REG to value 0"]
impl crate::Resettable for RegsIntSsC0ThThreshPulseStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
