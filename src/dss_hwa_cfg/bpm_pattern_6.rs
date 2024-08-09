#[doc = "Register `BPM_PATTERN_6` reader"]
pub type R = crate::R<BpmPattern6Spec>;
#[doc = "Register `BPM_PATTERN_6` writer"]
pub type W = crate::W<BpmPattern6Spec>;
#[doc = "Field `bpm_pattern_6` reader - 31:0\\]
BPM pattern \\[223:192\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmPattern6R = crate::FieldReader<u32>;
#[doc = "Field `bpm_pattern_6` writer - 31:0\\]
BPM pattern \\[223:192\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmPattern6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern \\[223:192\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    pub fn bpm_pattern_6(&self) -> BpmPattern6R {
        BpmPattern6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern \\[223:192\\]: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bpm_pattern_6(&mut self) -> BpmPattern6W<BpmPattern6Spec> {
        BpmPattern6W::new(self, 0)
    }
}
#[doc = "BPM_PATTERN_6\n\nYou can [`read`](crate::Reg::read) this register and get [`bpm_pattern_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpm_pattern_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpmPattern6Spec;
impl crate::RegisterSpec for BpmPattern6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpm_pattern_6::R`](R) reader structure"]
impl crate::Readable for BpmPattern6Spec {}
#[doc = "`write(|w| ..)` method takes [`bpm_pattern_6::W`](W) writer structure"]
impl crate::Writable for BpmPattern6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPM_PATTERN_6 to value 0"]
impl crate::Resettable for BpmPattern6Spec {
    const RESET_VALUE: u32 = 0;
}
