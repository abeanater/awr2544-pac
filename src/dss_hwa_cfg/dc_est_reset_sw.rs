#[doc = "Register `DC_EST_RESET_SW` reader"]
pub type R = crate::R<DcEstResetSwSpec>;
#[doc = "Register `DC_EST_RESET_SW` writer"]
pub type W = crate::W<DcEstResetSwSpec>;
#[doc = "Field `dc_est_reset_sw` reader - 0:0\\]
Reset for all DC estimation accumulators It s a Self clearing bit"]
pub type DcEstResetSwR = crate::BitReader;
#[doc = "Field `dc_est_reset_sw` writer - 0:0\\]
Reset for all DC estimation accumulators It s a Self clearing bit"]
pub type DcEstResetSwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reset for all DC estimation accumulators It s a Self clearing bit"]
    #[inline(always)]
    pub fn dc_est_reset_sw(&self) -> DcEstResetSwR {
        DcEstResetSwR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reset for all DC estimation accumulators It s a Self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_reset_sw(&mut self) -> DcEstResetSwW<DcEstResetSwSpec> {
        DcEstResetSwW::new(self, 0)
    }
}
#[doc = "DC_EST_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_reset_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_reset_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcEstResetSwSpec;
impl crate::RegisterSpec for DcEstResetSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_est_reset_sw::R`](R) reader structure"]
impl crate::Readable for DcEstResetSwSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_est_reset_sw::W`](W) writer structure"]
impl crate::Writable for DcEstResetSwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_EST_RESET_SW to value 0"]
impl crate::Resettable for DcEstResetSwSpec {
    const RESET_VALUE: u32 = 0;
}
