#[doc = "Register `INTF_STATS_SUM_MAGDIFF_VAL` reader"]
pub type R = crate::R<IntfStatsSumMagdiffValSpec>;
#[doc = "Register `INTF_STATS_SUM_MAGDIFF_VAL` writer"]
pub type W = crate::W<IntfStatsSumMagdiffValSpec>;
#[doc = "Field `intf_stats_sum_magdiff_val` reader - 23:0\\]
Indicates the sum of magdiff values ; Only Configured BCNT magdiff values are added"]
pub type IntfStatsSumMagdiffValR = crate::FieldReader<u32>;
#[doc = "Field `intf_stats_sum_magdiff_val` writer - 23:0\\]
Indicates the sum of magdiff values ; Only Configured BCNT magdiff values are added"]
pub type IntfStatsSumMagdiffValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates the sum of magdiff values ; Only Configured BCNT magdiff values are added"]
    #[inline(always)]
    pub fn intf_stats_sum_magdiff_val(&self) -> IntfStatsSumMagdiffValR {
        IntfStatsSumMagdiffValR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates the sum of magdiff values ; Only Configured BCNT magdiff values are added"]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_sum_magdiff_val(
        &mut self,
    ) -> IntfStatsSumMagdiffValW<IntfStatsSumMagdiffValSpec> {
        IntfStatsSumMagdiffValW::new(self, 0)
    }
}
#[doc = "INTF_STATS_SUM_MAGDIFF_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_sum_magdiff_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_sum_magdiff_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsSumMagdiffValSpec;
impl crate::RegisterSpec for IntfStatsSumMagdiffValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_sum_magdiff_val::R`](R) reader structure"]
impl crate::Readable for IntfStatsSumMagdiffValSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_sum_magdiff_val::W`](W) writer structure"]
impl crate::Writable for IntfStatsSumMagdiffValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_SUM_MAGDIFF_VAL to value 0"]
impl crate::Resettable for IntfStatsSumMagdiffValSpec {
    const RESET_VALUE: u32 = 0;
}
