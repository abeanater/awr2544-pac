#[doc = "Register `CPSW_CPDMA_INT_CPDMA_IN_VECTOR_REG` reader"]
pub type R = crate::R<CpswCpdmaIntCpdmaInVectorRegSpec>;
#[doc = "Register `CPSW_CPDMA_INT_CPDMA_IN_VECTOR_REG` writer"]
pub type W = crate::W<CpswCpdmaIntCpdmaInVectorRegSpec>;
#[doc = "Field `CPDMA_DMA_IN` reader - 31:0\\]
CPDMA DMA IN Vector"]
pub type CpdmaDmaInR = crate::FieldReader<u32>;
#[doc = "Field `CPDMA_DMA_IN` writer - 31:0\\]
CPDMA DMA IN Vector"]
pub type CpdmaDmaInW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CPDMA DMA IN Vector"]
    #[inline(always)]
    pub fn cpdma_dma_in(&self) -> CpdmaDmaInR {
        CpdmaDmaInR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CPDMA DMA IN Vector"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_dma_in(&mut self) -> CpdmaDmaInW<CpswCpdmaIntCpdmaInVectorRegSpec> {
        CpdmaDmaInW::new(self, 0)
    }
}
#[doc = "CPDMA DMA IN Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_in_vector_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_in_vector_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaIntCpdmaInVectorRegSpec;
impl crate::RegisterSpec for CpswCpdmaIntCpdmaInVectorRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_int_cpdma_in_vector_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaIntCpdmaInVectorRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_int_cpdma_in_vector_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaIntCpdmaInVectorRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_INT_CPDMA_IN_VECTOR_REG to value 0"]
impl crate::Resettable for CpswCpdmaIntCpdmaInVectorRegSpec {
    const RESET_VALUE: u32 = 0;
}
