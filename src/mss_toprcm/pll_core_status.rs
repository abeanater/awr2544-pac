#[doc = "Register `PLL_CORE_STATUS` reader"]
pub type R = crate::R<PllCoreStatusSpec>;
#[doc = "Register `PLL_CORE_STATUS` writer"]
pub type W = crate::W<PllCoreStatusSpec>;
#[doc = "Field `BYPASS` reader - 0:0\\]
Bypass status signal. 1 CLKOUT in bypass"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - 0:0\\]
Bypass status signal. 1 CLKOUT in bypass"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHJITTER` reader - 1:1\\]
1 indicates jitter. After PHASELOCK is asserted high, the HIGHJITTER flag is asserted high if phase error between REFCLK and FBCLK greater than 24%."]
pub type HighjitterR = crate::BitReader;
#[doc = "Field `HIGHJITTER` writer - 1:1\\]
1 indicates jitter. After PHASELOCK is asserted high, the HIGHJITTER flag is asserted high if phase error between REFCLK and FBCLK greater than 24%."]
pub type HighjitterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSCACK` reader - 2:2\\]
Spread Spectrum status 0x0 : Spread-spectrum Clocking is disabled on output clocks 0x1 : Spread-spectrum Clocking is enabled on output clocks"]
pub type SscackR = crate::BitReader;
#[doc = "Field `SSCACK` writer - 2:2\\]
Spread Spectrum status 0x0 : Spread-spectrum Clocking is disabled on output clocks 0x1 : Spread-spectrum Clocking is enabled on output clocks"]
pub type SscackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M2CHANGEACK` reader - 3:3\\]
Acknowledge for change to M2 divider. Toggles from 1-0 or 0-1 (depending on current value) once CLKOUT frequency change has completed."]
pub type M2changeackR = crate::BitReader;
#[doc = "Field `M2CHANGEACK` writer - 3:3\\]
Acknowledge for change to M2 divider. Toggles from 1-0 or 0-1 (depending on current value) once CLKOUT frequency change has completed."]
pub type M2changeackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK2` reader - 4:4\\]
ADPLL internal loop lock status"]
pub type Lock2R = crate::BitReader;
#[doc = "Field `LOCK2` writer - 4:4\\]
ADPLL internal loop lock status"]
pub type Lock2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUTENACK` reader - 5:5\\]
Indicates the enable/disable condition of CLKOUTEN 0x0 = CLKOUT gating completed 0x1 = CLKOUT enabling completed"]
pub type ClkoutenackR = crate::BitReader;
#[doc = "Field `CLKOUTENACK` writer - 5:5\\]
Indicates the enable/disable condition of CLKOUTEN 0x0 = CLKOUT gating completed 0x1 = CLKOUT enabling completed"]
pub type ClkoutenackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOSSREF` reader - 6:6\\]
Reference input loss"]
pub type LossrefR = crate::BitReader;
#[doc = "Field `LOSSREF` writer - 6:6\\]
Reference input loss"]
pub type LossrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBYRETACK` reader - 7:7\\]
Standby and retention status 0x0: indicates to SOC that all internal clocks in ADPLLLJ are active and it is starting the relock process. 0x1: indicates to SOC that all internal clocks in ADPLLLJ are gated and it is ready for retention."]
pub type StbyretackR = crate::BitReader;
#[doc = "Field `STBYRETACK` writer - 7:7\\]
Standby and retention status 0x0: indicates to SOC that all internal clocks in ADPLLLJ are active and it is starting the relock process. 0x1: indicates to SOC that all internal clocks in ADPLLLJ are gated and it is ready for retention."]
pub type StbyretackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSACK` reader - 8:8\\]
Status of BYPASSACK output pin"]
pub type BypassackR = crate::BitReader;
#[doc = "Field `BYPASSACK` writer - 8:8\\]
Status of BYPASSACK output pin"]
pub type BypassackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQLOCK` reader - 9:9\\]
Status on FREQLOCK output pin"]
pub type FreqlockR = crate::BitReader;
#[doc = "Field `FREQLOCK` writer - 9:9\\]
Status on FREQLOCK output pin"]
pub type FreqlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHASELOCK` reader - 10:10\\]
Status on PHASELOCK output pin"]
pub type PhaselockR = crate::BitReader;
#[doc = "Field `PHASELOCK` writer - 10:10\\]
Status on PHASELOCK output pin"]
pub type PhaselockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDCOLDOACK` reader - 11:11\\]
Status on PHASELOCK output pin"]
pub type ClkdcoldoackR = crate::BitReader;
#[doc = "Field `CLKDCOLDOACK` writer - 11:11\\]
Status on PHASELOCK output pin"]
pub type ClkdcoldoackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECAL_OPPIN` reader - 27:27\\]
Recalibration status flag. 1 ADPLLLJ requires recalibration"]
pub type RecalOppinR = crate::BitReader;
#[doc = "Field `RECAL_OPPIN` writer - 27:27\\]
Recalibration status flag. 1 ADPLLLJ requires recalibration"]
pub type RecalOppinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECAL_BSTATUS3` reader - 28:28\\]
Recalibration status flag. 1 ADPLLLJ requires recalibration"]
pub type RecalBstatus3R = crate::BitReader;
#[doc = "Field `RECAL_BSTATUS3` writer - 28:28\\]
Recalibration status flag. 1 ADPLLLJ requires recalibration"]
pub type RecalBstatus3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOPWDN` reader - 29:29\\]
1 indicates ADPLLLJ internal LDO is power down. VDDLDOOUT will be un-defined in this condition"]
pub type LdopwdnR = crate::BitReader;
#[doc = "Field `LDOPWDN` writer - 29:29\\]
1 indicates ADPLLLJ internal LDO is power down. VDDLDOOUT will be un-defined in this condition"]
pub type LdopwdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGOODOUT` reader - 30:30\\]
Status of the strong power-switch 0x0 : indicates the/OFF status of the strong power-switch in digital to SOC. 0x1 : ndicates the ON status of the strong power-switch in digital to SOC."]
pub type PgoodoutR = crate::BitReader;
#[doc = "Field `PGOODOUT` writer - 30:30\\]
Status of the strong power-switch 0x0 : indicates the/OFF status of the strong power-switch in digital to SOC. 0x1 : ndicates the ON status of the strong power-switch in digital to SOC."]
pub type PgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PONOUT` reader - 31:31\\]
Status of the weak power-switch 0x0 : indicates the/OFF status of the weak power-switch in digital to SOC. 0x1 : ndicates the ON status of the weak power-switch in digital to SOC."]
pub type PonoutR = crate::BitReader;
#[doc = "Field `PONOUT` writer - 31:31\\]
Status of the weak power-switch 0x0 : indicates the/OFF status of the weak power-switch in digital to SOC. 0x1 : ndicates the ON status of the weak power-switch in digital to SOC."]
pub type PonoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Bypass status signal. 1 CLKOUT in bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 indicates jitter. After PHASELOCK is asserted high, the HIGHJITTER flag is asserted high if phase error between REFCLK and FBCLK greater than 24%."]
    #[inline(always)]
    pub fn highjitter(&self) -> HighjitterR {
        HighjitterR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Spread Spectrum status 0x0 : Spread-spectrum Clocking is disabled on output clocks 0x1 : Spread-spectrum Clocking is enabled on output clocks"]
    #[inline(always)]
    pub fn sscack(&self) -> SscackR {
        SscackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Acknowledge for change to M2 divider. Toggles from 1-0 or 0-1 (depending on current value) once CLKOUT frequency change has completed."]
    #[inline(always)]
    pub fn m2changeack(&self) -> M2changeackR {
        M2changeackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ADPLL internal loop lock status"]
    #[inline(always)]
    pub fn lock2(&self) -> Lock2R {
        Lock2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates the enable/disable condition of CLKOUTEN 0x0 = CLKOUT gating completed 0x1 = CLKOUT enabling completed"]
    #[inline(always)]
    pub fn clkoutenack(&self) -> ClkoutenackR {
        ClkoutenackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Reference input loss"]
    #[inline(always)]
    pub fn lossref(&self) -> LossrefR {
        LossrefR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Standby and retention status 0x0: indicates to SOC that all internal clocks in ADPLLLJ are active and it is starting the relock process. 0x1: indicates to SOC that all internal clocks in ADPLLLJ are gated and it is ready for retention."]
    #[inline(always)]
    pub fn stbyretack(&self) -> StbyretackR {
        StbyretackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of BYPASSACK output pin"]
    #[inline(always)]
    pub fn bypassack(&self) -> BypassackR {
        BypassackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Status on FREQLOCK output pin"]
    #[inline(always)]
    pub fn freqlock(&self) -> FreqlockR {
        FreqlockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Status on PHASELOCK output pin"]
    #[inline(always)]
    pub fn phaselock(&self) -> PhaselockR {
        PhaselockR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Status on PHASELOCK output pin"]
    #[inline(always)]
    pub fn clkdcoldoack(&self) -> ClkdcoldoackR {
        ClkdcoldoackR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Recalibration status flag. 1 ADPLLLJ requires recalibration"]
    #[inline(always)]
    pub fn recal_oppin(&self) -> RecalOppinR {
        RecalOppinR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Recalibration status flag. 1 ADPLLLJ requires recalibration"]
    #[inline(always)]
    pub fn recal_bstatus3(&self) -> RecalBstatus3R {
        RecalBstatus3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
1 indicates ADPLLLJ internal LDO is power down. VDDLDOOUT will be un-defined in this condition"]
    #[inline(always)]
    pub fn ldopwdn(&self) -> LdopwdnR {
        LdopwdnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Status of the strong power-switch 0x0 : indicates the/OFF status of the strong power-switch in digital to SOC. 0x1 : ndicates the ON status of the strong power-switch in digital to SOC."]
    #[inline(always)]
    pub fn pgoodout(&self) -> PgoodoutR {
        PgoodoutR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Status of the weak power-switch 0x0 : indicates the/OFF status of the weak power-switch in digital to SOC. 0x1 : ndicates the ON status of the weak power-switch in digital to SOC."]
    #[inline(always)]
    pub fn ponout(&self) -> PonoutR {
        PonoutR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Bypass status signal. 1 CLKOUT in bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<PllCoreStatusSpec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1 indicates jitter. After PHASELOCK is asserted high, the HIGHJITTER flag is asserted high if phase error between REFCLK and FBCLK greater than 24%."]
    #[inline(always)]
    #[must_use]
    pub fn highjitter(&mut self) -> HighjitterW<PllCoreStatusSpec> {
        HighjitterW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Spread Spectrum status 0x0 : Spread-spectrum Clocking is disabled on output clocks 0x1 : Spread-spectrum Clocking is enabled on output clocks"]
    #[inline(always)]
    #[must_use]
    pub fn sscack(&mut self) -> SscackW<PllCoreStatusSpec> {
        SscackW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Acknowledge for change to M2 divider. Toggles from 1-0 or 0-1 (depending on current value) once CLKOUT frequency change has completed."]
    #[inline(always)]
    #[must_use]
    pub fn m2changeack(&mut self) -> M2changeackW<PllCoreStatusSpec> {
        M2changeackW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
ADPLL internal loop lock status"]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> Lock2W<PllCoreStatusSpec> {
        Lock2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates the enable/disable condition of CLKOUTEN 0x0 = CLKOUT gating completed 0x1 = CLKOUT enabling completed"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutenack(&mut self) -> ClkoutenackW<PllCoreStatusSpec> {
        ClkoutenackW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Reference input loss"]
    #[inline(always)]
    #[must_use]
    pub fn lossref(&mut self) -> LossrefW<PllCoreStatusSpec> {
        LossrefW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Standby and retention status 0x0: indicates to SOC that all internal clocks in ADPLLLJ are active and it is starting the relock process. 0x1: indicates to SOC that all internal clocks in ADPLLLJ are gated and it is ready for retention."]
    #[inline(always)]
    #[must_use]
    pub fn stbyretack(&mut self) -> StbyretackW<PllCoreStatusSpec> {
        StbyretackW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of BYPASSACK output pin"]
    #[inline(always)]
    #[must_use]
    pub fn bypassack(&mut self) -> BypassackW<PllCoreStatusSpec> {
        BypassackW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Status on FREQLOCK output pin"]
    #[inline(always)]
    #[must_use]
    pub fn freqlock(&mut self) -> FreqlockW<PllCoreStatusSpec> {
        FreqlockW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Status on PHASELOCK output pin"]
    #[inline(always)]
    #[must_use]
    pub fn phaselock(&mut self) -> PhaselockW<PllCoreStatusSpec> {
        PhaselockW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Status on PHASELOCK output pin"]
    #[inline(always)]
    #[must_use]
    pub fn clkdcoldoack(&mut self) -> ClkdcoldoackW<PllCoreStatusSpec> {
        ClkdcoldoackW::new(self, 11)
    }
    #[doc = "Bit 27 - 27:27\\]
Recalibration status flag. 1 ADPLLLJ requires recalibration"]
    #[inline(always)]
    #[must_use]
    pub fn recal_oppin(&mut self) -> RecalOppinW<PllCoreStatusSpec> {
        RecalOppinW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Recalibration status flag. 1 ADPLLLJ requires recalibration"]
    #[inline(always)]
    #[must_use]
    pub fn recal_bstatus3(&mut self) -> RecalBstatus3W<PllCoreStatusSpec> {
        RecalBstatus3W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
1 indicates ADPLLLJ internal LDO is power down. VDDLDOOUT will be un-defined in this condition"]
    #[inline(always)]
    #[must_use]
    pub fn ldopwdn(&mut self) -> LdopwdnW<PllCoreStatusSpec> {
        LdopwdnW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Status of the strong power-switch 0x0 : indicates the/OFF status of the strong power-switch in digital to SOC. 0x1 : ndicates the ON status of the strong power-switch in digital to SOC."]
    #[inline(always)]
    #[must_use]
    pub fn pgoodout(&mut self) -> PgoodoutW<PllCoreStatusSpec> {
        PgoodoutW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Status of the weak power-switch 0x0 : indicates the/OFF status of the weak power-switch in digital to SOC. 0x1 : ndicates the ON status of the weak power-switch in digital to SOC."]
    #[inline(always)]
    #[must_use]
    pub fn ponout(&mut self) -> PonoutW<PllCoreStatusSpec> {
        PonoutW::new(self, 31)
    }
}
#[doc = "PLL_CORE_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCoreStatusSpec;
impl crate::RegisterSpec for PllCoreStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_core_status::R`](R) reader structure"]
impl crate::Readable for PllCoreStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_core_status::W`](W) writer structure"]
impl crate::Writable for PllCoreStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_CORE_STATUS to value 0"]
impl crate::Resettable for PllCoreStatusSpec {
    const RESET_VALUE: u32 = 0;
}
