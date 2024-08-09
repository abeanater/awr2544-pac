#[doc = "Register `PLL_CORE_CLKCTRL` reader"]
pub type R = crate::R<PllCoreClkctrlSpec>;
#[doc = "Register `PLL_CORE_CLKCTRL` writer"]
pub type W = crate::W<PllCoreClkctrlSpec>;
#[doc = "Field `TINTZ` reader - 0:0\\]
PLL core soft reset"]
pub type TintzR = crate::BitReader;
#[doc = "Field `TINTZ` writer - 0:0\\]
PLL core soft reset"]
pub type TintzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSCTYPE` reader - 1:1\\]
SSC Type"]
pub type SsctypeR = crate::BitReader;
#[doc = "Field `SSCTYPE` writer - 1:1\\]
SSC Type"]
pub type SsctypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RELAXED_LOCK` reader - 8:8\\]
Decides when FREQLOCK asserted 0x0: FREQLOCK asserted when DC frequency error less than 1% 0x1: FREQLOCK asserted when DC frequency error less than 2%"]
pub type RelaxedLockR = crate::BitReader;
#[doc = "Field `RELAXED_LOCK` writer - 8:8\\]
Decides when FREQLOCK asserted 0x0: FREQLOCK asserted when DC frequency error less than 1% 0x1: FREQLOCK asserted when DC frequency error less than 2%"]
pub type RelaxedLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELFREQDCO` reader - 12:10\\]
DCO Clock (DCOCLK = CLKINP * \\[M/(N+1)\\]) frequency range selector. 0x0: Reserved 0x2: HS2 : DCOCLK range is from 500 MHz to 1000 MHz 0x3: Reserved 0x4: HS1: DCOCLK range is from 1000 MHz to 2000 MHz 0x5: Reserved"]
pub type SelfreqdcoR = crate::FieldReader;
#[doc = "Field `SELFREQDCO` writer - 12:10\\]
DCO Clock (DCOCLK = CLKINP * \\[M/(N+1)\\]) frequency range selector. 0x0: Reserved 0x2: HS2 : DCOCLK range is from 500 MHz to 1000 MHz 0x3: Reserved 0x4: HS1: DCOCLK range is from 1000 MHz to 2000 MHz 0x5: Reserved"]
pub type SelfreqdcoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STOPMODE` reader - 14:14\\]
When in Lossclk/Stbyret 0x0 : Limp mode 0x1 : Stopmode"]
pub type StopmodeR = crate::BitReader;
#[doc = "Field `STOPMODE` writer - 14:14\\]
When in Lossclk/Stbyret 0x0 : Limp mode 0x1 : Stopmode"]
pub type StopmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M2PWDNZ` reader - 16:16\\]
M2 divider power down mode 0x0: Asynchronous power down for M2 divider 0x1 : M2 divider is functional"]
pub type M2pwdnzR = crate::BitReader;
#[doc = "Field `M2PWDNZ` writer - 16:16\\]
M2 divider power down mode 0x0: Asynchronous power down for M2 divider 0x1 : M2 divider is functional"]
pub type M2pwdnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDCOLDOPWDNZ` reader - 17:17\\]
0 Asynchronous power down for CLKDCOLDO o/p."]
pub type ClkdcoldopwdnzR = crate::BitReader;
#[doc = "Field `CLKDCOLDOPWDNZ` writer - 17:17\\]
0 Asynchronous power down for CLKDCOLDO o/p."]
pub type ClkdcoldopwdnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULOWCLKEN` reader - 18:18\\]
Select CLKOUT source in bypass 0x0: When ADPLLLJ in bypass mode, CLKOUT = CLKINP/(N2+1) 0x1: When ADPLLLJ in bypass mode, CLKOUT = CLKINPULOW."]
pub type UlowclkenR = crate::BitReader;
#[doc = "Field `ULOWCLKEN` writer - 18:18\\]
Select CLKOUT source in bypass 0x0: When ADPLLLJ in bypass mode, CLKOUT = CLKINP/(N2+1) 0x1: When ADPLLLJ in bypass mode, CLKOUT = CLKINPULOW."]
pub type UlowclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUTLDOEN` reader - 19:19\\]
Synchronously enables/disables CLKOUTLDO 0x0 : synchronously disables CLKOUTLDO 0x1 : synchronously enables CLKOUTLDO"]
pub type ClkoutldoenR = crate::BitReader;
#[doc = "Field `CLKOUTLDOEN` writer - 19:19\\]
Synchronously enables/disables CLKOUTLDO 0x0 : synchronously disables CLKOUTLDO 0x1 : synchronously enables CLKOUTLDO"]
pub type ClkoutldoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUTEN` reader - 20:20\\]
CLKOUT enable or disable 0x0 : synchronously disables CLKOUT 0x1 : synchronously enables CLKOUT"]
pub type ClkoutenR = crate::BitReader;
#[doc = "Field `CLKOUTEN` writer - 20:20\\]
CLKOUT enable or disable 0x0 : synchronously disables CLKOUT 0x1 : synchronously enables CLKOUT"]
pub type ClkoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBYRET` reader - 21:21\\]
Standby retention control 0x0 : prepares ADPLLLJ for relock when out of retention by removing the gating on all internal clocks. 0x1 : prepares ADPLLLJ for retention by gating all the internal clocks."]
pub type StbyretR = crate::BitReader;
#[doc = "Field `STBYRET` writer - 21:21\\]
Standby retention control 0x0 : prepares ADPLLLJ for relock when out of retention by removing the gating on all internal clocks. 0x1 : prepares ADPLLLJ for retention by gating all the internal clocks."]
pub type StbyretW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSACKZ` reader - 22:22\\]
BYPASSACKZ is a special purpose input to the module. In general this input is expected to be tied to static low. For the output clocks of the module that do not have an internal bypass mux viz. CLKDCOLDO and CLKOUTLDO, a bypass mux could be implemented external to the module."]
pub type BypassackzR = crate::BitReader;
#[doc = "Field `BYPASSACKZ` writer - 22:22\\]
BYPASSACKZ is a special purpose input to the module. In general this input is expected to be tied to static low. For the output clocks of the module that do not have an internal bypass mux viz. CLKDCOLDO and CLKOUTLDO, a bypass mux could be implemented external to the module."]
pub type BypassackzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - 23:23\\]
Sets PLL to Idle mode 0x0 : When SYSRESET = 0 and TINITZ = 1 IDLE = 0 PLL will go to Active and Locked 0x1 : When SYSRESET = 0 and TINITZ = 1 IDLE = 1 PLL will go to Idle Bypass low powe"]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - 23:23\\]
Sets PLL to Idle mode 0x0 : When SYSRESET = 0 and TINITZ = 1 IDLE = 0 PLL will go to Active and Locked 0x1 : When SYSRESET = 0 and TINITZ = 1 IDLE = 1 PLL will go to Idle Bypass low powe"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWELLTRIM` reader - 28:24\\]
Trim value for the PLL"]
pub type NwelltrimR = crate::FieldReader;
#[doc = "Field `NWELLTRIM` writer - 28:24\\]
Trim value for the PLL"]
pub type NwelltrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLKDCOLDOEN` reader - 29:29\\]
Synchronously enables/disables CLKDCOLDO 0x0 : synchronously disables CLKDCOLDO 0x1 : synchronously enables CLKDCOLDO"]
pub type ClkdcoldoenR = crate::BitReader;
#[doc = "Field `CLKDCOLDOEN` writer - 29:29\\]
Synchronously enables/disables CLKDCOLDO 0x0 : synchronously disables CLKDCOLDO 0x1 : synchronously enables CLKDCOLDO"]
pub type ClkdcoldoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSSC` reader - 30:30\\]
Controls Clock Spreading. SSC is not supported. Should be set to 0x0 to disable clock spreading."]
pub type EnsscR = crate::BitReader;
#[doc = "Field `ENSSC` writer - 30:30\\]
Controls Clock Spreading. SSC is not supported. Should be set to 0x0 to disable clock spreading."]
pub type EnsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYCLESLIPEN` reader - 31:31\\]
FailSafe enable to trigger re-calibration in case CycleSlip occurs between REFCLK and FBCLK."]
pub type CycleslipenR = crate::BitReader;
#[doc = "Field `CYCLESLIPEN` writer - 31:31\\]
FailSafe enable to trigger re-calibration in case CycleSlip occurs between REFCLK and FBCLK."]
pub type CycleslipenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PLL core soft reset"]
    #[inline(always)]
    pub fn tintz(&self) -> TintzR {
        TintzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SSC Type"]
    #[inline(always)]
    pub fn ssctype(&self) -> SsctypeR {
        SsctypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Decides when FREQLOCK asserted 0x0: FREQLOCK asserted when DC frequency error less than 1% 0x1: FREQLOCK asserted when DC frequency error less than 2%"]
    #[inline(always)]
    pub fn relaxed_lock(&self) -> RelaxedLockR {
        RelaxedLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:12 - 12:10\\]
DCO Clock (DCOCLK = CLKINP * \\[M/(N+1)\\]) frequency range selector. 0x0: Reserved 0x2: HS2 : DCOCLK range is from 500 MHz to 1000 MHz 0x3: Reserved 0x4: HS1: DCOCLK range is from 1000 MHz to 2000 MHz 0x5: Reserved"]
    #[inline(always)]
    pub fn selfreqdco(&self) -> SelfreqdcoR {
        SelfreqdcoR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
When in Lossclk/Stbyret 0x0 : Limp mode 0x1 : Stopmode"]
    #[inline(always)]
    pub fn stopmode(&self) -> StopmodeR {
        StopmodeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
M2 divider power down mode 0x0: Asynchronous power down for M2 divider 0x1 : M2 divider is functional"]
    #[inline(always)]
    pub fn m2pwdnz(&self) -> M2pwdnzR {
        M2pwdnzR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0 Asynchronous power down for CLKDCOLDO o/p."]
    #[inline(always)]
    pub fn clkdcoldopwdnz(&self) -> ClkdcoldopwdnzR {
        ClkdcoldopwdnzR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Select CLKOUT source in bypass 0x0: When ADPLLLJ in bypass mode, CLKOUT = CLKINP/(N2+1) 0x1: When ADPLLLJ in bypass mode, CLKOUT = CLKINPULOW."]
    #[inline(always)]
    pub fn ulowclken(&self) -> UlowclkenR {
        UlowclkenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Synchronously enables/disables CLKOUTLDO 0x0 : synchronously disables CLKOUTLDO 0x1 : synchronously enables CLKOUTLDO"]
    #[inline(always)]
    pub fn clkoutldoen(&self) -> ClkoutldoenR {
        ClkoutldoenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
CLKOUT enable or disable 0x0 : synchronously disables CLKOUT 0x1 : synchronously enables CLKOUT"]
    #[inline(always)]
    pub fn clkouten(&self) -> ClkoutenR {
        ClkoutenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Standby retention control 0x0 : prepares ADPLLLJ for relock when out of retention by removing the gating on all internal clocks. 0x1 : prepares ADPLLLJ for retention by gating all the internal clocks."]
    #[inline(always)]
    pub fn stbyret(&self) -> StbyretR {
        StbyretR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
BYPASSACKZ is a special purpose input to the module. In general this input is expected to be tied to static low. For the output clocks of the module that do not have an internal bypass mux viz. CLKDCOLDO and CLKOUTLDO, a bypass mux could be implemented external to the module."]
    #[inline(always)]
    pub fn bypassackz(&self) -> BypassackzR {
        BypassackzR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Sets PLL to Idle mode 0x0 : When SYSRESET = 0 and TINITZ = 1 IDLE = 0 PLL will go to Active and Locked 0x1 : When SYSRESET = 0 and TINITZ = 1 IDLE = 1 PLL will go to Idle Bypass low powe"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Trim value for the PLL"]
    #[inline(always)]
    pub fn nwelltrim(&self) -> NwelltrimR {
        NwelltrimR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Synchronously enables/disables CLKDCOLDO 0x0 : synchronously disables CLKDCOLDO 0x1 : synchronously enables CLKDCOLDO"]
    #[inline(always)]
    pub fn clkdcoldoen(&self) -> ClkdcoldoenR {
        ClkdcoldoenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls Clock Spreading. SSC is not supported. Should be set to 0x0 to disable clock spreading."]
    #[inline(always)]
    pub fn enssc(&self) -> EnsscR {
        EnsscR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
FailSafe enable to trigger re-calibration in case CycleSlip occurs between REFCLK and FBCLK."]
    #[inline(always)]
    pub fn cycleslipen(&self) -> CycleslipenR {
        CycleslipenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PLL core soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn tintz(&mut self) -> TintzW<PllCoreClkctrlSpec> {
        TintzW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SSC Type"]
    #[inline(always)]
    #[must_use]
    pub fn ssctype(&mut self) -> SsctypeW<PllCoreClkctrlSpec> {
        SsctypeW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Decides when FREQLOCK asserted 0x0: FREQLOCK asserted when DC frequency error less than 1% 0x1: FREQLOCK asserted when DC frequency error less than 2%"]
    #[inline(always)]
    #[must_use]
    pub fn relaxed_lock(&mut self) -> RelaxedLockW<PllCoreClkctrlSpec> {
        RelaxedLockW::new(self, 8)
    }
    #[doc = "Bits 10:12 - 12:10\\]
DCO Clock (DCOCLK = CLKINP * \\[M/(N+1)\\]) frequency range selector. 0x0: Reserved 0x2: HS2 : DCOCLK range is from 500 MHz to 1000 MHz 0x3: Reserved 0x4: HS1: DCOCLK range is from 1000 MHz to 2000 MHz 0x5: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn selfreqdco(&mut self) -> SelfreqdcoW<PllCoreClkctrlSpec> {
        SelfreqdcoW::new(self, 10)
    }
    #[doc = "Bit 14 - 14:14\\]
When in Lossclk/Stbyret 0x0 : Limp mode 0x1 : Stopmode"]
    #[inline(always)]
    #[must_use]
    pub fn stopmode(&mut self) -> StopmodeW<PllCoreClkctrlSpec> {
        StopmodeW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
M2 divider power down mode 0x0: Asynchronous power down for M2 divider 0x1 : M2 divider is functional"]
    #[inline(always)]
    #[must_use]
    pub fn m2pwdnz(&mut self) -> M2pwdnzW<PllCoreClkctrlSpec> {
        M2pwdnzW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
0 Asynchronous power down for CLKDCOLDO o/p."]
    #[inline(always)]
    #[must_use]
    pub fn clkdcoldopwdnz(&mut self) -> ClkdcoldopwdnzW<PllCoreClkctrlSpec> {
        ClkdcoldopwdnzW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Select CLKOUT source in bypass 0x0: When ADPLLLJ in bypass mode, CLKOUT = CLKINP/(N2+1) 0x1: When ADPLLLJ in bypass mode, CLKOUT = CLKINPULOW."]
    #[inline(always)]
    #[must_use]
    pub fn ulowclken(&mut self) -> UlowclkenW<PllCoreClkctrlSpec> {
        UlowclkenW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Synchronously enables/disables CLKOUTLDO 0x0 : synchronously disables CLKOUTLDO 0x1 : synchronously enables CLKOUTLDO"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutldoen(&mut self) -> ClkoutldoenW<PllCoreClkctrlSpec> {
        ClkoutldoenW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
CLKOUT enable or disable 0x0 : synchronously disables CLKOUT 0x1 : synchronously enables CLKOUT"]
    #[inline(always)]
    #[must_use]
    pub fn clkouten(&mut self) -> ClkoutenW<PllCoreClkctrlSpec> {
        ClkoutenW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Standby retention control 0x0 : prepares ADPLLLJ for relock when out of retention by removing the gating on all internal clocks. 0x1 : prepares ADPLLLJ for retention by gating all the internal clocks."]
    #[inline(always)]
    #[must_use]
    pub fn stbyret(&mut self) -> StbyretW<PllCoreClkctrlSpec> {
        StbyretW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
BYPASSACKZ is a special purpose input to the module. In general this input is expected to be tied to static low. For the output clocks of the module that do not have an internal bypass mux viz. CLKDCOLDO and CLKOUTLDO, a bypass mux could be implemented external to the module."]
    #[inline(always)]
    #[must_use]
    pub fn bypassackz(&mut self) -> BypassackzW<PllCoreClkctrlSpec> {
        BypassackzW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Sets PLL to Idle mode 0x0 : When SYSRESET = 0 and TINITZ = 1 IDLE = 0 PLL will go to Active and Locked 0x1 : When SYSRESET = 0 and TINITZ = 1 IDLE = 1 PLL will go to Idle Bypass low powe"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<PllCoreClkctrlSpec> {
        IdleW::new(self, 23)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Trim value for the PLL"]
    #[inline(always)]
    #[must_use]
    pub fn nwelltrim(&mut self) -> NwelltrimW<PllCoreClkctrlSpec> {
        NwelltrimW::new(self, 24)
    }
    #[doc = "Bit 29 - 29:29\\]
Synchronously enables/disables CLKDCOLDO 0x0 : synchronously disables CLKDCOLDO 0x1 : synchronously enables CLKDCOLDO"]
    #[inline(always)]
    #[must_use]
    pub fn clkdcoldoen(&mut self) -> ClkdcoldoenW<PllCoreClkctrlSpec> {
        ClkdcoldoenW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls Clock Spreading. SSC is not supported. Should be set to 0x0 to disable clock spreading."]
    #[inline(always)]
    #[must_use]
    pub fn enssc(&mut self) -> EnsscW<PllCoreClkctrlSpec> {
        EnsscW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
FailSafe enable to trigger re-calibration in case CycleSlip occurs between REFCLK and FBCLK."]
    #[inline(always)]
    #[must_use]
    pub fn cycleslipen(&mut self) -> CycleslipenW<PllCoreClkctrlSpec> {
        CycleslipenW::new(self, 31)
    }
}
#[doc = "PLL_CORE_CLKCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCoreClkctrlSpec;
impl crate::RegisterSpec for PllCoreClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_core_clkctrl::R`](R) reader structure"]
impl crate::Readable for PllCoreClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_core_clkctrl::W`](W) writer structure"]
impl crate::Writable for PllCoreClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_CORE_CLKCTRL to value 0"]
impl crate::Resettable for PllCoreClkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
