#[doc = "Register `PADCT_cfg_reg` reader"]
pub type R = crate::R<PadctCfgRegSpec>;
#[doc = "Register `PADCT_cfg_reg` writer"]
pub type W = crate::W<PadctCfgRegSpec>;
#[doc = "Field `func_sel` reader - 3:0\\]
Function select"]
pub type FuncSelR = crate::FieldReader;
#[doc = "Field `func_sel` writer - 3:0\\]
Function select"]
pub type FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ie_override_ctrl` reader - 4:4\\]
Active Low Input Override Control : Write 1 to select Active low Input Override value to control IOs IE_N/RXACTIVE_N instead of the control from hardware"]
pub type IeOverrideCtrlR = crate::BitReader;
#[doc = "Field `ie_override_ctrl` writer - 4:4\\]
Active Low Input Override Control : Write 1 to select Active low Input Override value to control IOs IE_N/RXACTIVE_N instead of the control from hardware"]
pub type IeOverrideCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ie_override` reader - 5:5\\]
Active Low Input Override"]
pub type IeOverrideR = crate::BitReader;
#[doc = "Field `ie_override` writer - 5:5\\]
Active Low Input Override"]
pub type IeOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `oe_override_ctrl` reader - 6:6\\]
Active Low Output Override Control : Write 1 to select Active low Output Override value to control IOs OE_N/GZ instead of the control from hardware"]
pub type OeOverrideCtrlR = crate::BitReader;
#[doc = "Field `oe_override_ctrl` writer - 6:6\\]
Active Low Output Override Control : Write 1 to select Active low Output Override value to control IOs OE_N/GZ instead of the control from hardware"]
pub type OeOverrideCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `oe_override` reader - 7:7\\]
Active Low Output Override"]
pub type OeOverrideR = crate::BitReader;
#[doc = "Field `oe_override` writer - 7:7\\]
Active Low Output Override"]
pub type OeOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pi` reader - 8:8\\]
Pull Inhibit/Pull Disable 0 -- Enable"]
pub type PiR = crate::BitReader;
#[doc = "Field `pi` writer - 8:8\\]
Pull Inhibit/Pull Disable 0 -- Enable"]
pub type PiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pupdsel` reader - 9:9\\]
Pullup/PullDown Selection 0 -- Pull Down"]
pub type PupdselR = crate::BitReader;
#[doc = "Field `pupdsel` writer - 9:9\\]
Pullup/PullDown Selection 0 -- Pull Down"]
pub type PupdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sc1` reader - 10:10\\]
IO Slew rate control : 0 : higher slew rate. 1: Lower slew rate."]
pub type Sc1R = crate::BitReader;
#[doc = "Field `sc1` writer - 10:10\\]
IO Slew rate control : 0 : higher slew rate. 1: Lower slew rate."]
pub type Sc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:11\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:11\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Function select"]
    #[inline(always)]
    pub fn func_sel(&self) -> FuncSelR {
        FuncSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Active Low Input Override Control : Write 1 to select Active low Input Override value to control IOs IE_N/RXACTIVE_N instead of the control from hardware"]
    #[inline(always)]
    pub fn ie_override_ctrl(&self) -> IeOverrideCtrlR {
        IeOverrideCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Active Low Input Override"]
    #[inline(always)]
    pub fn ie_override(&self) -> IeOverrideR {
        IeOverrideR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Active Low Output Override Control : Write 1 to select Active low Output Override value to control IOs OE_N/GZ instead of the control from hardware"]
    #[inline(always)]
    pub fn oe_override_ctrl(&self) -> OeOverrideCtrlR {
        OeOverrideCtrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Active Low Output Override"]
    #[inline(always)]
    pub fn oe_override(&self) -> OeOverrideR {
        OeOverrideR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Pull Inhibit/Pull Disable 0 -- Enable"]
    #[inline(always)]
    pub fn pi(&self) -> PiR {
        PiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Pullup/PullDown Selection 0 -- Pull Down"]
    #[inline(always)]
    pub fn pupdsel(&self) -> PupdselR {
        PupdselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
IO Slew rate control : 0 : higher slew rate. 1: Lower slew rate."]
    #[inline(always)]
    pub fn sc1(&self) -> Sc1R {
        Sc1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Function select"]
    #[inline(always)]
    #[must_use]
    pub fn func_sel(&mut self) -> FuncSelW<PadctCfgRegSpec> {
        FuncSelW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Active Low Input Override Control : Write 1 to select Active low Input Override value to control IOs IE_N/RXACTIVE_N instead of the control from hardware"]
    #[inline(always)]
    #[must_use]
    pub fn ie_override_ctrl(&mut self) -> IeOverrideCtrlW<PadctCfgRegSpec> {
        IeOverrideCtrlW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Active Low Input Override"]
    #[inline(always)]
    #[must_use]
    pub fn ie_override(&mut self) -> IeOverrideW<PadctCfgRegSpec> {
        IeOverrideW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Active Low Output Override Control : Write 1 to select Active low Output Override value to control IOs OE_N/GZ instead of the control from hardware"]
    #[inline(always)]
    #[must_use]
    pub fn oe_override_ctrl(&mut self) -> OeOverrideCtrlW<PadctCfgRegSpec> {
        OeOverrideCtrlW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Active Low Output Override"]
    #[inline(always)]
    #[must_use]
    pub fn oe_override(&mut self) -> OeOverrideW<PadctCfgRegSpec> {
        OeOverrideW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Pull Inhibit/Pull Disable 0 -- Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PiW<PadctCfgRegSpec> {
        PiW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Pullup/PullDown Selection 0 -- Pull Down"]
    #[inline(always)]
    #[must_use]
    pub fn pupdsel(&mut self) -> PupdselW<PadctCfgRegSpec> {
        PupdselW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
IO Slew rate control : 0 : higher slew rate. 1: Lower slew rate."]
    #[inline(always)]
    #[must_use]
    pub fn sc1(&mut self) -> Sc1W<PadctCfgRegSpec> {
        Sc1W::new(self, 10)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<PadctCfgRegSpec> {
        NuW::new(self, 11)
    }
}
#[doc = "PADCT_cfg_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`padct_cfg_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padct_cfg_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadctCfgRegSpec;
impl crate::RegisterSpec for PadctCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padct_cfg_reg::R`](R) reader structure"]
impl crate::Readable for PadctCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`padct_cfg_reg::W`](W) writer structure"]
impl crate::Writable for PadctCfgRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCT_cfg_reg to value 0"]
impl crate::Resettable for PadctCfgRegSpec {
    const RESET_VALUE: u32 = 0;
}
