#[doc = "Register `MSS_GPADC_CLK_GATE` reader"]
pub type R = crate::R<MssGpadcClkGateSpec>;
#[doc = "Register `MSS_GPADC_CLK_GATE` writer"]
pub type W = crate::W<MssGpadcClkGateSpec>;
#[doc = "Field `gated` reader - 2:0\\]
writing '111' will gate clock for MSS GPADC"]
pub type GatedR = crate::FieldReader;
#[doc = "Field `gated` writer - 2:0\\]
writing '111' will gate clock for MSS GPADC"]
pub type GatedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will gate clock for MSS GPADC"]
    #[inline(always)]
    pub fn gated(&self) -> GatedR {
        GatedR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will gate clock for MSS GPADC"]
    #[inline(always)]
    #[must_use]
    pub fn gated(&mut self) -> GatedW<MssGpadcClkGateSpec> {
        GatedW::new(self, 0)
    }
}
#[doc = "MSS_GPADC_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssGpadcClkGateSpec;
impl crate::RegisterSpec for MssGpadcClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_gpadc_clk_gate::R`](R) reader structure"]
impl crate::Readable for MssGpadcClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_gpadc_clk_gate::W`](W) writer structure"]
impl crate::Writable for MssGpadcClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_GPADC_CLK_GATE to value 0"]
impl crate::Resettable for MssGpadcClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
