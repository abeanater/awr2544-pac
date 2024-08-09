#[doc = "Register `BPM_PATTERN_0` reader"]
pub type R = crate::R<BpmPattern0Spec>;
#[doc = "Register `BPM_PATTERN_0` writer"]
pub type W = crate::W<BpmPattern0Spec>;
#[doc = "Field `bpm_pattern_0` reader - 31:0\\]
BPM pattern \\[31:0\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmPattern0R = crate::FieldReader<u32>;
#[doc = "Field `bpm_pattern_0` writer - 31:0\\]
BPM pattern \\[31:0\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmPattern0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern \\[31:0\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    pub fn bpm_pattern_0(&self) -> BpmPattern0R {
        BpmPattern0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern \\[31:0\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bpm_pattern_0(&mut self) -> BpmPattern0W<BpmPattern0Spec> {
        BpmPattern0W::new(self, 0)
    }
}
#[doc = "BPM_PATTERN_0\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpmPattern0Spec;
impl crate::RegisterSpec for BpmPattern0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpm_pattern_0::R`](R) reader structure"]
impl crate::Readable for BpmPattern0Spec {}
#[doc = "`write(|w| ..)` method takes [`bpm_pattern_0::W`](W) writer structure"]
impl crate::Writable for BpmPattern0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPM_PATTERN_0 to value 0"]
impl crate::Resettable for BpmPattern0Spec {
    const RESET_VALUE: u32 = 0;
}
