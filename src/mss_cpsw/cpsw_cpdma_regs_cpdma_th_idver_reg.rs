#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_TH_IDVER_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaThIdverRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_TH_IDVER_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaThIdverRegSpec>;
#[doc = "Field `CPDMA_THOST_IDVER` reader - 31:0\\]
CPDMA THost IDVER"]
pub type CpdmaThostIdverR = crate::FieldReader<u32>;
#[doc = "Field `CPDMA_THOST_IDVER` writer - 31:0\\]
CPDMA THost IDVER"]
pub type CpdmaThostIdverW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CPDMA THost IDVER"]
    #[inline(always)]
    pub fn cpdma_thost_idver(&self) -> CpdmaThostIdverR {
        CpdmaThostIdverR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CPDMA THost IDVER"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_idver(&mut self) -> CpdmaThostIdverW<CpswCpdmaRegsCpdmaThIdverRegSpec> {
        CpdmaThostIdverW::new(self, 0)
    }
}
#[doc = "CPDMA THost IDVER\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_th_idver_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_th_idver_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaThIdverRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaThIdverRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_th_idver_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaThIdverRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_th_idver_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaThIdverRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_TH_IDVER_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaThIdverRegSpec {
    const RESET_VALUE: u32 = 0;
}
