#[doc = "Register `ALE_ALE_PRESCALE` reader"]
pub type R = crate::R<AleAlePrescaleSpec>;
#[doc = "Register `ALE_ALE_PRESCALE` writer"]
pub type W = crate::W<AleAlePrescaleSpec>;
#[doc = "Field `ALE_PRESCALE_THE` reader - 19:0\\]
ALE Prescale - The input clock is divided by this value for use in the multicast/broadcast rate limiters. The minimum operating value is 0x10. The prescaler is off when the value is zero."]
pub type AlePrescaleTheR = crate::FieldReader<u32>;
#[doc = "Field `ALE_PRESCALE_THE` writer - 19:0\\]
ALE Prescale - The input clock is divided by this value for use in the multicast/broadcast rate limiters. The minimum operating value is 0x10. The prescaler is off when the value is zero."]
pub type AlePrescaleTheW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
ALE Prescale - The input clock is divided by this value for use in the multicast/broadcast rate limiters. The minimum operating value is 0x10. The prescaler is off when the value is zero."]
    #[inline(always)]
    pub fn ale_prescale_the(&self) -> AlePrescaleTheR {
        AlePrescaleTheR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
ALE Prescale - The input clock is divided by this value for use in the multicast/broadcast rate limiters. The minimum operating value is 0x10. The prescaler is off when the value is zero."]
    #[inline(always)]
    #[must_use]
    pub fn ale_prescale_the(&mut self) -> AlePrescaleTheW<AleAlePrescaleSpec> {
        AlePrescaleTheW::new(self, 0)
    }
}
#[doc = "The ALE Prescale Register is used to set the Broadcast and Multicast rate limiting prescaler value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_prescale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_prescale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAlePrescaleSpec;
impl crate::RegisterSpec for AleAlePrescaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_prescale::R`](R) reader structure"]
impl crate::Readable for AleAlePrescaleSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_prescale::W`](W) writer structure"]
impl crate::Writable for AleAlePrescaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_PRESCALE to value 0"]
impl crate::Resettable for AleAlePrescaleSpec {
    const RESET_VALUE: u32 = 0;
}
