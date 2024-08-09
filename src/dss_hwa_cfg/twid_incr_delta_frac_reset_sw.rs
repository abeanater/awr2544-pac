#[doc = "Register `TWID_INCR_DELTA_FRAC_RESET_SW` reader"]
pub type R = crate::R<TwidIncrDeltaFracResetSwSpec>;
#[doc = "Register `TWID_INCR_DELTA_FRAC_RESET_SW` writer"]
pub type W = crate::W<TwidIncrDeltaFracResetSwSpec>;
#[doc = "Field `twid_incr_delta_frac_reset_sw` reader - 0:0\\]
This resets the param set counter used in Complex multiplier mode 10 . It s a Self clearing bit"]
pub type TwidIncrDeltaFracResetSwR = crate::BitReader;
#[doc = "Field `twid_incr_delta_frac_reset_sw` writer - 0:0\\]
This resets the param set counter used in Complex multiplier mode 10 . It s a Self clearing bit"]
pub type TwidIncrDeltaFracResetSwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This resets the param set counter used in Complex multiplier mode 10 . It s a Self clearing bit"]
    #[inline(always)]
    pub fn twid_incr_delta_frac_reset_sw(&self) -> TwidIncrDeltaFracResetSwR {
        TwidIncrDeltaFracResetSwR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This resets the param set counter used in Complex multiplier mode 10 . It s a Self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn twid_incr_delta_frac_reset_sw(
        &mut self,
    ) -> TwidIncrDeltaFracResetSwW<TwidIncrDeltaFracResetSwSpec> {
        TwidIncrDeltaFracResetSwW::new(self, 0)
    }
}
#[doc = "TWID_INCR_DELTA_FRAC_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`twid_incr_delta_frac_reset_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twid_incr_delta_frac_reset_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwidIncrDeltaFracResetSwSpec;
impl crate::RegisterSpec for TwidIncrDeltaFracResetSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twid_incr_delta_frac_reset_sw::R`](R) reader structure"]
impl crate::Readable for TwidIncrDeltaFracResetSwSpec {}
#[doc = "`write(|w| ..)` method takes [`twid_incr_delta_frac_reset_sw::W`](W) writer structure"]
impl crate::Writable for TwidIncrDeltaFracResetSwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TWID_INCR_DELTA_FRAC_RESET_SW to value 0"]
impl crate::Resettable for TwidIncrDeltaFracResetSwSpec {
    const RESET_VALUE: u32 = 0;
}
