#[doc = "Register `CPSW_CPDMA_SRAM_CPDMA_FH5_HDP_REG` reader"]
pub type R = crate::R<CpswCpdmaSramCpdmaFh5HdpRegSpec>;
#[doc = "Register `CPSW_CPDMA_SRAM_CPDMA_FH5_HDP_REG` writer"]
pub type W = crate::W<CpswCpdmaSramCpdmaFh5HdpRegSpec>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 31:0\\]
CPDMA FHost Channel 5 Head Descriptor Pointer"]
pub type CpdmaFhostChannelR = crate::FieldReader<u32>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 31:0\\]
CPDMA FHost Channel 5 Head Descriptor Pointer"]
pub type CpdmaFhostChannelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CPDMA FHost Channel 5 Head Descriptor Pointer"]
    #[inline(always)]
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CPDMA FHost Channel 5 Head Descriptor Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel(&mut self) -> CpdmaFhostChannelW<CpswCpdmaSramCpdmaFh5HdpRegSpec> {
        CpdmaFhostChannelW::new(self, 0)
    }
}
#[doc = "CPDMA FHost Channel 5 Head Descriptor Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_sram_cpdma_fh5_hdp_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_sram_cpdma_fh5_hdp_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaSramCpdmaFh5HdpRegSpec;
impl crate::RegisterSpec for CpswCpdmaSramCpdmaFh5HdpRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_sram_cpdma_fh5_hdp_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaSramCpdmaFh5HdpRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_sram_cpdma_fh5_hdp_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaSramCpdmaFh5HdpRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_SRAM_CPDMA_FH5_HDP_REG to value 0"]
impl crate::Resettable for CpswCpdmaSramCpdmaFh5HdpRegSpec {
    const RESET_VALUE: u32 = 0;
}
