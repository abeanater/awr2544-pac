#[doc = "Register `CSI2_STOPCLK_TIMING` reader"]
pub type R = crate::R<Csi2StopclkTimingSpec>;
#[doc = "Register `CSI2_STOPCLK_TIMING` writer"]
pub type W = crate::W<Csi2StopclkTimingSpec>;
#[doc = "Field `CSI2_STOPCLK_LATENCY` reader - 7:0\\]
Clock gating latency from CSI2 Protocol to TxByteClkHS"]
pub type Csi2StopclkLatencyR = crate::FieldReader;
#[doc = "Field `CSI2_STOPCLK_LATENCY` writer - 7:0\\]
Clock gating latency from CSI2 Protocol to TxByteClkHS"]
pub type Csi2StopclkLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clock gating latency from CSI2 Protocol to TxByteClkHS"]
    #[inline(always)]
    pub fn csi2_stopclk_latency(&self) -> Csi2StopclkLatencyR {
        Csi2StopclkLatencyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Clock gating latency from CSI2 Protocol to TxByteClkHS"]
    #[inline(always)]
    #[must_use]
    pub fn csi2_stopclk_latency(&mut self) -> Csi2StopclkLatencyW<Csi2StopclkTimingSpec> {
        Csi2StopclkLatencyW::new(self, 0)
    }
}
#[doc = "Number of functional clock cycles to wait for TxByteClkHS to stop/start after change in CSI2StopClk signal\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_stopclk_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_stopclk_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2StopclkTimingSpec;
impl crate::RegisterSpec for Csi2StopclkTimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_stopclk_timing::R`](R) reader structure"]
impl crate::Readable for Csi2StopclkTimingSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_stopclk_timing::W`](W) writer structure"]
impl crate::Writable for Csi2StopclkTimingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_STOPCLK_TIMING to value 0"]
impl crate::Resettable for Csi2StopclkTimingSpec {
    const RESET_VALUE: u32 = 0;
}
