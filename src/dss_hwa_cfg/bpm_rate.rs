#[doc = "Register `BPM_RATE` reader"]
pub type R = crate::R<BpmRateSpec>;
#[doc = "Register `BPM_RATE` writer"]
pub type W = crate::W<BpmRateSpec>;
#[doc = "Field `bpm_rate` reader - 9:0\\]
BPM rate: Specifies the number of input samples corresponding to each BPM bit. Minimum valid value for this register is 1."]
pub type BpmRateR = crate::FieldReader<u16>;
#[doc = "Field `bpm_rate` writer - 9:0\\]
BPM rate: Specifies the number of input samples corresponding to each BPM bit. Minimum valid value for this register is 1."]
pub type BpmRateW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
BPM rate: Specifies the number of input samples corresponding to each BPM bit. Minimum valid value for this register is 1."]
    #[inline(always)]
    pub fn bpm_rate(&self) -> BpmRateR {
        BpmRateR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
BPM rate: Specifies the number of input samples corresponding to each BPM bit. Minimum valid value for this register is 1."]
    #[inline(always)]
    #[must_use]
    pub fn bpm_rate(&mut self) -> BpmRateW<BpmRateSpec> {
        BpmRateW::new(self, 0)
    }
}
#[doc = "BPM_RATE\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_rate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_rate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpmRateSpec;
impl crate::RegisterSpec for BpmRateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpm_rate::R`](R) reader structure"]
impl crate::Readable for BpmRateSpec {}
#[doc = "`write(|w| ..)` method takes [`bpm_rate::W`](W) writer structure"]
impl crate::Writable for BpmRateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPM_RATE to value 0"]
impl crate::Resettable for BpmRateSpec {
    const RESET_VALUE: u32 = 0;
}
