#[doc = "Register `HSM_RTI_CLK_DIV_VAL` reader"]
pub type R = crate::R<HsmRtiClkDivValSpec>;
#[doc = "Register `HSM_RTI_CLK_DIV_VAL` writer"]
pub type W = crate::W<HsmRtiClkDivValSpec>;
#[doc = "Field `clkdivr` reader - 11:0\\]
Divider value HSM RTI selected clock.Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register. Refer to TPR12 clock spec for clock reference"]
pub type ClkdivrR = crate::FieldReader<u16>;
#[doc = "Field `clkdivr` writer - 11:0\\]
Divider value HSM RTI selected clock.Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register. Refer to TPR12 clock spec for clock reference"]
pub type ClkdivrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Divider value HSM RTI selected clock.Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register. Refer to TPR12 clock spec for clock reference"]
    #[inline(always)]
    pub fn clkdivr(&self) -> ClkdivrR {
        ClkdivrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Divider value HSM RTI selected clock.Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register. Refer to TPR12 clock spec for clock reference"]
    #[inline(always)]
    #[must_use]
    pub fn clkdivr(&mut self) -> ClkdivrW<HsmRtiClkDivValSpec> {
        ClkdivrW::new(self, 0)
    }
}
#[doc = "HSM_RTI_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`hsm_rti_clk_div_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsm_rti_clk_div_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsmRtiClkDivValSpec;
impl crate::RegisterSpec for HsmRtiClkDivValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsm_rti_clk_div_val::R`](R) reader structure"]
impl crate::Readable for HsmRtiClkDivValSpec {}
#[doc = "`write(|w| ..)` method takes [`hsm_rti_clk_div_val::W`](W) writer structure"]
impl crate::Writable for HsmRtiClkDivValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSM_RTI_CLK_DIV_VAL to value 0"]
impl crate::Resettable for HsmRtiClkDivValSpec {
    const RESET_VALUE: u32 = 0;
}
