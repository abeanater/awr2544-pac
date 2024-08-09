#[doc = "Register `PLL_PER_HSDIVIDER_CLKOUT2` reader"]
pub type R = crate::R<PllPerHsdividerClkout2Spec>;
#[doc = "Register `PLL_PER_HSDIVIDER_CLKOUT2` writer"]
pub type W = crate::W<PllPerHsdividerClkout2Spec>;
#[doc = "Field `DIV` reader - 4:0\\]
DPLL post-divider factor, M6, for internal clock generation. Divide values from 1 to 31. 0h (R/W) = Reserved"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - 4:0\\]
DPLL post-divider factor, M6, for internal clock generation. Divide values from 1 to 31. 0h (R/W) = Reserved"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DIVCHACK` reader - 5:5\\]
Toggle on this status bit after changing HSDIVIDER_CLKOUT2_DIV indicates that the change in divider value has taken effect"]
pub type DivchackR = crate::BitReader;
#[doc = "Field `DIVCHACK` writer - 5:5\\]
Toggle on this status bit after changing HSDIVIDER_CLKOUT2_DIV indicates that the change in divider value has taken effect"]
pub type DivchackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATE_CTRL` reader - 8:8\\]
Control gating of HSDIVIDER CLKOUT2 0h (R/W) = Automatically gate this clock when there is no dependency for it 1h (R/W) = Force this clock to stay enabled even if there is no request"]
pub type GateCtrlR = crate::BitReader;
#[doc = "Field `GATE_CTRL` writer - 8:8\\]
Control gating of HSDIVIDER CLKOUT2 0h (R/W) = Automatically gate this clock when there is no dependency for it 1h (R/W) = Force this clock to stay enabled even if there is no request"]
pub type GateCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS` reader - 9:9\\]
HSDIVIDER CLKOUT2 status 0h (R) = The clock output is gated 1h (R) = The clock output is enabled"]
pub type StatusR = crate::BitReader;
#[doc = "Field `STATUS` writer - 9:9\\]
HSDIVIDER CLKOUT2 status 0h (R) = The clock output is gated 1h (R) = The clock output is enabled"]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWDN` reader - 12:12\\]
Power down for HSDIVIDER M6 divider and hence CLKOUT2 output 0h (R/W) = CLKOUT2 divider active 1h (R/W) = CLKOUT2 divider is powered down"]
pub type PwdnR = crate::BitReader;
#[doc = "Field `PWDN` writer - 12:12\\]
Power down for HSDIVIDER M6 divider and hence CLKOUT2 output 0h (R/W) = CLKOUT2 divider active 1h (R/W) = CLKOUT2 divider is powered down"]
pub type PwdnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DPLL post-divider factor, M6, for internal clock generation. Divide values from 1 to 31. 0h (R/W) = Reserved"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Toggle on this status bit after changing HSDIVIDER_CLKOUT2_DIV indicates that the change in divider value has taken effect"]
    #[inline(always)]
    pub fn divchack(&self) -> DivchackR {
        DivchackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Control gating of HSDIVIDER CLKOUT2 0h (R/W) = Automatically gate this clock when there is no dependency for it 1h (R/W) = Force this clock to stay enabled even if there is no request"]
    #[inline(always)]
    pub fn gate_ctrl(&self) -> GateCtrlR {
        GateCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
HSDIVIDER CLKOUT2 status 0h (R) = The clock output is gated 1h (R) = The clock output is enabled"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Power down for HSDIVIDER M6 divider and hence CLKOUT2 output 0h (R/W) = CLKOUT2 divider active 1h (R/W) = CLKOUT2 divider is powered down"]
    #[inline(always)]
    pub fn pwdn(&self) -> PwdnR {
        PwdnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DPLL post-divider factor, M6, for internal clock generation. Divide values from 1 to 31. 0h (R/W) = Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<PllPerHsdividerClkout2Spec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Toggle on this status bit after changing HSDIVIDER_CLKOUT2_DIV indicates that the change in divider value has taken effect"]
    #[inline(always)]
    #[must_use]
    pub fn divchack(&mut self) -> DivchackW<PllPerHsdividerClkout2Spec> {
        DivchackW::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Control gating of HSDIVIDER CLKOUT2 0h (R/W) = Automatically gate this clock when there is no dependency for it 1h (R/W) = Force this clock to stay enabled even if there is no request"]
    #[inline(always)]
    #[must_use]
    pub fn gate_ctrl(&mut self) -> GateCtrlW<PllPerHsdividerClkout2Spec> {
        GateCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
HSDIVIDER CLKOUT2 status 0h (R) = The clock output is gated 1h (R) = The clock output is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<PllPerHsdividerClkout2Spec> {
        StatusW::new(self, 9)
    }
    #[doc = "Bit 12 - 12:12\\]
Power down for HSDIVIDER M6 divider and hence CLKOUT2 output 0h (R/W) = CLKOUT2 divider active 1h (R/W) = CLKOUT2 divider is powered down"]
    #[inline(always)]
    #[must_use]
    pub fn pwdn(&mut self) -> PwdnW<PllPerHsdividerClkout2Spec> {
        PwdnW::new(self, 12)
    }
}
#[doc = "PLL_PER_HSDIVIDER_CLKOUT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_hsdivider_clkout2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_hsdivider_clkout2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllPerHsdividerClkout2Spec;
impl crate::RegisterSpec for PllPerHsdividerClkout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_per_hsdivider_clkout2::R`](R) reader structure"]
impl crate::Readable for PllPerHsdividerClkout2Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_per_hsdivider_clkout2::W`](W) writer structure"]
impl crate::Writable for PllPerHsdividerClkout2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_PER_HSDIVIDER_CLKOUT2 to value 0"]
impl crate::Resettable for PllPerHsdividerClkout2Spec {
    const RESET_VALUE: u32 = 0;
}
