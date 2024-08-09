#[doc = "Register `PREVIOUS_NAME` reader"]
pub type R = crate::R<PreviousNameSpec>;
#[doc = "Register `PREVIOUS_NAME` writer"]
pub type W = crate::W<PreviousNameSpec>;
#[doc = "Field `hwa_en` reader - 2:0\\]
Enable/Disable Control: A value of ACC_ENABLE = 111b enables the Radar Hardware Accelerator and any other value of the register keeps the Accelerator Engine in disabled state. A 000b to 111b transition is expected to trigger a new Paramset execution"]
pub type HwaEnR = crate::FieldReader;
#[doc = "Field `hwa_en` writer - 2:0\\]
Enable/Disable Control: A value of ACC_ENABLE = 111b enables the Radar Hardware Accelerator and any other value of the register keeps the Accelerator Engine in disabled state. A 000b to 111b transition is expected to trigger a new Paramset execution"]
pub type HwaEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwa_clk_en` reader - 10:8\\]
Clock-gating Control: This register controls the enable/disable for the clock of the Radar Accelerator. This register bit can be set to 0 to clock-gate the accelerator when not using the accelerator. Before enabling the accelerator or before configuring the accelerator s registers, this register bit should be set first to 111b, so that the clock is available"]
pub type HwaClkEnR = crate::FieldReader;
#[doc = "Field `hwa_clk_en` writer - 10:8\\]
Clock-gating Control: This register controls the enable/disable for the clock of the Radar Accelerator. This register bit can be set to 0 to clock-gate the accelerator when not using the accelerator. Before enabling the accelerator or before configuring the accelerator s registers, this register bit should be set first to 111b, so that the clock is available"]
pub type HwaClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwa_reset` reader - 18:16\\]
Software Reset Control: This register provides software reset control for the Radar Hardware Accelerator. The assertion of these register bits by the main processor will bring the Accelerator Engine to a known reset state. This is mostly applicable for resetting the accelerator in case of unexpected behavior. The sequence to be followed in case software reset is to write 111b to this register and then a 000b"]
pub type HwaResetR = crate::FieldReader;
#[doc = "Field `hwa_reset` writer - 18:16\\]
Software Reset Control: This register provides software reset control for the Radar Hardware Accelerator. The assertion of these register bits by the main processor will bring the Accelerator Engine to a known reset state. This is mostly applicable for resetting the accelerator in case of unexpected behavior. The sequence to be followed in case software reset is to write 111b to this register and then a 000b"]
pub type HwaResetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwa_dyn_clk_en` reader - 24:24\\]
Dynamic Clock-gating Control: Setting this register bit to '1' enables the capability to clock gate the 4 Radar Accelerator core IPs (FFT datapath,CFAR,Memory compression,Local Maxima) based on the ParamSet being executed."]
pub type HwaDynClkEnR = crate::BitReader;
#[doc = "Field `hwa_dyn_clk_en` writer - 24:24\\]
Dynamic Clock-gating Control: Setting this register bit to '1' enables the capability to clock gate the 4 Radar Accelerator core IPs (FFT datapath,CFAR,Memory compression,Local Maxima) based on the ParamSet being executed."]
pub type HwaDynClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable/Disable Control: A value of ACC_ENABLE = 111b enables the Radar Hardware Accelerator and any other value of the register keeps the Accelerator Engine in disabled state. A 000b to 111b transition is expected to trigger a new Paramset execution"]
    #[inline(always)]
    pub fn hwa_en(&self) -> HwaEnR {
        HwaEnR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Clock-gating Control: This register controls the enable/disable for the clock of the Radar Accelerator. This register bit can be set to 0 to clock-gate the accelerator when not using the accelerator. Before enabling the accelerator or before configuring the accelerator s registers, this register bit should be set first to 111b, so that the clock is available"]
    #[inline(always)]
    pub fn hwa_clk_en(&self) -> HwaClkEnR {
        HwaClkEnR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Software Reset Control: This register provides software reset control for the Radar Hardware Accelerator. The assertion of these register bits by the main processor will bring the Accelerator Engine to a known reset state. This is mostly applicable for resetting the accelerator in case of unexpected behavior. The sequence to be followed in case software reset is to write 111b to this register and then a 000b"]
    #[inline(always)]
    pub fn hwa_reset(&self) -> HwaResetR {
        HwaResetR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Dynamic Clock-gating Control: Setting this register bit to '1' enables the capability to clock gate the 4 Radar Accelerator core IPs (FFT datapath,CFAR,Memory compression,Local Maxima) based on the ParamSet being executed."]
    #[inline(always)]
    pub fn hwa_dyn_clk_en(&self) -> HwaDynClkEnR {
        HwaDynClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable/Disable Control: A value of ACC_ENABLE = 111b enables the Radar Hardware Accelerator and any other value of the register keeps the Accelerator Engine in disabled state. A 000b to 111b transition is expected to trigger a new Paramset execution"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_en(&mut self) -> HwaEnW<PreviousNameSpec> {
        HwaEnW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Clock-gating Control: This register controls the enable/disable for the clock of the Radar Accelerator. This register bit can be set to 0 to clock-gate the accelerator when not using the accelerator. Before enabling the accelerator or before configuring the accelerator s registers, this register bit should be set first to 111b, so that the clock is available"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_clk_en(&mut self) -> HwaClkEnW<PreviousNameSpec> {
        HwaClkEnW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Software Reset Control: This register provides software reset control for the Radar Hardware Accelerator. The assertion of these register bits by the main processor will bring the Accelerator Engine to a known reset state. This is mostly applicable for resetting the accelerator in case of unexpected behavior. The sequence to be followed in case software reset is to write 111b to this register and then a 000b"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_reset(&mut self) -> HwaResetW<PreviousNameSpec> {
        HwaResetW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Dynamic Clock-gating Control: Setting this register bit to '1' enables the capability to clock gate the 4 Radar Accelerator core IPs (FFT datapath,CFAR,Memory compression,Local Maxima) based on the ParamSet being executed."]
    #[inline(always)]
    #[must_use]
    pub fn hwa_dyn_clk_en(&mut self) -> HwaDynClkEnW<PreviousNameSpec> {
        HwaDynClkEnW::new(self, 24)
    }
}
#[doc = "PREVIOUS_NAME\n\nYou can [`read`](crate::Reg::read) this register and get [`previous_name::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`previous_name::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PreviousNameSpec;
impl crate::RegisterSpec for PreviousNameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`previous_name::R`](R) reader structure"]
impl crate::Readable for PreviousNameSpec {}
#[doc = "`write(|w| ..)` method takes [`previous_name::W`](W) writer structure"]
impl crate::Writable for PreviousNameSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREVIOUS_NAME to value 0"]
impl crate::Resettable for PreviousNameSpec {
    const RESET_VALUE: u32 = 0;
}
