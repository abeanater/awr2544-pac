#[doc = "Register `TRCCLKOUT_DIV_VAL` reader"]
pub type R = crate::R<TrcclkoutDivValSpec>;
#[doc = "Register `TRCCLKOUT_DIV_VAL` writer"]
pub type W = crate::W<TrcclkoutDivValSpec>;
#[doc = "Field `clkdiv` reader - 11:0\\]
Divider value for TRC Clkout selected clock. Data should be loaded as multibit. For example: if divider value of 0x5 should be selected then 0x555 should be configured to the register."]
pub type ClkdivR = crate::FieldReader<u16>;
#[doc = "Field `clkdiv` writer - 11:0\\]
Divider value for TRC Clkout selected clock. Data should be loaded as multibit. For example: if divider value of 0x5 should be selected then 0x555 should be configured to the register."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Divider value for TRC Clkout selected clock. Data should be loaded as multibit. For example: if divider value of 0x5 should be selected then 0x555 should be configured to the register."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Divider value for TRC Clkout selected clock. Data should be loaded as multibit. For example: if divider value of 0x5 should be selected then 0x555 should be configured to the register."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<TrcclkoutDivValSpec> {
        ClkdivW::new(self, 0)
    }
}
#[doc = "TRCCLKOUT_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclkout_div_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclkout_div_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcclkoutDivValSpec;
impl crate::RegisterSpec for TrcclkoutDivValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcclkout_div_val::R`](R) reader structure"]
impl crate::Readable for TrcclkoutDivValSpec {}
#[doc = "`write(|w| ..)` method takes [`trcclkout_div_val::W`](W) writer structure"]
impl crate::Writable for TrcclkoutDivValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCLKOUT_DIV_VAL to value 0"]
impl crate::Resettable for TrcclkoutDivValSpec {
    const RESET_VALUE: u32 = 0;
}
