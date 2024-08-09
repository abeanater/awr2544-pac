#[doc = "Register `ALE_POLICECFG6` reader"]
pub type R = crate::R<AlePolicecfg6Spec>;
#[doc = "Register `ALE_POLICECFG6` writer"]
pub type W = crate::W<AlePolicecfg6Spec>;
#[doc = "Field `PEAK_INFORMATION_RATE` reader - 31:0\\]
Peak Information Rate Idle Increment Value - The number added to the PIR counter every clock cycle. If zero the PIR counter is disabled and packets will never be marked or processed as RED."]
pub type PeakInformationRateR = crate::FieldReader<u32>;
#[doc = "Field `PEAK_INFORMATION_RATE` writer - 31:0\\]
Peak Information Rate Idle Increment Value - The number added to the PIR counter every clock cycle. If zero the PIR counter is disabled and packets will never be marked or processed as RED."]
pub type PeakInformationRateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peak Information Rate Idle Increment Value - The number added to the PIR counter every clock cycle. If zero the PIR counter is disabled and packets will never be marked or processed as RED."]
    #[inline(always)]
    pub fn peak_information_rate(&self) -> PeakInformationRateR {
        PeakInformationRateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peak Information Rate Idle Increment Value - The number added to the PIR counter every clock cycle. If zero the PIR counter is disabled and packets will never be marked or processed as RED."]
    #[inline(always)]
    #[must_use]
    pub fn peak_information_rate(&mut self) -> PeakInformationRateW<AlePolicecfg6Spec> {
        PeakInformationRateW::new(self, 0)
    }
}
#[doc = "The PIR counter is a 37 bit internal counter where ~ipir_idle_inc_val is added every clock and the frame size &amp;lt;&amp;lt; 18 is subtracted at EOF if not RED at LUT time. If the counter is negative the packet will be marked RED, else it can be YELLOW or GREEN based on the CIR counter. If only this counter is used (aka cir_idle_inc_val==0) Packet are marked RED or GREEN based on PIR counter only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicecfg6Spec;
impl crate::RegisterSpec for AlePolicecfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policecfg6::R`](R) reader structure"]
impl crate::Readable for AlePolicecfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_policecfg6::W`](W) writer structure"]
impl crate::Writable for AlePolicecfg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICECFG6 to value 0"]
impl crate::Resettable for AlePolicecfg6Spec {
    const RESET_VALUE: u32 = 0;
}
