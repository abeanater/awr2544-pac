#[doc = "Register `PLL_DSP_HSDIVIDER_CLKOUT2` reader"]
pub type R = crate::R<PllDspHsdividerClkout2Spec>;
#[doc = "Register `PLL_DSP_HSDIVIDER_CLKOUT2` writer"]
pub type W = crate::W<PllDspHsdividerClkout2Spec>;
#[doc = "Field `DIV` reader - 4:0\\]
Do not use. TI Reserved."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - 4:0\\]
Do not use. TI Reserved."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DIVCHACK` reader - 5:5\\]
Do not use. TI Reserved."]
pub type DivchackR = crate::BitReader;
#[doc = "Field `DIVCHACK` writer - 5:5\\]
Do not use. TI Reserved."]
pub type DivchackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATE_CTRL` reader - 8:8\\]
Do not use. TI Reserved."]
pub type GateCtrlR = crate::BitReader;
#[doc = "Field `GATE_CTRL` writer - 8:8\\]
Do not use. TI Reserved."]
pub type GateCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS` reader - 9:9\\]
Do not use. TI Reserved."]
pub type StatusR = crate::BitReader;
#[doc = "Field `STATUS` writer - 9:9\\]
Do not use. TI Reserved."]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWDN` reader - 12:12\\]
Do not use. TI Reserved."]
pub type PwdnR = crate::BitReader;
#[doc = "Field `PWDN` writer - 12:12\\]
Do not use. TI Reserved."]
pub type PwdnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn divchack(&self) -> DivchackR {
        DivchackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn gate_ctrl(&self) -> GateCtrlR {
        GateCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn pwdn(&self) -> PwdnR {
        PwdnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<PllDspHsdividerClkout2Spec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn divchack(&mut self) -> DivchackW<PllDspHsdividerClkout2Spec> {
        DivchackW::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn gate_ctrl(&mut self) -> GateCtrlW<PllDspHsdividerClkout2Spec> {
        GateCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<PllDspHsdividerClkout2Spec> {
        StatusW::new(self, 9)
    }
    #[doc = "Bit 12 - 12:12\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pwdn(&mut self) -> PwdnW<PllDspHsdividerClkout2Spec> {
        PwdnW::new(self, 12)
    }
}
#[doc = "PLL_DSP_HSDIVIDER_CLKOUT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_hsdivider_clkout2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_hsdivider_clkout2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDspHsdividerClkout2Spec;
impl crate::RegisterSpec for PllDspHsdividerClkout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_dsp_hsdivider_clkout2::R`](R) reader structure"]
impl crate::Readable for PllDspHsdividerClkout2Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_dsp_hsdivider_clkout2::W`](W) writer structure"]
impl crate::Writable for PllDspHsdividerClkout2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_DSP_HSDIVIDER_CLKOUT2 to value 0"]
impl crate::Resettable for PllDspHsdividerClkout2Spec {
    const RESET_VALUE: u32 = 0;
}
