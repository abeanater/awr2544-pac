#[doc = "Register `ALE_POLICECFG7` reader"]
pub type R = crate::R<AlePolicecfg7Spec>;
#[doc = "Register `ALE_POLICECFG7` writer"]
pub type W = crate::W<AlePolicecfg7Spec>;
#[doc = "Field `COMMITTED_INFORMATION_IDLE` reader - 31:0\\]
Committed Information Idle Increment Value - The number added to the CIR counter every clock cycle. If zero the CIR counter is disabled and packets will never be marked or processed as YELLOW."]
pub type CommittedInformationIdleR = crate::FieldReader<u32>;
#[doc = "Field `COMMITTED_INFORMATION_IDLE` writer - 31:0\\]
Committed Information Idle Increment Value - The number added to the CIR counter every clock cycle. If zero the CIR counter is disabled and packets will never be marked or processed as YELLOW."]
pub type CommittedInformationIdleW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Committed Information Idle Increment Value - The number added to the CIR counter every clock cycle. If zero the CIR counter is disabled and packets will never be marked or processed as YELLOW."]
    #[inline(always)]
    pub fn committed_information_idle(&self) -> CommittedInformationIdleR {
        CommittedInformationIdleR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Committed Information Idle Increment Value - The number added to the CIR counter every clock cycle. If zero the CIR counter is disabled and packets will never be marked or processed as YELLOW."]
    #[inline(always)]
    #[must_use]
    pub fn committed_information_idle(&mut self) -> CommittedInformationIdleW<AlePolicecfg7Spec> {
        CommittedInformationIdleW::new(self, 0)
    }
}
#[doc = "The CIR counter is a 37 bit internal counter where ~icir_idle_inc_val is added every clock and the frame size &amp;lt;&amp;lt; 18 is subtracted at EOF if not RED or YELLOW at LUT time. If the counter is positive the packet will be marked GREEN, else it can be YELLOW or RED based on the PIR counter. If only this counter is used (aka pir_idle_inc_val==0) Packet are marked YELLOW or GREEN based on CIR counter only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicecfg7Spec;
impl crate::RegisterSpec for AlePolicecfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policecfg7::R`](R) reader structure"]
impl crate::Readable for AlePolicecfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_policecfg7::W`](W) writer structure"]
impl crate::Writable for AlePolicecfg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICECFG7 to value 0"]
impl crate::Resettable for AlePolicecfg7Spec {
    const RESET_VALUE: u32 = 0;
}
