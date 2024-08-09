#[doc = "Register `CLR_CLIP_MISC` reader"]
pub type R = crate::R<ClrClipMiscSpec>;
#[doc = "Register `CLR_CLIP_MISC` writer"]
pub type W = crate::W<ClrClipMiscSpec>;
#[doc = "Field `clr_clip_status` reader - 0:0\\]
This clears the following clip register channel_comb_clip_status dc_acc_clip_status dc_est_clip_status intf_stats_mag_accumulator_clip_status intf_stats_magdiff_accumulator_clip_status intf_stats_thresh_mag_clip_status intf_stats_thresh_magdiff_clip_status twid_incr_delta_frac_clip_status ip_formatter_clip_status op_formatter_clip_status intf_stats_sum_mag_val_clip_status intf_stats_sum_magdiff_val_clip_status dc_sub_clip Its a self clearing bit"]
pub type ClrClipStatusR = crate::BitReader;
#[doc = "Field `clr_clip_status` writer - 0:0\\]
This clears the following clip register channel_comb_clip_status dc_acc_clip_status dc_est_clip_status intf_stats_mag_accumulator_clip_status intf_stats_magdiff_accumulator_clip_status intf_stats_thresh_mag_clip_status intf_stats_thresh_magdiff_clip_status twid_incr_delta_frac_clip_status ip_formatter_clip_status op_formatter_clip_status intf_stats_sum_mag_val_clip_status intf_stats_sum_magdiff_val_clip_status dc_sub_clip Its a self clearing bit"]
pub type ClrClipStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This clears the following clip register channel_comb_clip_status dc_acc_clip_status dc_est_clip_status intf_stats_mag_accumulator_clip_status intf_stats_magdiff_accumulator_clip_status intf_stats_thresh_mag_clip_status intf_stats_thresh_magdiff_clip_status twid_incr_delta_frac_clip_status ip_formatter_clip_status op_formatter_clip_status intf_stats_sum_mag_val_clip_status intf_stats_sum_magdiff_val_clip_status dc_sub_clip Its a self clearing bit"]
    #[inline(always)]
    pub fn clr_clip_status(&self) -> ClrClipStatusR {
        ClrClipStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This clears the following clip register channel_comb_clip_status dc_acc_clip_status dc_est_clip_status intf_stats_mag_accumulator_clip_status intf_stats_magdiff_accumulator_clip_status intf_stats_thresh_mag_clip_status intf_stats_thresh_magdiff_clip_status twid_incr_delta_frac_clip_status ip_formatter_clip_status op_formatter_clip_status intf_stats_sum_mag_val_clip_status intf_stats_sum_magdiff_val_clip_status dc_sub_clip Its a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_clip_status(&mut self) -> ClrClipStatusW<ClrClipMiscSpec> {
        ClrClipStatusW::new(self, 0)
    }
}
#[doc = "CLR_CLIP_MISC\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_clip_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_clip_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrClipMiscSpec;
impl crate::RegisterSpec for ClrClipMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_clip_misc::R`](R) reader structure"]
impl crate::Readable for ClrClipMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_clip_misc::W`](W) writer structure"]
impl crate::Writable for ClrClipMiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR_CLIP_MISC to value 0"]
impl crate::Resettable for ClrClipMiscSpec {
    const RESET_VALUE: u32 = 0;
}
