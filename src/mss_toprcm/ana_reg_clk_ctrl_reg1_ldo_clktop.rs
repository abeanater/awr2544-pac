#[doc = "Register `ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP` reader"]
pub type R = crate::R<AnaRegClkCtrlReg1LdoClktopSpec>;
#[doc = "Register `ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP` writer"]
pub type W = crate::W<AnaRegClkCtrlReg1LdoClktopSpec>;
#[doc = "Field `EN_SLICER_LDO` reader - 0:0\\]
SLICER LDO ENABLE 0 = Slicer LDO Disabled 1 = Slicer LDO Enabled 0x1 = Functional Reset"]
pub type EnSlicerLdoR = crate::BitReader;
#[doc = "Field `EN_SLICER_LDO` writer - 0:0\\]
SLICER LDO ENABLE 0 = Slicer LDO Disabled 1 = Slicer LDO Enabled 0x1 = Functional Reset"]
pub type EnSlicerLdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED0` reader - 8:1\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 8:1\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_BIST_DISABLE_LDO` reader - 9:9\\]
DC BIST Disable for LDO 0 = Normal operation of DC BIST 1 = DC BIST Disabled 0x0 = Functional Reset"]
pub type ClkBistDisableLdoR = crate::BitReader;
#[doc = "Field `CLK_BIST_DISABLE_LDO` writer - 9:9\\]
DC BIST Disable for LDO 0 = Normal operation of DC BIST 1 = DC BIST Disabled 0x0 = Functional Reset"]
pub type ClkBistDisableLdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:10\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:10\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SLICER LDO ENABLE 0 = Slicer LDO Disabled 1 = Slicer LDO Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn en_slicer_ldo(&self) -> EnSlicerLdoR {
        EnSlicerLdoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - 8:1\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
DC BIST Disable for LDO 0 = Normal operation of DC BIST 1 = DC BIST Disabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn clk_bist_disable_ldo(&self) -> ClkBistDisableLdoR {
        ClkBistDisableLdoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SLICER LDO ENABLE 0 = Slicer LDO Disabled 1 = Slicer LDO Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn en_slicer_ldo(&mut self) -> EnSlicerLdoW<AnaRegClkCtrlReg1LdoClktopSpec> {
        EnSlicerLdoW::new(self, 0)
    }
    #[doc = "Bits 1:8 - 8:1\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegClkCtrlReg1LdoClktopSpec> {
        Reserved0W::new(self, 1)
    }
    #[doc = "Bit 9 - 9:9\\]
DC BIST Disable for LDO 0 = Normal operation of DC BIST 1 = DC BIST Disabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn clk_bist_disable_ldo(&mut self) -> ClkBistDisableLdoW<AnaRegClkCtrlReg1LdoClktopSpec> {
        ClkBistDisableLdoW::new(self, 9)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AnaRegClkCtrlReg1LdoClktopSpec> {
        Reserved1W::new(self, 10)
    }
}
#[doc = "ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_clk_ctrl_reg1_ldo_clktop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_clk_ctrl_reg1_ldo_clktop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegClkCtrlReg1LdoClktopSpec;
impl crate::RegisterSpec for AnaRegClkCtrlReg1LdoClktopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_clk_ctrl_reg1_ldo_clktop::R`](R) reader structure"]
impl crate::Readable for AnaRegClkCtrlReg1LdoClktopSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_clk_ctrl_reg1_ldo_clktop::W`](W) writer structure"]
impl crate::Writable for AnaRegClkCtrlReg1LdoClktopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_CLK_CTRL_REG1_LDO_CLKTOP to value 0"]
impl crate::Resettable for AnaRegClkCtrlReg1LdoClktopSpec {
    const RESET_VALUE: u32 = 0;
}
