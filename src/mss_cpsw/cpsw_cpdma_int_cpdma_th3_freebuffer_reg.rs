#[doc = "Register `CPSW_CPDMA_INT_CPDMA_TH3_FREEBUFFER_REG` reader"]
pub type R = crate::R<CpswCpdmaIntCpdmaTh3FreebufferRegSpec>;
#[doc = "Register `CPSW_CPDMA_INT_CPDMA_TH3_FREEBUFFER_REG` writer"]
pub type W = crate::W<CpswCpdmaIntCpdmaTh3FreebufferRegSpec>;
#[doc = "Field `CPDMA_THOST_FREE` reader - 14:0\\]
CPDMA THost Free Buffer Count Register"]
pub type CpdmaThostFreeR = crate::FieldReader<u16>;
#[doc = "Field `CPDMA_THOST_FREE` writer - 14:0\\]
CPDMA THost Free Buffer Count Register"]
pub type CpdmaThostFreeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - 14:0\\]
CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    pub fn cpdma_thost_free(&self) -> CpdmaThostFreeR {
        CpdmaThostFreeR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - 14:0\\]
CPDMA THost Free Buffer Count Register"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_free(&mut self) -> CpdmaThostFreeW<CpswCpdmaIntCpdmaTh3FreebufferRegSpec> {
        CpdmaThostFreeW::new(self, 0)
    }
}
#[doc = "CPDMA THost Free Buffer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_th3_freebuffer_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_th3_freebuffer_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaIntCpdmaTh3FreebufferRegSpec;
impl crate::RegisterSpec for CpswCpdmaIntCpdmaTh3FreebufferRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_int_cpdma_th3_freebuffer_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaIntCpdmaTh3FreebufferRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_int_cpdma_th3_freebuffer_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaIntCpdmaTh3FreebufferRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_INT_CPDMA_TH3_FREEBUFFER_REG to value 0"]
impl crate::Resettable for CpswCpdmaIntCpdmaTh3FreebufferRegSpec {
    const RESET_VALUE: u32 = 0;
}
