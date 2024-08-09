#[doc = "Register `HSM_WDT_CLK_GATE` reader"]
pub type R = crate::R<HsmWdtClkGateSpec>;
#[doc = "Register `HSM_WDT_CLK_GATE` writer"]
pub type W = crate::W<HsmWdtClkGateSpec>;
#[doc = "Field `gated` reader - 2:0\\]
writing '111' will gate clock for HSM WDT"]
pub type GatedR = crate::FieldReader;
#[doc = "Field `gated` writer - 2:0\\]
writing '111' will gate clock for HSM WDT"]
pub type GatedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will gate clock for HSM WDT"]
    #[inline(always)]
    pub fn gated(&self) -> GatedR {
        GatedR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will gate clock for HSM WDT"]
    #[inline(always)]
    #[must_use]
    pub fn gated(&mut self) -> GatedW<HsmWdtClkGateSpec> {
        GatedW::new(self, 0)
    }
}
#[doc = "HSM_WDT_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_wdt_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_wdt_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsmWdtClkGateSpec;
impl crate::RegisterSpec for HsmWdtClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsm_wdt_clk_gate::R`](R) reader structure"]
impl crate::Readable for HsmWdtClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`hsm_wdt_clk_gate::W`](W) writer structure"]
impl crate::Writable for HsmWdtClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSM_WDT_CLK_GATE to value 0"]
impl crate::Resettable for HsmWdtClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
