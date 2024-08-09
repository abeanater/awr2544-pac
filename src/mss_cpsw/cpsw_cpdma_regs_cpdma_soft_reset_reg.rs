#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_SOFT_RESET_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaSoftResetRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_SOFT_RESET_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaSoftResetRegSpec>;
#[doc = "Field `CPDMA_AND_CPSW` reader - 0:0\\]
CPDMA and CPSW Soft Reset Enable"]
pub type CpdmaAndCpswR = crate::BitReader;
#[doc = "Field `CPDMA_AND_CPSW` writer - 0:0\\]
CPDMA and CPSW Soft Reset Enable"]
pub type CpdmaAndCpswW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CPDMA and CPSW Soft Reset Enable"]
    #[inline(always)]
    pub fn cpdma_and_cpsw(&self) -> CpdmaAndCpswR {
        CpdmaAndCpswR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CPDMA and CPSW Soft Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_and_cpsw(&mut self) -> CpdmaAndCpswW<CpswCpdmaRegsCpdmaSoftResetRegSpec> {
        CpdmaAndCpswW::new(self, 0)
    }
}
#[doc = "CPDMA Soft Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_soft_reset_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_soft_reset_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaSoftResetRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaSoftResetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_soft_reset_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaSoftResetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_soft_reset_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaSoftResetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_SOFT_RESET_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaSoftResetRegSpec {
    const RESET_VALUE: u32 = 0;
}
