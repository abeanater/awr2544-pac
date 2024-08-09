#[doc = "Register `CPSW_CPDMA_INT_CPDMA_EOI_VECTOR_REG` reader"]
pub type R = crate::R<CpswCpdmaIntCpdmaEoiVectorRegSpec>;
#[doc = "Register `CPSW_CPDMA_INT_CPDMA_EOI_VECTOR_REG` writer"]
pub type W = crate::W<CpswCpdmaIntCpdmaEoiVectorRegSpec>;
#[doc = "Field `CPDMA_DMA_EOI` reader - 4:0\\]
CPDMA DMA EOI Vector"]
pub type CpdmaDmaEoiR = crate::FieldReader;
#[doc = "Field `CPDMA_DMA_EOI` writer - 4:0\\]
CPDMA DMA EOI Vector"]
pub type CpdmaDmaEoiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
CPDMA DMA EOI Vector"]
    #[inline(always)]
    pub fn cpdma_dma_eoi(&self) -> CpdmaDmaEoiR {
        CpdmaDmaEoiR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
CPDMA DMA EOI Vector"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_dma_eoi(&mut self) -> CpdmaDmaEoiW<CpswCpdmaIntCpdmaEoiVectorRegSpec> {
        CpdmaDmaEoiW::new(self, 0)
    }
}
#[doc = "CPDMA DMA EOI Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_eoi_vector_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_eoi_vector_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaIntCpdmaEoiVectorRegSpec;
impl crate::RegisterSpec for CpswCpdmaIntCpdmaEoiVectorRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_int_cpdma_eoi_vector_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaIntCpdmaEoiVectorRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_int_cpdma_eoi_vector_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaIntCpdmaEoiVectorRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_INT_CPDMA_EOI_VECTOR_REG to value 0"]
impl crate::Resettable for CpswCpdmaIntCpdmaEoiVectorRegSpec {
    const RESET_VALUE: u32 = 0;
}
