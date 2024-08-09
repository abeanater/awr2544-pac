#[doc = "Register `PLLC_CLK2_GATE` reader"]
pub type R = crate::R<PllcClk2GateSpec>;
#[doc = "Register `PLLC_CLK2_GATE` writer"]
pub type W = crate::W<PllcClk2GateSpec>;
#[doc = "Field `gated` reader - 2:0\\]
Clock gatring config for PLLCORE_HSDIV_CLKOUT2_MUXED. Data should be loaded as multibit. Write 3'b000 : Clock is ungated (multibit 000) Write 3'b111 : Clock is gated (multibit 111)"]
pub type GatedR = crate::FieldReader;
#[doc = "Field `gated` writer - 2:0\\]
Clock gatring config for PLLCORE_HSDIV_CLKOUT2_MUXED. Data should be loaded as multibit. Write 3'b000 : Clock is ungated (multibit 000) Write 3'b111 : Clock is gated (multibit 111)"]
pub type GatedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Clock gatring config for PLLCORE_HSDIV_CLKOUT2_MUXED. Data should be loaded as multibit. Write 3'b000 : Clock is ungated (multibit 000) Write 3'b111 : Clock is gated (multibit 111)"]
    #[inline(always)]
    pub fn gated(&self) -> GatedR {
        GatedR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Clock gatring config for PLLCORE_HSDIV_CLKOUT2_MUXED. Data should be loaded as multibit. Write 3'b000 : Clock is ungated (multibit 000) Write 3'b111 : Clock is gated (multibit 111)"]
    #[inline(always)]
    #[must_use]
    pub fn gated(&mut self) -> GatedW<PllcClk2GateSpec> {
        GatedW::new(self, 0)
    }
}
#[doc = "PLLC_CLK2_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`pllc_clk2_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllc_clk2_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcClk2GateSpec;
impl crate::RegisterSpec for PllcClk2GateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllc_clk2_gate::R`](R) reader structure"]
impl crate::Readable for PllcClk2GateSpec {}
#[doc = "`write(|w| ..)` method takes [`pllc_clk2_gate::W`](W) writer structure"]
impl crate::Writable for PllcClk2GateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLC_CLK2_GATE to value 0"]
impl crate::Resettable for PllcClk2GateSpec {
    const RESET_VALUE: u32 = 0;
}
