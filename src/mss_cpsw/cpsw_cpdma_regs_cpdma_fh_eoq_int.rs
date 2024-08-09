#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_FH_EOQ_INT` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaFhEoqIntSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_FH_EOQ_INT` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaFhEoqIntSpec>;
#[doc = "Field `CPDMA_FHOST_INTERRUPT` reader - 7:0\\]
CPDMA FHost Interrupt on EOQ only"]
pub type CpdmaFhostInterruptR = crate::FieldReader;
#[doc = "Field `CPDMA_FHOST_INTERRUPT` writer - 7:0\\]
CPDMA FHost Interrupt on EOQ only"]
pub type CpdmaFhostInterruptW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPDMA_FHOST_HARDWARE` reader - 15:8\\]
CPDMA FHost Hardware Trigger Control Enable"]
pub type CpdmaFhostHardwareR = crate::FieldReader;
#[doc = "Field `CPDMA_FHOST_HARDWARE` writer - 15:8\\]
CPDMA FHost Hardware Trigger Control Enable"]
pub type CpdmaFhostHardwareW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
CPDMA FHost Interrupt on EOQ only"]
    #[inline(always)]
    pub fn cpdma_fhost_interrupt(&self) -> CpdmaFhostInterruptR {
        CpdmaFhostInterruptR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
CPDMA FHost Hardware Trigger Control Enable"]
    #[inline(always)]
    pub fn cpdma_fhost_hardware(&self) -> CpdmaFhostHardwareR {
        CpdmaFhostHardwareR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
CPDMA FHost Interrupt on EOQ only"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_interrupt(
        &mut self,
    ) -> CpdmaFhostInterruptW<CpswCpdmaRegsCpdmaFhEoqIntSpec> {
        CpdmaFhostInterruptW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
CPDMA FHost Hardware Trigger Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_hardware(&mut self) -> CpdmaFhostHardwareW<CpswCpdmaRegsCpdmaFhEoqIntSpec> {
        CpdmaFhostHardwareW::new(self, 8)
    }
}
#[doc = "CPDMA FHost Interrupt on EOQ only Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_fh_eoq_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_fh_eoq_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaFhEoqIntSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaFhEoqIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_fh_eoq_int::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaFhEoqIntSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_fh_eoq_int::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaFhEoqIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_FH_EOQ_INT to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaFhEoqIntSpec {
    const RESET_VALUE: u32 = 0;
}
