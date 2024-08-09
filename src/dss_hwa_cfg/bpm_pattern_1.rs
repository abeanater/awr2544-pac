#[doc = "Register `BPM_PATTERN_1` reader"]
pub type R = crate::R<BpmPattern1Spec>;
#[doc = "Register `BPM_PATTERN_1` writer"]
pub type W = crate::W<BpmPattern1Spec>;
#[doc = "Field `bpm_pattern_1` reader - 31:0\\]
BPM pattern \\[63:32\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmPattern1R = crate::FieldReader<u32>;
#[doc = "Field `bpm_pattern_1` writer - 31:0\\]
BPM pattern \\[63:32\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmPattern1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern \\[63:32\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    pub fn bpm_pattern_1(&self) -> BpmPattern1R {
        BpmPattern1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern \\[63:32\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bpm_pattern_1(&mut self) -> BpmPattern1W<BpmPattern1Spec> {
        BpmPattern1W::new(self, 0)
    }
}
#[doc = "BPM_PATTERN_1\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpmPattern1Spec;
impl crate::RegisterSpec for BpmPattern1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpm_pattern_1::R`](R) reader structure"]
impl crate::Readable for BpmPattern1Spec {}
#[doc = "`write(|w| ..)` method takes [`bpm_pattern_1::W`](W) writer structure"]
impl crate::Writable for BpmPattern1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPM_PATTERN_1 to value 0"]
impl crate::Resettable for BpmPattern1Spec {
    const RESET_VALUE: u32 = 0;
}
