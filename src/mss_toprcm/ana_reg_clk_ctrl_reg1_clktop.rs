#[doc = "Register `ANA_REG_CLK_CTRL_REG1_CLKTOP` reader"]
pub type R = crate::R<AnaRegClkCtrlReg1ClktopSpec>;
#[doc = "Register `ANA_REG_CLK_CTRL_REG1_CLKTOP` writer"]
pub type W = crate::W<AnaRegClkCtrlReg1ClktopSpec>;
#[doc = "Field `ENABLE_BIAS_XO_SLICER` reader - 0:0\\]
Enable Bias for Crystal Oscillator and Slicer 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
pub type EnableBiasXoSlicerR = crate::BitReader;
#[doc = "Field `ENABLE_BIAS_XO_SLICER` writer - 0:0\\]
Enable Bias for Crystal Oscillator and Slicer 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
pub type EnableBiasXoSlicerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLICER_CLKP` reader - 1:1\\]
Enable CLKP Input Slicer 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
pub type EnableSlicerClkpR = crate::BitReader;
#[doc = "Field `ENABLE_SLICER_CLKP` writer - 1:1\\]
Enable CLKP Input Slicer 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
pub type EnableSlicerClkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_XOSC` reader - 2:2\\]
Enable Crystal Oscillator 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
pub type EnableXoscR = crate::BitReader;
#[doc = "Field `ENABLE_XOSC` writer - 2:2\\]
Enable Crystal Oscillator 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
pub type EnableXoscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED0` reader - 31:3\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED0` writer - 31:3\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Bias for Crystal Oscillator and Slicer 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn enable_bias_xo_slicer(&self) -> EnableBiasXoSlicerR {
        EnableBiasXoSlicerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable CLKP Input Slicer 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn enable_slicer_clkp(&self) -> EnableSlicerClkpR {
        EnableSlicerClkpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Crystal Oscillator 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn enable_xosc(&self) -> EnableXoscR {
        EnableXoscR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Bias for Crystal Oscillator and Slicer 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn enable_bias_xo_slicer(&mut self) -> EnableBiasXoSlicerW<AnaRegClkCtrlReg1ClktopSpec> {
        EnableBiasXoSlicerW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable CLKP Input Slicer 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn enable_slicer_clkp(&mut self) -> EnableSlicerClkpW<AnaRegClkCtrlReg1ClktopSpec> {
        EnableSlicerClkpW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable Crystal Oscillator 0 = Disabled 1 = Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn enable_xosc(&mut self) -> EnableXoscW<AnaRegClkCtrlReg1ClktopSpec> {
        EnableXoscW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegClkCtrlReg1ClktopSpec> {
        Reserved0W::new(self, 3)
    }
}
#[doc = "ANA_REG_CLK_CTRL_REG1_CLKTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg1_clktop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg1_clktop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegClkCtrlReg1ClktopSpec;
impl crate::RegisterSpec for AnaRegClkCtrlReg1ClktopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_clk_ctrl_reg1_clktop::R`](R) reader structure"]
impl crate::Readable for AnaRegClkCtrlReg1ClktopSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_clk_ctrl_reg1_clktop::W`](W) writer structure"]
impl crate::Writable for AnaRegClkCtrlReg1ClktopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_CLK_CTRL_REG1_CLKTOP to value 0"]
impl crate::Resettable for AnaRegClkCtrlReg1ClktopSpec {
    const RESET_VALUE: u32 = 0;
}
