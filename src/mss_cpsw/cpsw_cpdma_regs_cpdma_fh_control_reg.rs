#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_FH_CONTROL_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaFhControlRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_FH_CONTROL_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaFhControlRegSpec>;
#[doc = "Field `CPDMA_FHOST_DMA` reader - 0:0\\]
CPDMA FHost DMA Enable"]
pub type CpdmaFhostDmaR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_DMA` writer - 0:0\\]
CPDMA FHost DMA Enable"]
pub type CpdmaFhostDmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CPDMA FHost DMA Enable"]
    #[inline(always)]
    pub fn cpdma_fhost_dma(&self) -> CpdmaFhostDmaR {
        CpdmaFhostDmaR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CPDMA FHost DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_dma(&mut self) -> CpdmaFhostDmaW<CpswCpdmaRegsCpdmaFhControlRegSpec> {
        CpdmaFhostDmaW::new(self, 0)
    }
}
#[doc = "CPDMA FHost Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_fh_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_fh_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaFhControlRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaFhControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_fh_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaFhControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_fh_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaFhControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_FH_CONTROL_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaFhControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
