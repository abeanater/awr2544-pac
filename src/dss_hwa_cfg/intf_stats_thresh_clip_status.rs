#[doc = "Register `INTF_STATS_THRESH_CLIP_STATUS` reader"]
pub type R = crate::R<IntfStatsThreshClipStatusSpec>;
#[doc = "Register `INTF_STATS_THRESH_CLIP_STATUS` writer"]
pub type W = crate::W<IntfStatsThreshClipStatusSpec>;
#[doc = "Field `intf_stats_thresh_mag_clip_status` reader - 11:0\\]
Interference magnitude threshold Clip status"]
pub type IntfStatsThreshMagClipStatusR = crate::FieldReader<u16>;
#[doc = "Field `intf_stats_thresh_mag_clip_status` writer - 11:0\\]
Interference magnitude threshold Clip status"]
pub type IntfStatsThreshMagClipStatusW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `intf_stats_thresh_magdiff_clip_status` reader - 27:16\\]
Interference magnitude difference threshold Clip status"]
pub type IntfStatsThreshMagdiffClipStatusR = crate::FieldReader<u16>;
#[doc = "Field `intf_stats_thresh_magdiff_clip_status` writer - 27:16\\]
Interference magnitude difference threshold Clip status"]
pub type IntfStatsThreshMagdiffClipStatusW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Interference magnitude threshold Clip status"]
    #[inline(always)]
    pub fn intf_stats_thresh_mag_clip_status(&self) -> IntfStatsThreshMagClipStatusR {
        IntfStatsThreshMagClipStatusR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Interference magnitude difference threshold Clip status"]
    #[inline(always)]
    pub fn intf_stats_thresh_magdiff_clip_status(&self) -> IntfStatsThreshMagdiffClipStatusR {
        IntfStatsThreshMagdiffClipStatusR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Interference magnitude threshold Clip status"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_thresh_mag_clip_status(
        &mut self,
    ) -> IntfStatsThreshMagClipStatusW<IntfStatsThreshClipStatusSpec> {
        IntfStatsThreshMagClipStatusW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Interference magnitude difference threshold Clip status"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_thresh_magdiff_clip_status(
        &mut self,
    ) -> IntfStatsThreshMagdiffClipStatusW<IntfStatsThreshClipStatusSpec> {
        IntfStatsThreshMagdiffClipStatusW::new(self, 16)
    }
}
#[doc = "INTF_STATS_THRESH_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_thresh_clip_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_thresh_clip_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsThreshClipStatusSpec;
impl crate::RegisterSpec for IntfStatsThreshClipStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_thresh_clip_status::R`](R) reader structure"]
impl crate::Readable for IntfStatsThreshClipStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_thresh_clip_status::W`](W) writer structure"]
impl crate::Writable for IntfStatsThreshClipStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_THRESH_CLIP_STATUS to value 0"]
impl crate::Resettable for IntfStatsThreshClipStatusSpec {
    const RESET_VALUE: u32 = 0;
}
