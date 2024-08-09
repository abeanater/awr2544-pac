#[doc = "Register `DSS_CSI_CLK_GATE` reader"]
pub type R = crate::R<DssCsiClkGateSpec>;
#[doc = "Register `DSS_CSI_CLK_GATE` writer"]
pub type W = crate::W<DssCsiClkGateSpec>;
#[doc = "Field `gated` reader - 2:0\\]
Clock gating config for DSS CSI2-TX Data should be loaded as multibit. Write 3'b000 : Clock is ungated Write 3'b111 : Clock is gated"]
pub type GatedR = crate::FieldReader;
#[doc = "Field `gated` writer - 2:0\\]
Clock gating config for DSS CSI2-TX Data should be loaded as multibit. Write 3'b000 : Clock is ungated Write 3'b111 : Clock is gated"]
pub type GatedW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Clock gating config for DSS CSI2-TX Data should be loaded as multibit. Write 3'b000 : Clock is ungated Write 3'b111 : Clock is gated"]
    #[inline(always)]
    pub fn gated(&self) -> GatedR {
        GatedR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Clock gating config for DSS CSI2-TX Data should be loaded as multibit. Write 3'b000 : Clock is ungated Write 3'b111 : Clock is gated"]
    #[inline(always)]
    #[must_use]
    pub fn gated(&mut self) -> GatedW<DssCsiClkGateSpec> {
        GatedW::new(self, 0)
    }
}
#[doc = "DSS_CSI_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_csi_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_csi_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssCsiClkGateSpec;
impl crate::RegisterSpec for DssCsiClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_csi_clk_gate::R`](R) reader structure"]
impl crate::Readable for DssCsiClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_csi_clk_gate::W`](W) writer structure"]
impl crate::Writable for DssCsiClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_CSI_CLK_GATE to value 0"]
impl crate::Resettable for DssCsiClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
