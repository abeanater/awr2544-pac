#[doc = "Register `CPSW_CPDMA_INT_CPDMA_TH2_PENDTHRESH_REG` reader"]
pub type R = crate::R<CpswCpdmaIntCpdmaTh2PendthreshRegSpec>;
#[doc = "Register `CPSW_CPDMA_INT_CPDMA_TH2_PENDTHRESH_REG` writer"]
pub type W = crate::W<CpswCpdmaIntCpdmaTh2PendthreshRegSpec>;
#[doc = "Field `CPDMA_THOST_THRESHOLD` reader - 7:0\\]
CPDMA THost Threshold Pending Register"]
pub type CpdmaThostThresholdR = crate::FieldReader;
#[doc = "Field `CPDMA_THOST_THRESHOLD` writer - 7:0\\]
CPDMA THost Threshold Pending Register"]
pub type CpdmaThostThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    pub fn cpdma_thost_threshold(&self) -> CpdmaThostThresholdR {
        CpdmaThostThresholdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
CPDMA THost Threshold Pending Register"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_threshold(
        &mut self,
    ) -> CpdmaThostThresholdW<CpswCpdmaIntCpdmaTh2PendthreshRegSpec> {
        CpdmaThostThresholdW::new(self, 0)
    }
}
#[doc = "CPDMA THost Threshold Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th2_pendthresh_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th2_pendthresh_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaIntCpdmaTh2PendthreshRegSpec;
impl crate::RegisterSpec for CpswCpdmaIntCpdmaTh2PendthreshRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_int_cpdma_th2_pendthresh_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaIntCpdmaTh2PendthreshRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_int_cpdma_th2_pendthresh_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaIntCpdmaTh2PendthreshRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_INT_CPDMA_TH2_PENDTHRESH_REG to value 0"]
impl crate::Resettable for CpswCpdmaIntCpdmaTh2PendthreshRegSpec {
    const RESET_VALUE: u32 = 0;
}
