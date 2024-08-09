#[doc = "Register `TWID_INCR_DELTA_FRAC_CLIP_STATUS` reader"]
pub type R = crate::R<TwidIncrDeltaFracClipStatusSpec>;
#[doc = "Register `TWID_INCR_DELTA_FRAC_CLIP_STATUS` writer"]
pub type W = crate::W<TwidIncrDeltaFracClipStatusSpec>;
#[doc = "Field `twid_incr_delta_frac_clip_status` reader - 0:0\\]
Indicates the clip status for TWID_INCR_DELTA_FRAC accumulator"]
pub type TwidIncrDeltaFracClipStatusR = crate::BitReader;
#[doc = "Field `twid_incr_delta_frac_clip_status` writer - 0:0\\]
Indicates the clip status for TWID_INCR_DELTA_FRAC accumulator"]
pub type TwidIncrDeltaFracClipStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the clip status for TWID_INCR_DELTA_FRAC accumulator"]
    #[inline(always)]
    pub fn twid_incr_delta_frac_clip_status(&self) -> TwidIncrDeltaFracClipStatusR {
        TwidIncrDeltaFracClipStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the clip status for TWID_INCR_DELTA_FRAC accumulator"]
    #[inline(always)]
    #[must_use]
    pub fn twid_incr_delta_frac_clip_status(
        &mut self,
    ) -> TwidIncrDeltaFracClipStatusW<TwidIncrDeltaFracClipStatusSpec> {
        TwidIncrDeltaFracClipStatusW::new(self, 0)
    }
}
#[doc = "TWID_INCR_DELTA_FRAC_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`twid_incr_delta_frac_clip_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twid_incr_delta_frac_clip_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwidIncrDeltaFracClipStatusSpec;
impl crate::RegisterSpec for TwidIncrDeltaFracClipStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twid_incr_delta_frac_clip_status::R`](R) reader structure"]
impl crate::Readable for TwidIncrDeltaFracClipStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`twid_incr_delta_frac_clip_status::W`](W) writer structure"]
impl crate::Writable for TwidIncrDeltaFracClipStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TWID_INCR_DELTA_FRAC_CLIP_STATUS to value 0"]
impl crate::Resettable for TwidIncrDeltaFracClipStatusSpec {
    const RESET_VALUE: u32 = 0;
}
