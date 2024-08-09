#[doc = "Register `INTF_STATS_RESET_SW` reader"]
pub type R = crate::R<IntfStatsResetSwSpec>;
#[doc = "Register `INTF_STATS_RESET_SW` writer"]
pub type W = crate::W<IntfStatsResetSwSpec>;
#[doc = "Field `intf_stats_reset_sw` reader - 0:0\\]
SW reset for Interference stats module. It s a self clearing bit."]
pub type IntfStatsResetSwR = crate::BitReader;
#[doc = "Field `intf_stats_reset_sw` writer - 0:0\\]
SW reset for Interference stats module. It s a self clearing bit."]
pub type IntfStatsResetSwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SW reset for Interference stats module. It s a self clearing bit."]
    #[inline(always)]
    pub fn intf_stats_reset_sw(&self) -> IntfStatsResetSwR {
        IntfStatsResetSwR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SW reset for Interference stats module. It s a self clearing bit."]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_reset_sw(&mut self) -> IntfStatsResetSwW<IntfStatsResetSwSpec> {
        IntfStatsResetSwW::new(self, 0)
    }
}
#[doc = "INTF_STATS_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_reset_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_reset_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsResetSwSpec;
impl crate::RegisterSpec for IntfStatsResetSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_reset_sw::R`](R) reader structure"]
impl crate::Readable for IntfStatsResetSwSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_reset_sw::W`](W) writer structure"]
impl crate::Writable for IntfStatsResetSwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_RESET_SW to value 0"]
impl crate::Resettable for IntfStatsResetSwSpec {
    const RESET_VALUE: u32 = 0;
}
