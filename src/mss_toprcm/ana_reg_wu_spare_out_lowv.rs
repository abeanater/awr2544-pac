#[doc = "Register `ANA_REG_WU_SPARE_OUT_LOWV` reader"]
pub type R = crate::R<AnaRegWuSpareOutLowvSpec>;
#[doc = "Register `ANA_REG_WU_SPARE_OUT_LOWV` writer"]
pub type W = crate::W<AnaRegWuSpareOutLowvSpec>;
#[doc = "Field `VDDCLK18DET` reader - 0:0\\]
Status of 1.8V CLK Supply"]
pub type Vddclk18detR = crate::BitReader;
#[doc = "Field `VDDCLK18DET` writer - 0:0\\]
Status of 1.8V CLK Supply"]
pub type Vddclk18detW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDARF_DET` reader - 1:1\\]
Status of 1.3V RF Supply"]
pub type VddarfDetR = crate::BitReader;
#[doc = "Field `VDDARF_DET` writer - 1:1\\]
Status of 1.3V RF Supply"]
pub type VddarfDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDS18DET` reader - 2:2\\]
Status of 1.8V IO Bias Supply"]
pub type Vdds18detR = crate::BitReader;
#[doc = "Field `VDDS18DET` writer - 2:2\\]
Status of 1.8V IO Bias Supply"]
pub type Vdds18detW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVMODE` reader - 3:3\\]
Status of VIO supply. 3.3V or 1.8V"]
pub type HvmodeR = crate::BitReader;
#[doc = "Field `HVMODE` writer - 3:3\\]
Status of VIO supply. 3.3V or 1.8V"]
pub type HvmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPPDET_OV_CTRL` reader - 4:4\\]
Supply Detect Override Bit"]
pub type SuppdetOvCtrlR = crate::BitReader;
#[doc = "Field `SUPPDET_OV_CTRL` writer - 4:4\\]
Supply Detect Override Bit"]
pub type SuppdetOvCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_OSC_CTRL` reader - 5:5\\]
Internal Oscillator Control"]
pub type IntOscCtrlR = crate::BitReader;
#[doc = "Field `INT_OSC_CTRL` writer - 5:5\\]
Internal Oscillator Control"]
pub type IntOscCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_OVDET_LOWV` reader - 6:6\\]
OV Detect of Core Supply-Unlatched"]
pub type CoreOvdetLowvR = crate::BitReader;
#[doc = "Field `CORE_OVDET_LOWV` writer - 6:6\\]
OV Detect of Core Supply-Unlatched"]
pub type CoreOvdetLowvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_UVDET_LOWV` reader - 7:7\\]
UV Detect of Core Supply-Unlatched"]
pub type CoreUvdetLowvR = crate::BitReader;
#[doc = "Field `CORE_UVDET_LOWV` writer - 7:7\\]
UV Detect of Core Supply-Unlatched"]
pub type CoreUvdetLowvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - 31:8\\]
Reserved Reads return 0x0 and writes have no effect."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` writer - 31:8\\]
Reserved Reads return 0x0 and writes have no effect."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of 1.8V CLK Supply"]
    #[inline(always)]
    pub fn vddclk18det(&self) -> Vddclk18detR {
        Vddclk18detR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of 1.3V RF Supply"]
    #[inline(always)]
    pub fn vddarf_det(&self) -> VddarfDetR {
        VddarfDetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of 1.8V IO Bias Supply"]
    #[inline(always)]
    pub fn vdds18det(&self) -> Vdds18detR {
        Vdds18detR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of VIO supply. 3.3V or 1.8V"]
    #[inline(always)]
    pub fn hvmode(&self) -> HvmodeR {
        HvmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Supply Detect Override Bit"]
    #[inline(always)]
    pub fn suppdet_ov_ctrl(&self) -> SuppdetOvCtrlR {
        SuppdetOvCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal Oscillator Control"]
    #[inline(always)]
    pub fn int_osc_ctrl(&self) -> IntOscCtrlR {
        IntOscCtrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
OV Detect of Core Supply-Unlatched"]
    #[inline(always)]
    pub fn core_ovdet_lowv(&self) -> CoreOvdetLowvR {
        CoreOvdetLowvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
UV Detect of Core Supply-Unlatched"]
    #[inline(always)]
    pub fn core_uvdet_lowv(&self) -> CoreUvdetLowvR {
        CoreUvdetLowvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved Reads return 0x0 and writes have no effect."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of 1.8V CLK Supply"]
    #[inline(always)]
    #[must_use]
    pub fn vddclk18det(&mut self) -> Vddclk18detW<AnaRegWuSpareOutLowvSpec> {
        Vddclk18detW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of 1.3V RF Supply"]
    #[inline(always)]
    #[must_use]
    pub fn vddarf_det(&mut self) -> VddarfDetW<AnaRegWuSpareOutLowvSpec> {
        VddarfDetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of 1.8V IO Bias Supply"]
    #[inline(always)]
    #[must_use]
    pub fn vdds18det(&mut self) -> Vdds18detW<AnaRegWuSpareOutLowvSpec> {
        Vdds18detW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of VIO supply. 3.3V or 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn hvmode(&mut self) -> HvmodeW<AnaRegWuSpareOutLowvSpec> {
        HvmodeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Supply Detect Override Bit"]
    #[inline(always)]
    #[must_use]
    pub fn suppdet_ov_ctrl(&mut self) -> SuppdetOvCtrlW<AnaRegWuSpareOutLowvSpec> {
        SuppdetOvCtrlW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal Oscillator Control"]
    #[inline(always)]
    #[must_use]
    pub fn int_osc_ctrl(&mut self) -> IntOscCtrlW<AnaRegWuSpareOutLowvSpec> {
        IntOscCtrlW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
OV Detect of Core Supply-Unlatched"]
    #[inline(always)]
    #[must_use]
    pub fn core_ovdet_lowv(&mut self) -> CoreOvdetLowvW<AnaRegWuSpareOutLowvSpec> {
        CoreOvdetLowvW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
UV Detect of Core Supply-Unlatched"]
    #[inline(always)]
    #[must_use]
    pub fn core_uvdet_lowv(&mut self) -> CoreUvdetLowvW<AnaRegWuSpareOutLowvSpec> {
        CoreUvdetLowvW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved Reads return 0x0 and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegWuSpareOutLowvSpec> {
        Reserved0W::new(self, 8)
    }
}
#[doc = "ANA_REG_WU_SPARE_OUT_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_spare_out_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_spare_out_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegWuSpareOutLowvSpec;
impl crate::RegisterSpec for AnaRegWuSpareOutLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_wu_spare_out_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegWuSpareOutLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_wu_spare_out_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegWuSpareOutLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_WU_SPARE_OUT_LOWV to value 0"]
impl crate::Resettable for AnaRegWuSpareOutLowvSpec {
    const RESET_VALUE: u32 = 0;
}
