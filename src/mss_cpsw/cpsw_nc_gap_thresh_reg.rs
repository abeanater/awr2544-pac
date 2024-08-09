#[doc = "Register `CPSW_NC_GAP_THRESH_REG` reader"]
pub type R = crate::R<CpswNcGapThreshRegSpec>;
#[doc = "Register `CPSW_NC_GAP_THRESH_REG` writer"]
pub type W = crate::W<CpswNcGapThreshRegSpec>;
#[doc = "Field `SHORT_GAP_THRESHOLD` reader - 4:0\\]
Short Gap Threshold"]
pub type ShortGapThresholdR = crate::FieldReader;
#[doc = "Field `SHORT_GAP_THRESHOLD` writer - 4:0\\]
Short Gap Threshold"]
pub type ShortGapThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Short Gap Threshold"]
    #[inline(always)]
    pub fn short_gap_threshold(&self) -> ShortGapThresholdR {
        ShortGapThresholdR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Short Gap Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn short_gap_threshold(&mut self) -> ShortGapThresholdW<CpswNcGapThreshRegSpec> {
        ShortGapThresholdW::new(self, 0)
    }
}
#[doc = "CPSW Transmit FIFO Short Gap Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_gap_thresh_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_gap_thresh_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcGapThreshRegSpec;
impl crate::RegisterSpec for CpswNcGapThreshRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_gap_thresh_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcGapThreshRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_gap_thresh_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcGapThreshRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_GAP_THRESH_REG to value 0"]
impl crate::Resettable for CpswNcGapThreshRegSpec {
    const RESET_VALUE: u32 = 0;
}
