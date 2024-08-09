#[doc = "Register `WARM_RESET_CONFIG` reader"]
pub type R = crate::R<WarmResetConfigSpec>;
#[doc = "Register `WARM_RESET_CONFIG` writer"]
pub type W = crate::W<WarmResetConfigSpec>;
#[doc = "Field `pad_bypass` reader - 2:0\\]
Bypass the Warm reset from Pad Input Data should be loaded as multibit. Write 3'b000 : Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
pub type PadBypassR = crate::FieldReader;
#[doc = "Field `pad_bypass` writer - 2:0\\]
Bypass the Warm reset from Pad Input Data should be loaded as multibit. Write 3'b000 : Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
pub type PadBypassW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sw_rst` reader - 10:8\\]
Data should be loaded as multibit. Write 3'b000 to assert warm reset from SW Write 3'b111 to deassert warm reset from SW if this is the only source of warm reset"]
pub type SwRstR = crate::FieldReader;
#[doc = "Field `sw_rst` writer - 10:8\\]
Data should be loaded as multibit. Write 3'b000 to assert warm reset from SW Write 3'b111 to deassert warm reset from SW if this is the only source of warm reset"]
pub type SwRstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wdog_rst_en` reader - 18:16\\]
Data should be loaded as multibit. Write 3'b000 to disable MSS Watchdog control on Warm reset Write 3'b111 enable MSS Watchdog to control Warm reset"]
pub type WdogRstEnR = crate::FieldReader;
#[doc = "Field `wdog_rst_en` writer - 18:16\\]
Data should be loaded as multibit. Write 3'b000 to disable MSS Watchdog control on Warm reset Write 3'b111 enable MSS Watchdog to control Warm reset"]
pub type WdogRstEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Bypass the Warm reset from Pad Input Data should be loaded as multibit. Write 3'b000 : Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
    #[inline(always)]
    pub fn pad_bypass(&self) -> PadBypassR {
        PadBypassR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Data should be loaded as multibit. Write 3'b000 to assert warm reset from SW Write 3'b111 to deassert warm reset from SW if this is the only source of warm reset"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SwRstR {
        SwRstR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Data should be loaded as multibit. Write 3'b000 to disable MSS Watchdog control on Warm reset Write 3'b111 enable MSS Watchdog to control Warm reset"]
    #[inline(always)]
    pub fn wdog_rst_en(&self) -> WdogRstEnR {
        WdogRstEnR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Bypass the Warm reset from Pad Input Data should be loaded as multibit. Write 3'b000 : Reset is not asserted by SW (multibit 000) Write 3'b111 : Reset is asserted by SW (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_bypass(&mut self) -> PadBypassW<WarmResetConfigSpec> {
        PadBypassW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Data should be loaded as multibit. Write 3'b000 to assert warm reset from SW Write 3'b111 to deassert warm reset from SW if this is the only source of warm reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst(&mut self) -> SwRstW<WarmResetConfigSpec> {
        SwRstW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Data should be loaded as multibit. Write 3'b000 to disable MSS Watchdog control on Warm reset Write 3'b111 enable MSS Watchdog to control Warm reset"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_rst_en(&mut self) -> WdogRstEnW<WarmResetConfigSpec> {
        WdogRstEnW::new(self, 16)
    }
}
#[doc = "WARM_RESET_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`warm_reset_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`warm_reset_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WarmResetConfigSpec;
impl crate::RegisterSpec for WarmResetConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`warm_reset_config::R`](R) reader structure"]
impl crate::Readable for WarmResetConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`warm_reset_config::W`](W) writer structure"]
impl crate::Writable for WarmResetConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WARM_RESET_CONFIG to value 0"]
impl crate::Resettable for WarmResetConfigSpec {
    const RESET_VALUE: u32 = 0;
}
