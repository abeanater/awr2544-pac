#[doc = "Register `INTF_STATS_ACC_CLIP_STATUS` reader"]
pub type R = crate::R<IntfStatsAccClipStatusSpec>;
#[doc = "Register `INTF_STATS_ACC_CLIP_STATUS` writer"]
pub type W = crate::W<IntfStatsAccClipStatusSpec>;
#[doc = "Field `intf_stats_mag_accumulator_clip_status` reader - 11:0\\]
Interference magnitue accumulator Clip status"]
pub type IntfStatsMagAccumulatorClipStatusR = crate::FieldReader<u16>;
#[doc = "Field `intf_stats_mag_accumulator_clip_status` writer - 11:0\\]
Interference magnitue accumulator Clip status"]
pub type IntfStatsMagAccumulatorClipStatusW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `intf_stats_magdiff_accumulator_clip_status` reader - 27:16\\]
Interference magnitue difference accumulator Clip status"]
pub type IntfStatsMagdiffAccumulatorClipStatusR = crate::FieldReader<u16>;
#[doc = "Field `intf_stats_magdiff_accumulator_clip_status` writer - 27:16\\]
Interference magnitue difference accumulator Clip status"]
pub type IntfStatsMagdiffAccumulatorClipStatusW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Interference magnitue accumulator Clip status"]
    #[inline(always)]
    pub fn intf_stats_mag_accumulator_clip_status(&self) -> IntfStatsMagAccumulatorClipStatusR {
        IntfStatsMagAccumulatorClipStatusR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Interference magnitue difference accumulator Clip status"]
    #[inline(always)]
    pub fn intf_stats_magdiff_accumulator_clip_status(
        &self,
    ) -> IntfStatsMagdiffAccumulatorClipStatusR {
        IntfStatsMagdiffAccumulatorClipStatusR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Interference magnitue accumulator Clip status"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_mag_accumulator_clip_status(
        &mut self,
    ) -> IntfStatsMagAccumulatorClipStatusW<IntfStatsAccClipStatusSpec> {
        IntfStatsMagAccumulatorClipStatusW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Interference magnitue difference accumulator Clip status"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_magdiff_accumulator_clip_status(
        &mut self,
    ) -> IntfStatsMagdiffAccumulatorClipStatusW<IntfStatsAccClipStatusSpec> {
        IntfStatsMagdiffAccumulatorClipStatusW::new(self, 16)
    }
}
#[doc = "INTF_STATS_ACC_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_acc_clip_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_acc_clip_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsAccClipStatusSpec;
impl crate::RegisterSpec for IntfStatsAccClipStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_acc_clip_status::R`](R) reader structure"]
impl crate::Readable for IntfStatsAccClipStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_acc_clip_status::W`](W) writer structure"]
impl crate::Writable for IntfStatsAccClipStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_ACC_CLIP_STATUS to value 0"]
impl crate::Resettable for IntfStatsAccClipStatusSpec {
    const RESET_VALUE: u32 = 0;
}
