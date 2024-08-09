#[doc = "Register `RSS_FRC_CLK_DIV_VAL` reader"]
pub type R = crate::R<RssFrcClkDivValSpec>;
#[doc = "Register `RSS_FRC_CLK_DIV_VAL` writer"]
pub type W = crate::W<RssFrcClkDivValSpec>;
#[doc = "Field `clkdivr` reader - 11:0\\]
Divider value for FRC selected clock.Data should be loaded as multibit. For example: if divider value of 8(1000) should be selected then '100010001000' should be configured to the register. Refer to TPR12 clock planner for clock reference"]
pub type ClkdivrR = crate::FieldReader<u16>;
#[doc = "Field `clkdivr` writer - 11:0\\]
Divider value for FRC selected clock.Data should be loaded as multibit. For example: if divider value of 8(1000) should be selected then '100010001000' should be configured to the register. Refer to TPR12 clock planner for clock reference"]
pub type ClkdivrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Divider value for FRC selected clock.Data should be loaded as multibit. For example: if divider value of 8(1000) should be selected then '100010001000' should be configured to the register. Refer to TPR12 clock planner for clock reference"]
    #[inline(always)]
    pub fn clkdivr(&self) -> ClkdivrR {
        ClkdivrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Divider value for FRC selected clock.Data should be loaded as multibit. For example: if divider value of 8(1000) should be selected then '100010001000' should be configured to the register. Refer to TPR12 clock planner for clock reference"]
    #[inline(always)]
    #[must_use]
    pub fn clkdivr(&mut self) -> ClkdivrW<RssFrcClkDivValSpec> {
        ClkdivrW::new(self, 0)
    }
}
#[doc = "RSS_FRC_CLK_DIV_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_frc_clk_div_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_frc_clk_div_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssFrcClkDivValSpec;
impl crate::RegisterSpec for RssFrcClkDivValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_frc_clk_div_val::R`](R) reader structure"]
impl crate::Readable for RssFrcClkDivValSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_frc_clk_div_val::W`](W) writer structure"]
impl crate::Writable for RssFrcClkDivValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_FRC_CLK_DIV_VAL to value 0"]
impl crate::Resettable for RssFrcClkDivValSpec {
    const RESET_VALUE: u32 = 0;
}
