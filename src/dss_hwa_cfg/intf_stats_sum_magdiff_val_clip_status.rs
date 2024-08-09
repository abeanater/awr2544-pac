#[doc = "Register `INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS` reader"]
pub type R = crate::R<IntfStatsSumMagdiffValClipStatusSpec>;
#[doc = "Register `INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS` writer"]
pub type W = crate::W<IntfStatsSumMagdiffValClipStatusSpec>;
#[doc = "Field `intf_stats_sum_magdiff_val_clip_status` reader - 0:0\\]
indicates the clip status of sum of magdiff values"]
pub type IntfStatsSumMagdiffValClipStatusR = crate::BitReader;
#[doc = "Field `intf_stats_sum_magdiff_val_clip_status` writer - 0:0\\]
indicates the clip status of sum of magdiff values"]
pub type IntfStatsSumMagdiffValClipStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
indicates the clip status of sum of magdiff values"]
    #[inline(always)]
    pub fn intf_stats_sum_magdiff_val_clip_status(&self) -> IntfStatsSumMagdiffValClipStatusR {
        IntfStatsSumMagdiffValClipStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
indicates the clip status of sum of magdiff values"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_sum_magdiff_val_clip_status(
        &mut self,
    ) -> IntfStatsSumMagdiffValClipStatusW<IntfStatsSumMagdiffValClipStatusSpec> {
        IntfStatsSumMagdiffValClipStatusW::new(self, 0)
    }
}
#[doc = "INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_sum_magdiff_val_clip_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_sum_magdiff_val_clip_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsSumMagdiffValClipStatusSpec;
impl crate::RegisterSpec for IntfStatsSumMagdiffValClipStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_sum_magdiff_val_clip_status::R`](R) reader structure"]
impl crate::Readable for IntfStatsSumMagdiffValClipStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_sum_magdiff_val_clip_status::W`](W) writer structure"]
impl crate::Writable for IntfStatsSumMagdiffValClipStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_SUM_MAGDIFF_VAL_CLIP_STATUS to value 0"]
impl crate::Resettable for IntfStatsSumMagdiffValClipStatusSpec {
    const RESET_VALUE: u32 = 0;
}
