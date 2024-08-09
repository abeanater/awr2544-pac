#[doc = "Register `CPTS_INTSTAT_MASKED_REG` reader"]
pub type R = crate::R<CptsIntstatMaskedRegSpec>;
#[doc = "Register `CPTS_INTSTAT_MASKED_REG` writer"]
pub type W = crate::W<CptsIntstatMaskedRegSpec>;
#[doc = "Field `TS_PEND_MASKED_INTERRUPT` reader - 0:0\\]
TS_PEND masked interrupt read (after enable)"]
pub type TsPendMaskedInterruptR = crate::BitReader;
#[doc = "Field `TS_PEND_MASKED_INTERRUPT` writer - 0:0\\]
TS_PEND masked interrupt read (after enable)"]
pub type TsPendMaskedInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND masked interrupt read (after enable)"]
    #[inline(always)]
    pub fn ts_pend_masked_interrupt(&self) -> TsPendMaskedInterruptR {
        TsPendMaskedInterruptR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND masked interrupt read (after enable)"]
    #[inline(always)]
    #[must_use]
    pub fn ts_pend_masked_interrupt(&mut self) -> TsPendMaskedInterruptW<CptsIntstatMaskedRegSpec> {
        TsPendMaskedInterruptW::new(self, 0)
    }
}
#[doc = "Interrupt Status Register Masked\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_intstat_masked_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_intstat_masked_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsIntstatMaskedRegSpec;
impl crate::RegisterSpec for CptsIntstatMaskedRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_intstat_masked_reg::R`](R) reader structure"]
impl crate::Readable for CptsIntstatMaskedRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_intstat_masked_reg::W`](W) writer structure"]
impl crate::Writable for CptsIntstatMaskedRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_INTSTAT_MASKED_REG to value 0"]
impl crate::Resettable for CptsIntstatMaskedRegSpec {
    const RESET_VALUE: u32 = 0;
}
