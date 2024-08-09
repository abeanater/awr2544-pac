#[doc = "Register `CPTS_INT_ENABLE_REG` reader"]
pub type R = crate::R<CptsIntEnableRegSpec>;
#[doc = "Register `CPTS_INT_ENABLE_REG` writer"]
pub type W = crate::W<CptsIntEnableRegSpec>;
#[doc = "Field `TS_PEND_MASKED_INTERRUPT` reader - 0:0\\]
TS_PEND masked interrupt enable"]
pub type TsPendMaskedInterruptR = crate::BitReader;
#[doc = "Field `TS_PEND_MASKED_INTERRUPT` writer - 0:0\\]
TS_PEND masked interrupt enable"]
pub type TsPendMaskedInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND masked interrupt enable"]
    #[inline(always)]
    pub fn ts_pend_masked_interrupt(&self) -> TsPendMaskedInterruptR {
        TsPendMaskedInterruptR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND masked interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts_pend_masked_interrupt(&mut self) -> TsPendMaskedInterruptW<CptsIntEnableRegSpec> {
        TsPendMaskedInterruptW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_int_enable_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_int_enable_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsIntEnableRegSpec;
impl crate::RegisterSpec for CptsIntEnableRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_int_enable_reg::R`](R) reader structure"]
impl crate::Readable for CptsIntEnableRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_int_enable_reg::W`](W) writer structure"]
impl crate::Writable for CptsIntEnableRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_INT_ENABLE_REG to value 0"]
impl crate::Resettable for CptsIntEnableRegSpec {
    const RESET_VALUE: u32 = 0;
}
