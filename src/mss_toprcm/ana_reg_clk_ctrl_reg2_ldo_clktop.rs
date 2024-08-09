#[doc = "Register `ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP` reader"]
pub type R = crate::R<AnaRegClkCtrlReg2LdoClktopSpec>;
#[doc = "Register `ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP` writer"]
pub type W = crate::W<AnaRegClkCtrlReg2LdoClktopSpec>;
#[doc = "Field `LDO_VOUT_CTRL` reader - 3:0\\]
SLICER LDO VOUT TRIM NEEDS updated description Need inverters on 0,1,2 0x0 = Functional Reset"]
pub type LdoVoutCtrlR = crate::FieldReader;
#[doc = "Field `LDO_VOUT_CTRL` writer - 3:0\\]
SLICER LDO VOUT TRIM NEEDS updated description Need inverters on 0,1,2 0x0 = Functional Reset"]
pub type LdoVoutCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENZ_LOW_BW_CAP` reader - 4:4\\]
SLICER LDO LOW BW MODE DISABLE 1 = Slicer LDO Low BW mode Enabled 0 = Slicer LDO Low BW mode Disabled Description IS updated above Need inverter 0x1 = Functional Reset"]
pub type EnzLowBwCapR = crate::BitReader;
#[doc = "Field `ENZ_LOW_BW_CAP` writer - 4:4\\]
SLICER LDO LOW BW MODE DISABLE 1 = Slicer LDO Low BW mode Enabled 0 = Slicer LDO Low BW mode Disabled Description IS updated above Need inverter 0x1 = Functional Reset"]
pub type EnzLowBwCapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_TEST_MODE` reader - 5:5\\]
SLICER LDO TEST MODE ENABLE 0 = Slicer LDO TEST MODE Disabled 1 = Slicer LDO TEST MODE Enabled 0x0 = Functional Reset"]
pub type EnTestModeR = crate::BitReader;
#[doc = "Field `EN_TEST_MODE` writer - 5:5\\]
SLICER LDO TEST MODE ENABLE 0 = Slicer LDO TEST MODE Disabled 1 = Slicer LDO TEST MODE Enabled 0x0 = Functional Reset"]
pub type EnTestModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_SHRT_CKT` reader - 6:6\\]
SLICER LDO SHORT CKT PROTECTION ENABLE 0 = Slicer LDO Short Ckt Protection Disabled 1 = Slicer LDO Short Ckt Protection Enabled 0x0= Functional Reset"]
pub type EnShrtCktR = crate::BitReader;
#[doc = "Field `EN_SHRT_CKT` writer - 6:6\\]
SLICER LDO SHORT CKT PROTECTION ENABLE 0 = Slicer LDO Short Ckt Protection Disabled 1 = Slicer LDO Short Ckt Protection Enabled 0x0= Functional Reset"]
pub type EnShrtCktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_BYPASS` reader - 7:7\\]
SLICER LDO BYPASS ENABLE 0 = Slicer LDO in normal mode 1 = Slicer LDO Bypassed with external voltage 0x0 = Functional Reset"]
pub type EnBypassR = crate::BitReader;
#[doc = "Field `EN_BYPASS` writer - 7:7\\]
SLICER LDO BYPASS ENABLE 0 = Slicer LDO in normal mode 1 = Slicer LDO Bypassed with external voltage 0x0 = Functional Reset"]
pub type EnBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDO_BW_CTRL` reader - 10:8\\]
SLICER LDO BANDWIDTH CONTROL Need inverters on bits 8 and 10 need updated description 0x7 = Functional Reset"]
pub type LdoBwCtrlR = crate::FieldReader;
#[doc = "Field `LDO_BW_CTRL` writer - 10:8\\]
SLICER LDO BANDWIDTH CONTROL Need inverters on bits 8 and 10 need updated description 0x7 = Functional Reset"]
pub type LdoBwCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCPRT_IBIAS_CTRL` reader - 11:11\\]
SLICER LDO SHORT CKT PROTECTION IBIAS CONTROL 0 = Nominal short circuit bias with nominal short circuit current limit 1 = 2X Nominal short circuit bias with higher short circuit current limit 0x0 = Functional Reset"]
pub type ScprtIbiasCtrlR = crate::BitReader;
#[doc = "Field `SCPRT_IBIAS_CTRL` writer - 11:11\\]
SLICER LDO SHORT CKT PROTECTION IBIAS CONTROL 0 = Nominal short circuit bias with nominal short circuit current limit 1 = 2X Nominal short circuit bias with higher short circuit current limit 0x0 = Functional Reset"]
pub type ScprtIbiasCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_PMOS_PULLDOWN` reader - 12:12\\]
SLICER LDO PMOS PULL DOWN ENABLE 0 = Slicer LDO PMOS Pull Down disabled 1 = Slicer LDO PMOS Pull Down enabled 0x0 = Functional Reset"]
pub type EnablePmosPulldownR = crate::BitReader;
#[doc = "Field `ENABLE_PMOS_PULLDOWN` writer - 12:12\\]
SLICER LDO PMOS PULL DOWN ENABLE 0 = Slicer LDO PMOS Pull Down disabled 1 = Slicer LDO PMOS Pull Down enabled 0x0 = Functional Reset"]
pub type EnablePmosPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLOAD_CTRL` reader - 15:13\\]
SLICER LDO TLOAD CONTROL Need inverter on bit13 updated description needed 0x0 = Functional Reset"]
pub type TloadCtrlR = crate::FieldReader;
#[doc = "Field `TLOAD_CTRL` writer - 15:13\\]
SLICER LDO TLOAD CONTROL Need inverter on bit13 updated description needed 0x0 = Functional Reset"]
pub type TloadCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TESTMUX_CTRL` reader - 19:16\\]
SLICER LDO TEST MUX CONTROL (ONE HOT) Analog MUX enables to test output port 0000 = HI-Z Output 0001 = 0.6 * VLDO_OUT 0010 = VDD18*0.5 = 0.9V 0100 = VSSA 1000 = LDO Test Current (12.5uA) WARNING: Enabling more than one bit may damage the device 0x0 = Functional Reset"]
pub type TestmuxCtrlR = crate::FieldReader;
#[doc = "Field `TESTMUX_CTRL` writer - 19:16\\]
SLICER LDO TEST MUX CONTROL (ONE HOT) Analog MUX enables to test output port 0000 = HI-Z Output 0001 = 0.6 * VLDO_OUT 0010 = VDD18*0.5 = 0.9V 0100 = VSSA 1000 = LDO Test Current (12.5uA) WARNING: Enabling more than one bit may damage the device 0x0 = Functional Reset"]
pub type TestmuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BISTMUX_CTRL` reader - 23:20\\]
SLICER LDO BIST MUX CONTROL (ONE HOT) Analog MUX enables to BIST output port 0000 = HI-Z Output 0001 = VBG_0P9*10/9 =1.0 V 0010 = VDD18*0.5 = 0.9V 0100 = VLDO Output * 0.6 1000 = Floating WARNING: Enabling more than one bit may damage the device 0x4 = Functional Reset"]
pub type BistmuxCtrlR = crate::FieldReader;
#[doc = "Field `BISTMUX_CTRL` writer - 23:20\\]
SLICER LDO BIST MUX CONTROL (ONE HOT) Analog MUX enables to BIST output port 0000 = HI-Z Output 0001 = VBG_0P9*10/9 =1.0 V 0010 = VDD18*0.5 = 0.9V 0100 = VLDO Output * 0.6 1000 = Floating WARNING: Enabling more than one bit may damage the device 0x4 = Functional Reset"]
pub type BistmuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED0` reader - 31:24\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 31:24\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
SLICER LDO VOUT TRIM NEEDS updated description Need inverters on 0,1,2 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn ldo_vout_ctrl(&self) -> LdoVoutCtrlR {
        LdoVoutCtrlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
SLICER LDO LOW BW MODE DISABLE 1 = Slicer LDO Low BW mode Enabled 0 = Slicer LDO Low BW mode Disabled Description IS updated above Need inverter 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn enz_low_bw_cap(&self) -> EnzLowBwCapR {
        EnzLowBwCapR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
SLICER LDO TEST MODE ENABLE 0 = Slicer LDO TEST MODE Disabled 1 = Slicer LDO TEST MODE Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn en_test_mode(&self) -> EnTestModeR {
        EnTestModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SLICER LDO SHORT CKT PROTECTION ENABLE 0 = Slicer LDO Short Ckt Protection Disabled 1 = Slicer LDO Short Ckt Protection Enabled 0x0= Functional Reset"]
    #[inline(always)]
    pub fn en_shrt_ckt(&self) -> EnShrtCktR {
        EnShrtCktR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SLICER LDO BYPASS ENABLE 0 = Slicer LDO in normal mode 1 = Slicer LDO Bypassed with external voltage 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn en_bypass(&self) -> EnBypassR {
        EnBypassR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
SLICER LDO BANDWIDTH CONTROL Need inverters on bits 8 and 10 need updated description 0x7 = Functional Reset"]
    #[inline(always)]
    pub fn ldo_bw_ctrl(&self) -> LdoBwCtrlR {
        LdoBwCtrlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
SLICER LDO SHORT CKT PROTECTION IBIAS CONTROL 0 = Nominal short circuit bias with nominal short circuit current limit 1 = 2X Nominal short circuit bias with higher short circuit current limit 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scprt_ibias_ctrl(&self) -> ScprtIbiasCtrlR {
        ScprtIbiasCtrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
SLICER LDO PMOS PULL DOWN ENABLE 0 = Slicer LDO PMOS Pull Down disabled 1 = Slicer LDO PMOS Pull Down enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn enable_pmos_pulldown(&self) -> EnablePmosPulldownR {
        EnablePmosPulldownR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
SLICER LDO TLOAD CONTROL Need inverter on bit13 updated description needed 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn tload_ctrl(&self) -> TloadCtrlR {
        TloadCtrlR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
SLICER LDO TEST MUX CONTROL (ONE HOT) Analog MUX enables to test output port 0000 = HI-Z Output 0001 = 0.6 * VLDO_OUT 0010 = VDD18*0.5 = 0.9V 0100 = VSSA 1000 = LDO Test Current (12.5uA) WARNING: Enabling more than one bit may damage the device 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn testmux_ctrl(&self) -> TestmuxCtrlR {
        TestmuxCtrlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
SLICER LDO BIST MUX CONTROL (ONE HOT) Analog MUX enables to BIST output port 0000 = HI-Z Output 0001 = VBG_0P9*10/9 =1.0 V 0010 = VDD18*0.5 = 0.9V 0100 = VLDO Output * 0.6 1000 = Floating WARNING: Enabling more than one bit may damage the device 0x4 = Functional Reset"]
    #[inline(always)]
    pub fn bistmux_ctrl(&self) -> BistmuxCtrlR {
        BistmuxCtrlR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
SLICER LDO VOUT TRIM NEEDS updated description Need inverters on 0,1,2 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ldo_vout_ctrl(&mut self) -> LdoVoutCtrlW<AnaRegClkCtrlReg2LdoClktopSpec> {
        LdoVoutCtrlW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SLICER LDO LOW BW MODE DISABLE 1 = Slicer LDO Low BW mode Enabled 0 = Slicer LDO Low BW mode Disabled Description IS updated above Need inverter 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn enz_low_bw_cap(&mut self) -> EnzLowBwCapW<AnaRegClkCtrlReg2LdoClktopSpec> {
        EnzLowBwCapW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
SLICER LDO TEST MODE ENABLE 0 = Slicer LDO TEST MODE Disabled 1 = Slicer LDO TEST MODE Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn en_test_mode(&mut self) -> EnTestModeW<AnaRegClkCtrlReg2LdoClktopSpec> {
        EnTestModeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
SLICER LDO SHORT CKT PROTECTION ENABLE 0 = Slicer LDO Short Ckt Protection Disabled 1 = Slicer LDO Short Ckt Protection Enabled 0x0= Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn en_shrt_ckt(&mut self) -> EnShrtCktW<AnaRegClkCtrlReg2LdoClktopSpec> {
        EnShrtCktW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
SLICER LDO BYPASS ENABLE 0 = Slicer LDO in normal mode 1 = Slicer LDO Bypassed with external voltage 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn en_bypass(&mut self) -> EnBypassW<AnaRegClkCtrlReg2LdoClktopSpec> {
        EnBypassW::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
SLICER LDO BANDWIDTH CONTROL Need inverters on bits 8 and 10 need updated description 0x7 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ldo_bw_ctrl(&mut self) -> LdoBwCtrlW<AnaRegClkCtrlReg2LdoClktopSpec> {
        LdoBwCtrlW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
SLICER LDO SHORT CKT PROTECTION IBIAS CONTROL 0 = Nominal short circuit bias with nominal short circuit current limit 1 = 2X Nominal short circuit bias with higher short circuit current limit 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scprt_ibias_ctrl(&mut self) -> ScprtIbiasCtrlW<AnaRegClkCtrlReg2LdoClktopSpec> {
        ScprtIbiasCtrlW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
SLICER LDO PMOS PULL DOWN ENABLE 0 = Slicer LDO PMOS Pull Down disabled 1 = Slicer LDO PMOS Pull Down enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn enable_pmos_pulldown(&mut self) -> EnablePmosPulldownW<AnaRegClkCtrlReg2LdoClktopSpec> {
        EnablePmosPulldownW::new(self, 12)
    }
    #[doc = "Bits 13:15 - 15:13\\]
SLICER LDO TLOAD CONTROL Need inverter on bit13 updated description needed 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tload_ctrl(&mut self) -> TloadCtrlW<AnaRegClkCtrlReg2LdoClktopSpec> {
        TloadCtrlW::new(self, 13)
    }
    #[doc = "Bits 16:19 - 19:16\\]
SLICER LDO TEST MUX CONTROL (ONE HOT) Analog MUX enables to test output port 0000 = HI-Z Output 0001 = 0.6 * VLDO_OUT 0010 = VDD18*0.5 = 0.9V 0100 = VSSA 1000 = LDO Test Current (12.5uA) WARNING: Enabling more than one bit may damage the device 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn testmux_ctrl(&mut self) -> TestmuxCtrlW<AnaRegClkCtrlReg2LdoClktopSpec> {
        TestmuxCtrlW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
SLICER LDO BIST MUX CONTROL (ONE HOT) Analog MUX enables to BIST output port 0000 = HI-Z Output 0001 = VBG_0P9*10/9 =1.0 V 0010 = VDD18*0.5 = 0.9V 0100 = VLDO Output * 0.6 1000 = Floating WARNING: Enabling more than one bit may damage the device 0x4 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn bistmux_ctrl(&mut self) -> BistmuxCtrlW<AnaRegClkCtrlReg2LdoClktopSpec> {
        BistmuxCtrlW::new(self, 20)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegClkCtrlReg2LdoClktopSpec> {
        Reserved0W::new(self, 24)
    }
}
#[doc = "ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg2_ldo_clktop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg2_ldo_clktop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegClkCtrlReg2LdoClktopSpec;
impl crate::RegisterSpec for AnaRegClkCtrlReg2LdoClktopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_clk_ctrl_reg2_ldo_clktop::R`](R) reader structure"]
impl crate::Readable for AnaRegClkCtrlReg2LdoClktopSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_clk_ctrl_reg2_ldo_clktop::W`](W) writer structure"]
impl crate::Writable for AnaRegClkCtrlReg2LdoClktopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_CLK_CTRL_REG2_LDO_CLKTOP to value 0"]
impl crate::Resettable for AnaRegClkCtrlReg2LdoClktopSpec {
    const RESET_VALUE: u32 = 0;
}
