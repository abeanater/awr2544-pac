#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_FH_IDVER_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaFhIdverRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_FH_IDVER_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaFhIdverRegSpec>;
#[doc = "Field `CPDMA_FHOST_IDVER` reader - 31:0\\]
CPDMA FHost IDVER"]
pub type CpdmaFhostIdverR = crate::FieldReader<u32>;
#[doc = "Field `CPDMA_FHOST_IDVER` writer - 31:0\\]
CPDMA FHost IDVER"]
pub type CpdmaFhostIdverW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CPDMA FHost IDVER"]
    #[inline(always)]
    pub fn cpdma_fhost_idver(&self) -> CpdmaFhostIdverR {
        CpdmaFhostIdverR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CPDMA FHost IDVER"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_idver(&mut self) -> CpdmaFhostIdverW<CpswCpdmaRegsCpdmaFhIdverRegSpec> {
        CpdmaFhostIdverW::new(self, 0)
    }
}
#[doc = "CPDMA FHost IDVER\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_fh_idver_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_fh_idver_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaFhIdverRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaFhIdverRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_fh_idver_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaFhIdverRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_fh_idver_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaFhIdverRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_FH_IDVER_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaFhIdverRegSpec {
    const RESET_VALUE: u32 = 0;
}
