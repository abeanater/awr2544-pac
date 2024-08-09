#[doc = "Register `ANA_REG_CLK_CTRL_REG1_XO_SLICER` reader"]
pub type R = crate::R<AnaRegClkCtrlReg1XoSlicerSpec>;
#[doc = "Register `ANA_REG_CLK_CTRL_REG1_XO_SLICER` writer"]
pub type W = crate::W<AnaRegClkCtrlReg1XoSlicerSpec>;
#[doc = "Field `RTRIM_BIAS_XO_SLICER` reader - 3:0\\]
Crystal Oscillator and Slicer Bias RTrim Binary-weighted bias control 0x0 = Functional Reset"]
pub type RtrimBiasXoSlicerR = crate::FieldReader;
#[doc = "Field `RTRIM_BIAS_XO_SLICER` writer - 3:0\\]
Crystal Oscillator and Slicer Bias RTrim Binary-weighted bias control 0x0 = Functional Reset"]
pub type RtrimBiasXoSlicerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XOSC_DRIVE_XO_SLICER` reader - 8:4\\]
Crystal Oscillator Output Drive Binary-weighted oscillator drive control 0x0 = Functional Reset"]
pub type XoscDriveXoSlicerR = crate::FieldReader;
#[doc = "Field `XOSC_DRIVE_XO_SLICER` writer - 8:4\\]
Crystal Oscillator Output Drive Binary-weighted oscillator drive control 0x0 = Functional Reset"]
pub type XoscDriveXoSlicerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FASTCHARGEZ_BIAS_XO_SLICER` reader - 9:9\\]
Bias Fast-charge Enable (Active Low) This bit bypasses the RC filtering on the XOSC/SLICER Bias to permit more rapid power-up. 0 = Bias fast-charge 1 = Normal operation (filtering present) 0x1 = Functional Reset"]
pub type FastchargezBiasXoSlicerR = crate::BitReader;
#[doc = "Field `FASTCHARGEZ_BIAS_XO_SLICER` writer - 9:9\\]
Bias Fast-charge Enable (Active Low) This bit bypasses the RC filtering on the XOSC/SLICER Bias to permit more rapid power-up. 0 = Bias fast-charge 1 = Normal operation (filtering present) 0x1 = Functional Reset"]
pub type FastchargezBiasXoSlicerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLICER_HIPWR_XO_SLICER` reader - 10:10\\]
Slicer High-power Mode This bit bypasses the input clock slicer current-starving/filtering circuitry to increase gain and reduce device phase-noise at the expense of power and reduced supply noise rejection. This permits the use of a high-speed external test clock (660MHz max). 0 = Normal operation (current-limiting present) 1 = High-power/high-speed test mode 0x0 = Functional Reset"]
pub type SlicerHipwrXoSlicerR = crate::BitReader;
#[doc = "Field `SLICER_HIPWR_XO_SLICER` writer - 10:10\\]
Slicer High-power Mode This bit bypasses the input clock slicer current-starving/filtering circuitry to increase gain and reduce device phase-noise at the expense of power and reduced supply noise rejection. This permits the use of a high-speed external test clock (660MHz max). 0 = Normal operation (current-limiting present) 1 = High-power/high-speed test mode 0x0 = Functional Reset"]
pub type SlicerHipwrXoSlicerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLICER_DCCPL_XO_SLICER` reader - 11:11\\]
Slicer DC-Coupled Mode 0 = Normal operation (AC-couple CLKP to internal slicer) 1 = DC-couple CLKP to internal slicer to CLKP 0x0 = Functional Reset"]
pub type SlicerDccplXoSlicerR = crate::BitReader;
#[doc = "Field `SLICER_DCCPL_XO_SLICER` writer - 11:11\\]
Slicer DC-Coupled Mode 0 = Normal operation (AC-couple CLKP to internal slicer) 1 = DC-couple CLKP to internal slicer to CLKP 0x0 = Functional Reset"]
pub type SlicerDccplXoSlicerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_DETECT_XO_SLICER` reader - 12:12\\]
XTAL Detect Enable This bit connects a pullup and sense circuitry to CLKM to detect the presence or absence of a crystal. This operation will conflict with oscillator functionality, so this bit must be asserted only when the oscillator is disabled (CTRL_CLKTOP_REG1 bit 2 must be \"0\"). After asserted, the internal XTAL_SENSE signal will reflect a \"1\" if a crystal is present (CLKM sees a high impedance) or \"0\" if CLKM is tied to ground. After the sense operation is detected, this bit must be cleared before the oscillator will function properly if enabled. 0 = Normal operation (pullup and sense circuitry are disconnected from CLKM, XTAL_SENSE outputs \"1\") 1 = XTAL sense function enabled (pullup and sense circuitry connected to CLKM, output of XTAL_SENSE reads \"1\" if high impedance, \"0\" if CLKM is tied to ground) 0x0 = Functional Reset"]
pub type XtalDetectXoSlicerR = crate::BitReader;
#[doc = "Field `XTAL_DETECT_XO_SLICER` writer - 12:12\\]
XTAL Detect Enable This bit connects a pullup and sense circuitry to CLKM to detect the presence or absence of a crystal. This operation will conflict with oscillator functionality, so this bit must be asserted only when the oscillator is disabled (CTRL_CLKTOP_REG1 bit 2 must be \"0\"). After asserted, the internal XTAL_SENSE signal will reflect a \"1\" if a crystal is present (CLKM sees a high impedance) or \"0\" if CLKM is tied to ground. After the sense operation is detected, this bit must be cleared before the oscillator will function properly if enabled. 0 = Normal operation (pullup and sense circuitry are disconnected from CLKM, XTAL_SENSE outputs \"1\") 1 = XTAL sense function enabled (pullup and sense circuitry connected to CLKM, output of XTAL_SENSE reads \"1\" if high impedance, \"0\" if CLKM is tied to ground) 0x0 = Functional Reset"]
pub type XtalDetectXoSlicerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLICER_APLL_BYPASS` reader - 13:13\\]
Slicer APLL Bypass This bit enables a high-speed slicer connected to CLKM which can be used to drive a high-speed clock directly as the SYNTH reference clock. 0 = Normal operation (bypass slicer disabled) 1 = APLL Bypass Slicer Enabled 0x0 = Functional Reset"]
pub type SlicerApllBypassR = crate::BitReader;
#[doc = "Field `SLICER_APLL_BYPASS` writer - 13:13\\]
Slicer APLL Bypass This bit enables a high-speed slicer connected to CLKM which can be used to drive a high-speed clock directly as the SYNTH reference clock. 0 = Normal operation (bypass slicer disabled) 1 = APLL Bypass Slicer Enabled 0x0 = Functional Reset"]
pub type SlicerApllBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLICER_APLL_BYPASS_DRV` reader - 14:14\\]
Slicer APLL Bypass Drive This bit controls the drive strength of the APLL Bypass Slicer 0 = Low-power drive 1 = High-power drive 0x0 = Functional Reset"]
pub type SlicerApllBypassDrvR = crate::BitReader;
#[doc = "Field `SLICER_APLL_BYPASS_DRV` writer - 14:14\\]
Slicer APLL Bypass Drive This bit controls the drive strength of the APLL Bypass Slicer 0 = Low-power drive 1 = High-power drive 0x0 = Functional Reset"]
pub type SlicerApllBypassDrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED0` reader - 31:15\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 31:15\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Crystal Oscillator and Slicer Bias RTrim Binary-weighted bias control 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn rtrim_bias_xo_slicer(&self) -> RtrimBiasXoSlicerR {
        RtrimBiasXoSlicerR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Crystal Oscillator Output Drive Binary-weighted oscillator drive control 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn xosc_drive_xo_slicer(&self) -> XoscDriveXoSlicerR {
        XoscDriveXoSlicerR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Bias Fast-charge Enable (Active Low) This bit bypasses the RC filtering on the XOSC/SLICER Bias to permit more rapid power-up. 0 = Bias fast-charge 1 = Normal operation (filtering present) 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn fastchargez_bias_xo_slicer(&self) -> FastchargezBiasXoSlicerR {
        FastchargezBiasXoSlicerR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Slicer High-power Mode This bit bypasses the input clock slicer current-starving/filtering circuitry to increase gain and reduce device phase-noise at the expense of power and reduced supply noise rejection. This permits the use of a high-speed external test clock (660MHz max). 0 = Normal operation (current-limiting present) 1 = High-power/high-speed test mode 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn slicer_hipwr_xo_slicer(&self) -> SlicerHipwrXoSlicerR {
        SlicerHipwrXoSlicerR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Slicer DC-Coupled Mode 0 = Normal operation (AC-couple CLKP to internal slicer) 1 = DC-couple CLKP to internal slicer to CLKP 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn slicer_dccpl_xo_slicer(&self) -> SlicerDccplXoSlicerR {
        SlicerDccplXoSlicerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
XTAL Detect Enable This bit connects a pullup and sense circuitry to CLKM to detect the presence or absence of a crystal. This operation will conflict with oscillator functionality, so this bit must be asserted only when the oscillator is disabled (CTRL_CLKTOP_REG1 bit 2 must be \"0\"). After asserted, the internal XTAL_SENSE signal will reflect a \"1\" if a crystal is present (CLKM sees a high impedance) or \"0\" if CLKM is tied to ground. After the sense operation is detected, this bit must be cleared before the oscillator will function properly if enabled. 0 = Normal operation (pullup and sense circuitry are disconnected from CLKM, XTAL_SENSE outputs \"1\") 1 = XTAL sense function enabled (pullup and sense circuitry connected to CLKM, output of XTAL_SENSE reads \"1\" if high impedance, \"0\" if CLKM is tied to ground) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn xtal_detect_xo_slicer(&self) -> XtalDetectXoSlicerR {
        XtalDetectXoSlicerR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Slicer APLL Bypass This bit enables a high-speed slicer connected to CLKM which can be used to drive a high-speed clock directly as the SYNTH reference clock. 0 = Normal operation (bypass slicer disabled) 1 = APLL Bypass Slicer Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn slicer_apll_bypass(&self) -> SlicerApllBypassR {
        SlicerApllBypassR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Slicer APLL Bypass Drive This bit controls the drive strength of the APLL Bypass Slicer 0 = Low-power drive 1 = High-power drive 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn slicer_apll_bypass_drv(&self) -> SlicerApllBypassDrvR {
        SlicerApllBypassDrvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Crystal Oscillator and Slicer Bias RTrim Binary-weighted bias control 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtrim_bias_xo_slicer(&mut self) -> RtrimBiasXoSlicerW<AnaRegClkCtrlReg1XoSlicerSpec> {
        RtrimBiasXoSlicerW::new(self, 0)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Crystal Oscillator Output Drive Binary-weighted oscillator drive control 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_drive_xo_slicer(&mut self) -> XoscDriveXoSlicerW<AnaRegClkCtrlReg1XoSlicerSpec> {
        XoscDriveXoSlicerW::new(self, 4)
    }
    #[doc = "Bit 9 - 9:9\\]
Bias Fast-charge Enable (Active Low) This bit bypasses the RC filtering on the XOSC/SLICER Bias to permit more rapid power-up. 0 = Bias fast-charge 1 = Normal operation (filtering present) 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fastchargez_bias_xo_slicer(
        &mut self,
    ) -> FastchargezBiasXoSlicerW<AnaRegClkCtrlReg1XoSlicerSpec> {
        FastchargezBiasXoSlicerW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Slicer High-power Mode This bit bypasses the input clock slicer current-starving/filtering circuitry to increase gain and reduce device phase-noise at the expense of power and reduced supply noise rejection. This permits the use of a high-speed external test clock (660MHz max). 0 = Normal operation (current-limiting present) 1 = High-power/high-speed test mode 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn slicer_hipwr_xo_slicer(
        &mut self,
    ) -> SlicerHipwrXoSlicerW<AnaRegClkCtrlReg1XoSlicerSpec> {
        SlicerHipwrXoSlicerW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Slicer DC-Coupled Mode 0 = Normal operation (AC-couple CLKP to internal slicer) 1 = DC-couple CLKP to internal slicer to CLKP 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn slicer_dccpl_xo_slicer(
        &mut self,
    ) -> SlicerDccplXoSlicerW<AnaRegClkCtrlReg1XoSlicerSpec> {
        SlicerDccplXoSlicerW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
XTAL Detect Enable This bit connects a pullup and sense circuitry to CLKM to detect the presence or absence of a crystal. This operation will conflict with oscillator functionality, so this bit must be asserted only when the oscillator is disabled (CTRL_CLKTOP_REG1 bit 2 must be \"0\"). After asserted, the internal XTAL_SENSE signal will reflect a \"1\" if a crystal is present (CLKM sees a high impedance) or \"0\" if CLKM is tied to ground. After the sense operation is detected, this bit must be cleared before the oscillator will function properly if enabled. 0 = Normal operation (pullup and sense circuitry are disconnected from CLKM, XTAL_SENSE outputs \"1\") 1 = XTAL sense function enabled (pullup and sense circuitry connected to CLKM, output of XTAL_SENSE reads \"1\" if high impedance, \"0\" if CLKM is tied to ground) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_detect_xo_slicer(&mut self) -> XtalDetectXoSlicerW<AnaRegClkCtrlReg1XoSlicerSpec> {
        XtalDetectXoSlicerW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Slicer APLL Bypass This bit enables a high-speed slicer connected to CLKM which can be used to drive a high-speed clock directly as the SYNTH reference clock. 0 = Normal operation (bypass slicer disabled) 1 = APLL Bypass Slicer Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn slicer_apll_bypass(&mut self) -> SlicerApllBypassW<AnaRegClkCtrlReg1XoSlicerSpec> {
        SlicerApllBypassW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Slicer APLL Bypass Drive This bit controls the drive strength of the APLL Bypass Slicer 0 = Low-power drive 1 = High-power drive 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn slicer_apll_bypass_drv(
        &mut self,
    ) -> SlicerApllBypassDrvW<AnaRegClkCtrlReg1XoSlicerSpec> {
        SlicerApllBypassDrvW::new(self, 14)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegClkCtrlReg1XoSlicerSpec> {
        Reserved0W::new(self, 15)
    }
}
#[doc = "ANA_REG_CLK_CTRL_REG1_XO_SLICER\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg1_xo_slicer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg1_xo_slicer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegClkCtrlReg1XoSlicerSpec;
impl crate::RegisterSpec for AnaRegClkCtrlReg1XoSlicerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_clk_ctrl_reg1_xo_slicer::R`](R) reader structure"]
impl crate::Readable for AnaRegClkCtrlReg1XoSlicerSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_clk_ctrl_reg1_xo_slicer::W`](W) writer structure"]
impl crate::Writable for AnaRegClkCtrlReg1XoSlicerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_CLK_CTRL_REG1_XO_SLICER to value 0"]
impl crate::Resettable for AnaRegClkCtrlReg1XoSlicerSpec {
    const RESET_VALUE: u32 = 0;
}
