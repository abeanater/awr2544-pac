#[doc = "Register `CPSW_CPDMA_SRAM_TEST_CPDMA_FH5_CP_REG` reader"]
pub type R = crate::R<CpswCpdmaSramTestCpdmaFh5CpRegSpec>;
#[doc = "Register `CPSW_CPDMA_SRAM_TEST_CPDMA_FH5_CP_REG` writer"]
pub type W = crate::W<CpswCpdmaSramTestCpdmaFh5CpRegSpec>;
#[doc = "Field `TEST_CPDMA_FHOST` reader - 31:0\\]
Test CPDMA FHost Channel 5 Completion Pointer"]
pub type TestCpdmaFhostR = crate::FieldReader<u32>;
#[doc = "Field `TEST_CPDMA_FHOST` writer - 31:0\\]
Test CPDMA FHost Channel 5 Completion Pointer"]
pub type TestCpdmaFhostW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Test CPDMA FHost Channel 5 Completion Pointer"]
    #[inline(always)]
    pub fn test_cpdma_fhost(&self) -> TestCpdmaFhostR {
        TestCpdmaFhostR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Test CPDMA FHost Channel 5 Completion Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn test_cpdma_fhost(&mut self) -> TestCpdmaFhostW<CpswCpdmaSramTestCpdmaFh5CpRegSpec> {
        TestCpdmaFhostW::new(self, 0)
    }
}
#[doc = "Test CPDMA FHost Channel 5 Completion Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_test_cpdma_fh5_cp_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_test_cpdma_fh5_cp_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaSramTestCpdmaFh5CpRegSpec;
impl crate::RegisterSpec for CpswCpdmaSramTestCpdmaFh5CpRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_sram_test_cpdma_fh5_cp_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaSramTestCpdmaFh5CpRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_sram_test_cpdma_fh5_cp_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaSramTestCpdmaFh5CpRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_SRAM_TEST_CPDMA_FH5_CP_REG to value 0"]
impl crate::Resettable for CpswCpdmaSramTestCpdmaFh5CpRegSpec {
    const RESET_VALUE: u32 = 0;
}
