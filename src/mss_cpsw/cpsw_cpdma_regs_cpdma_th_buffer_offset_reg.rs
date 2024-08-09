#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_TH_BUFFER_OFFSET_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaThBufferOffsetRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_TH_BUFFER_OFFSET_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaThBufferOffsetRegSpec>;
#[doc = "Field `CPDMA_THOST_BUFFER` reader - 11:0\\]
CPDMA THost Buffer Offset Register"]
pub type CpdmaThostBufferR = crate::FieldReader<u16>;
#[doc = "Field `CPDMA_THOST_BUFFER` writer - 11:0\\]
CPDMA THost Buffer Offset Register"]
pub type CpdmaThostBufferW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
CPDMA THost Buffer Offset Register"]
    #[inline(always)]
    pub fn cpdma_thost_buffer(&self) -> CpdmaThostBufferR {
        CpdmaThostBufferR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
CPDMA THost Buffer Offset Register"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_buffer(
        &mut self,
    ) -> CpdmaThostBufferW<CpswCpdmaRegsCpdmaThBufferOffsetRegSpec> {
        CpdmaThostBufferW::new(self, 0)
    }
}
#[doc = "CPDMA THost Buffer Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_th_buffer_offset_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_th_buffer_offset_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaThBufferOffsetRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaThBufferOffsetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_th_buffer_offset_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaThBufferOffsetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_th_buffer_offset_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaThBufferOffsetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_TH_BUFFER_OFFSET_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaThBufferOffsetRegSpec {
    const RESET_VALUE: u32 = 0;
}
