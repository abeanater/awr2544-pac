#[doc = "Register `PLL_DSP_CLKCTRL` reader"]
pub type R = crate::R<PllDspClkctrlSpec>;
#[doc = "Register `PLL_DSP_CLKCTRL` writer"]
pub type W = crate::W<PllDspClkctrlSpec>;
#[doc = "Field `TINTZ` reader - 0:0\\]
Do not use. TI Reserved."]
pub type TintzR = crate::BitReader;
#[doc = "Field `TINTZ` writer - 0:0\\]
Do not use. TI Reserved."]
pub type TintzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSCTYPE` reader - 1:1\\]
Do not use. TI Reserved."]
pub type SsctypeR = crate::BitReader;
#[doc = "Field `SSCTYPE` writer - 1:1\\]
Do not use. TI Reserved."]
pub type SsctypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RELAXED_LOCK` reader - 8:8\\]
Do not use. TI Reserved."]
pub type RelaxedLockR = crate::BitReader;
#[doc = "Field `RELAXED_LOCK` writer - 8:8\\]
Do not use. TI Reserved."]
pub type RelaxedLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELFREQDCO` reader - 12:10\\]
Do not use. TI Reserved."]
pub type SelfreqdcoR = crate::FieldReader;
#[doc = "Field `SELFREQDCO` writer - 12:10\\]
Do not use. TI Reserved."]
pub type SelfreqdcoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STOPMODE` reader - 14:14\\]
Do not use. TI Reserved."]
pub type StopmodeR = crate::BitReader;
#[doc = "Field `STOPMODE` writer - 14:14\\]
Do not use. TI Reserved."]
pub type StopmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M2PWDNZ` reader - 16:16\\]
Do not use. TI Reserved."]
pub type M2pwdnzR = crate::BitReader;
#[doc = "Field `M2PWDNZ` writer - 16:16\\]
Do not use. TI Reserved."]
pub type M2pwdnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDCOLDOPWDNZ` reader - 17:17\\]
Do not use. TI Reserved."]
pub type ClkdcoldopwdnzR = crate::BitReader;
#[doc = "Field `CLKDCOLDOPWDNZ` writer - 17:17\\]
Do not use. TI Reserved."]
pub type ClkdcoldopwdnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULOWCLKEN` reader - 18:18\\]
Do not use. TI Reserved."]
pub type UlowclkenR = crate::BitReader;
#[doc = "Field `ULOWCLKEN` writer - 18:18\\]
Do not use. TI Reserved."]
pub type UlowclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUTLDOEN` reader - 19:19\\]
Do not use. TI Reserved."]
pub type ClkoutldoenR = crate::BitReader;
#[doc = "Field `CLKOUTLDOEN` writer - 19:19\\]
Do not use. TI Reserved."]
pub type ClkoutldoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUTEN` reader - 20:20\\]
Do not use. TI Reserved."]
pub type ClkoutenR = crate::BitReader;
#[doc = "Field `CLKOUTEN` writer - 20:20\\]
Do not use. TI Reserved."]
pub type ClkoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBYRET` reader - 21:21\\]
Do not use. TI Reserved."]
pub type StbyretR = crate::BitReader;
#[doc = "Field `STBYRET` writer - 21:21\\]
Do not use. TI Reserved."]
pub type StbyretW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSACKZ` reader - 22:22\\]
Do not use. TI Reserved."]
pub type BypassackzR = crate::BitReader;
#[doc = "Field `BYPASSACKZ` writer - 22:22\\]
Do not use. TI Reserved."]
pub type BypassackzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - 23:23\\]
Do not use. TI Reserved."]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - 23:23\\]
Do not use. TI Reserved."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWELLTRIM` reader - 28:24\\]
Do not use. TI Reserved."]
pub type NwelltrimR = crate::FieldReader;
#[doc = "Field `NWELLTRIM` writer - 28:24\\]
Do not use. TI Reserved."]
pub type NwelltrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLKDCOLDOEN` reader - 29:29\\]
Do not use. TI Reserved."]
pub type ClkdcoldoenR = crate::BitReader;
#[doc = "Field `CLKDCOLDOEN` writer - 29:29\\]
Do not use. TI Reserved."]
pub type ClkdcoldoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSSC` reader - 30:30\\]
Do not use. TI Reserved."]
pub type EnsscR = crate::BitReader;
#[doc = "Field `ENSSC` writer - 30:30\\]
Do not use. TI Reserved."]
pub type EnsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYCLESLIPEN` reader - 31:31\\]
Do not use. TI Reserved."]
pub type CycleslipenR = crate::BitReader;
#[doc = "Field `CYCLESLIPEN` writer - 31:31\\]
Do not use. TI Reserved."]
pub type CycleslipenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn tintz(&self) -> TintzR {
        TintzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn ssctype(&self) -> SsctypeR {
        SsctypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn relaxed_lock(&self) -> RelaxedLockR {
        RelaxedLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn selfreqdco(&self) -> SelfreqdcoR {
        SelfreqdcoR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn stopmode(&self) -> StopmodeR {
        StopmodeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn m2pwdnz(&self) -> M2pwdnzR {
        M2pwdnzR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn clkdcoldopwdnz(&self) -> ClkdcoldopwdnzR {
        ClkdcoldopwdnzR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn ulowclken(&self) -> UlowclkenR {
        UlowclkenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn clkoutldoen(&self) -> ClkoutldoenR {
        ClkoutldoenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn clkouten(&self) -> ClkoutenR {
        ClkoutenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn stbyret(&self) -> StbyretR {
        StbyretR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn bypassackz(&self) -> BypassackzR {
        BypassackzR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn nwelltrim(&self) -> NwelltrimR {
        NwelltrimR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn clkdcoldoen(&self) -> ClkdcoldoenR {
        ClkdcoldoenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn enssc(&self) -> EnsscR {
        EnsscR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn cycleslipen(&self) -> CycleslipenR {
        CycleslipenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tintz(&mut self) -> TintzW<PllDspClkctrlSpec> {
        TintzW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ssctype(&mut self) -> SsctypeW<PllDspClkctrlSpec> {
        SsctypeW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn relaxed_lock(&mut self) -> RelaxedLockW<PllDspClkctrlSpec> {
        RelaxedLockW::new(self, 8)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn selfreqdco(&mut self) -> SelfreqdcoW<PllDspClkctrlSpec> {
        SelfreqdcoW::new(self, 10)
    }
    #[doc = "Bit 14 - 14:14\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn stopmode(&mut self) -> StopmodeW<PllDspClkctrlSpec> {
        StopmodeW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn m2pwdnz(&mut self) -> M2pwdnzW<PllDspClkctrlSpec> {
        M2pwdnzW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn clkdcoldopwdnz(&mut self) -> ClkdcoldopwdnzW<PllDspClkctrlSpec> {
        ClkdcoldopwdnzW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ulowclken(&mut self) -> UlowclkenW<PllDspClkctrlSpec> {
        UlowclkenW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn clkoutldoen(&mut self) -> ClkoutldoenW<PllDspClkctrlSpec> {
        ClkoutldoenW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn clkouten(&mut self) -> ClkoutenW<PllDspClkctrlSpec> {
        ClkoutenW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn stbyret(&mut self) -> StbyretW<PllDspClkctrlSpec> {
        StbyretW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bypassackz(&mut self) -> BypassackzW<PllDspClkctrlSpec> {
        BypassackzW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<PllDspClkctrlSpec> {
        IdleW::new(self, 23)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn nwelltrim(&mut self) -> NwelltrimW<PllDspClkctrlSpec> {
        NwelltrimW::new(self, 24)
    }
    #[doc = "Bit 29 - 29:29\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn clkdcoldoen(&mut self) -> ClkdcoldoenW<PllDspClkctrlSpec> {
        ClkdcoldoenW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn enssc(&mut self) -> EnsscW<PllDspClkctrlSpec> {
        EnsscW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn cycleslipen(&mut self) -> CycleslipenW<PllDspClkctrlSpec> {
        CycleslipenW::new(self, 31)
    }
}
#[doc = "PLL_DSP_CLKCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_dsp_clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_dsp_clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDspClkctrlSpec;
impl crate::RegisterSpec for PllDspClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_dsp_clkctrl::R`](R) reader structure"]
impl crate::Readable for PllDspClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_dsp_clkctrl::W`](W) writer structure"]
impl crate::Writable for PllDspClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_DSP_CLKCTRL to value 0"]
impl crate::Resettable for PllDspClkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
