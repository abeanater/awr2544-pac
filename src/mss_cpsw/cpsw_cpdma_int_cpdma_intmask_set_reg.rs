#[doc = "Register `CPSW_CPDMA_INT_CPDMA_INTMASK_SET_REG` reader"]
pub type R = crate::R<CpswCpdmaIntCpdmaIntmaskSetRegSpec>;
#[doc = "Register `CPSW_CPDMA_INT_CPDMA_INTMASK_SET_REG` writer"]
pub type W = crate::W<CpswCpdmaIntCpdmaIntmaskSetRegSpec>;
#[doc = "Field `CPDMA_STATISTICS_INTERRUPT` reader - 0:0\\]
CPDMA Statistics Interrupt Masked SET"]
pub type CpdmaStatisticsInterruptR = crate::BitReader;
#[doc = "Field `CPDMA_STATISTICS_INTERRUPT` writer - 0:0\\]
CPDMA Statistics Interrupt Masked SET"]
pub type CpdmaStatisticsInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_HOST_INTERRUPT` reader - 1:1\\]
CPDMA HOST Interrupt Masked SET"]
pub type CpdmaHostInterruptR = crate::BitReader;
#[doc = "Field `CPDMA_HOST_INTERRUPT` writer - 1:1\\]
CPDMA HOST Interrupt Masked SET"]
pub type CpdmaHostInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CPDMA Statistics Interrupt Masked SET"]
    #[inline(always)]
    pub fn cpdma_statistics_interrupt(&self) -> CpdmaStatisticsInterruptR {
        CpdmaStatisticsInterruptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA HOST Interrupt Masked SET"]
    #[inline(always)]
    pub fn cpdma_host_interrupt(&self) -> CpdmaHostInterruptR {
        CpdmaHostInterruptR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CPDMA Statistics Interrupt Masked SET"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_statistics_interrupt(
        &mut self,
    ) -> CpdmaStatisticsInterruptW<CpswCpdmaIntCpdmaIntmaskSetRegSpec> {
        CpdmaStatisticsInterruptW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA HOST Interrupt Masked SET"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_host_interrupt(
        &mut self,
    ) -> CpdmaHostInterruptW<CpswCpdmaIntCpdmaIntmaskSetRegSpec> {
        CpdmaHostInterruptW::new(self, 1)
    }
}
#[doc = "CPDMA DMA Interrupt Status SET\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_intmask_set_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_intmask_set_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaIntCpdmaIntmaskSetRegSpec;
impl crate::RegisterSpec for CpswCpdmaIntCpdmaIntmaskSetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_int_cpdma_intmask_set_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaIntCpdmaIntmaskSetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_int_cpdma_intmask_set_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaIntCpdmaIntmaskSetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_INT_CPDMA_INTMASK_SET_REG to value 0"]
impl crate::Resettable for CpswCpdmaIntCpdmaIntmaskSetRegSpec {
    const RESET_VALUE: u32 = 0;
}
