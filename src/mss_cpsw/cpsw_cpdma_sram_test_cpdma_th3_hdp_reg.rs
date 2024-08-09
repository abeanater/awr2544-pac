#[doc = "Register `CPSW_CPDMA_SRAM_TEST_CPDMA_TH3_HDP_REG` reader"]
pub type R = crate::R<CpswCpdmaSramTestCpdmaTh3HdpRegSpec>;
#[doc = "Register `CPSW_CPDMA_SRAM_TEST_CPDMA_TH3_HDP_REG` writer"]
pub type W = crate::W<CpswCpdmaSramTestCpdmaTh3HdpRegSpec>;
#[doc = "Field `TEST_CPDMA_THOST` reader - 31:0\\]
Test CPDMA THost Channel 3 Head Descriptor Pointer"]
pub type TestCpdmaThostR = crate::FieldReader<u32>;
#[doc = "Field `TEST_CPDMA_THOST` writer - 31:0\\]
Test CPDMA THost Channel 3 Head Descriptor Pointer"]
pub type TestCpdmaThostW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Test CPDMA THost Channel 3 Head Descriptor Pointer"]
    #[inline(always)]
    pub fn test_cpdma_thost(&self) -> TestCpdmaThostR {
        TestCpdmaThostR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Test CPDMA THost Channel 3 Head Descriptor Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn test_cpdma_thost(&mut self) -> TestCpdmaThostW<CpswCpdmaSramTestCpdmaTh3HdpRegSpec> {
        TestCpdmaThostW::new(self, 0)
    }
}
#[doc = "Test CPDMA THost Channel 3 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_th3_hdp_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_th3_hdp_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaSramTestCpdmaTh3HdpRegSpec;
impl crate::RegisterSpec for CpswCpdmaSramTestCpdmaTh3HdpRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_sram_test_cpdma_th3_hdp_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaSramTestCpdmaTh3HdpRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_sram_test_cpdma_th3_hdp_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaSramTestCpdmaTh3HdpRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_SRAM_TEST_CPDMA_TH3_HDP_REG to value 0"]
impl crate::Resettable for CpswCpdmaSramTestCpdmaTh3HdpRegSpec {
    const RESET_VALUE: u32 = 0;
}
