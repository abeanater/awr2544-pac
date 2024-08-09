#[doc = "Register `CCCB_CFG0` reader"]
pub type R = crate::R<CccbCfg0Spec>;
#[doc = "Register `CCCB_CFG0` writer"]
pub type W = crate::W<CccbCfg0Spec>;
#[doc = "Field `CCCB_clk0_sel` reader - 2:0\\]
Selection for Clock 0 0: Select clock0_src0 as source for counter0 1: Select clock0_src1 as source for counter0 2: Select clock0_src2 as source for counter0 ... 7: Select clock0_src7 as source for counter0"]
pub type CccbClk0SelR = crate::FieldReader;
#[doc = "Field `CCCB_clk0_sel` writer - 2:0\\]
Selection for Clock 0 0: Select clock0_src0 as source for counter0 1: Select clock0_src1 as source for counter0 2: Select clock0_src2 as source for counter0 ... 7: Select clock0_src7 as source for counter0"]
pub type CccbClk0SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCCB_clk1_sel` reader - 5:3\\]
Selection for Clock 1 0: Select clock0_src0 as source for counter1 1: Select clock0_src1 as source for counter1 2: Select clock0_src2 as source for counter1 ... 7: Select clock0_src7 as source for counter1"]
pub type CccbClk1SelR = crate::FieldReader;
#[doc = "Field `CCCB_clk1_sel` writer - 5:3\\]
Selection for Clock 1 0: Select clock0_src0 as source for counter1 1: Select clock0_src1 as source for counter1 2: Select clock0_src2 as source for counter1 ... 7: Select clock0_src7 as source for counter1"]
pub type CccbClk1SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cccb_disable_clocks` reader - 6:6\\]
1: Clock gated to counter0 and counter1 0: Normal mode"]
pub type CccbDisableClocksR = crate::BitReader;
#[doc = "Field `cccb_disable_clocks` writer - 6:6\\]
1: Clock gated to counter0 and counter1 0: Normal mode"]
pub type CccbDisableClocksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cccb_enable_module` reader - 7:7\\]
1'b1: Enables CCCB 1'b0: Disables CCCB"]
pub type CccbEnableModuleR = crate::BitReader;
#[doc = "Field `cccb_enable_module` writer - 7:7\\]
1'b1: Enables CCCB 1'b0: Disables CCCB"]
pub type CccbEnableModuleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cccb_single_shot_mode` reader - 8:8\\]
1: Single shot mode 0: Continuous mode"]
pub type CccbSingleShotModeR = crate::BitReader;
#[doc = "Field `cccb_single_shot_mode` writer - 8:8\\]
1: Single shot mode 0: Continuous mode"]
pub type CccbSingleShotModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cccb_margin_count` reader - 31:16\\]
Margin value for clock comparison in terms of counter1 clock.CCC error will not be generated if counter1 counter value is within count1_expected_val +/- MARGIN_COUNT"]
pub type CccbMarginCountR = crate::FieldReader<u16>;
#[doc = "Field `cccb_margin_count` writer - 31:16\\]
Margin value for clock comparison in terms of counter1 clock.CCC error will not be generated if counter1 counter value is within count1_expected_val +/- MARGIN_COUNT"]
pub type CccbMarginCountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selection for Clock 0 0: Select clock0_src0 as source for counter0 1: Select clock0_src1 as source for counter0 2: Select clock0_src2 as source for counter0 ... 7: Select clock0_src7 as source for counter0"]
    #[inline(always)]
    pub fn cccb_clk0_sel(&self) -> CccbClk0SelR {
        CccbClk0SelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Selection for Clock 1 0: Select clock0_src0 as source for counter1 1: Select clock0_src1 as source for counter1 2: Select clock0_src2 as source for counter1 ... 7: Select clock0_src7 as source for counter1"]
    #[inline(always)]
    pub fn cccb_clk1_sel(&self) -> CccbClk1SelR {
        CccbClk1SelR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
1: Clock gated to counter0 and counter1 0: Normal mode"]
    #[inline(always)]
    pub fn cccb_disable_clocks(&self) -> CccbDisableClocksR {
        CccbDisableClocksR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
1'b1: Enables CCCB 1'b0: Disables CCCB"]
    #[inline(always)]
    pub fn cccb_enable_module(&self) -> CccbEnableModuleR {
        CccbEnableModuleR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
1: Single shot mode 0: Continuous mode"]
    #[inline(always)]
    pub fn cccb_single_shot_mode(&self) -> CccbSingleShotModeR {
        CccbSingleShotModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Margin value for clock comparison in terms of counter1 clock.CCC error will not be generated if counter1 counter value is within count1_expected_val +/- MARGIN_COUNT"]
    #[inline(always)]
    pub fn cccb_margin_count(&self) -> CccbMarginCountR {
        CccbMarginCountR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selection for Clock 0 0: Select clock0_src0 as source for counter0 1: Select clock0_src1 as source for counter0 2: Select clock0_src2 as source for counter0 ... 7: Select clock0_src7 as source for counter0"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_clk0_sel(&mut self) -> CccbClk0SelW<CccbCfg0Spec> {
        CccbClk0SelW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Selection for Clock 1 0: Select clock0_src0 as source for counter1 1: Select clock0_src1 as source for counter1 2: Select clock0_src2 as source for counter1 ... 7: Select clock0_src7 as source for counter1"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_clk1_sel(&mut self) -> CccbClk1SelW<CccbCfg0Spec> {
        CccbClk1SelW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
1: Clock gated to counter0 and counter1 0: Normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_disable_clocks(&mut self) -> CccbDisableClocksW<CccbCfg0Spec> {
        CccbDisableClocksW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
1'b1: Enables CCCB 1'b0: Disables CCCB"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_enable_module(&mut self) -> CccbEnableModuleW<CccbCfg0Spec> {
        CccbEnableModuleW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
1: Single shot mode 0: Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_single_shot_mode(&mut self) -> CccbSingleShotModeW<CccbCfg0Spec> {
        CccbSingleShotModeW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Margin value for clock comparison in terms of counter1 clock.CCC error will not be generated if counter1 counter value is within count1_expected_val +/- MARGIN_COUNT"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_margin_count(&mut self) -> CccbMarginCountW<CccbCfg0Spec> {
        CccbMarginCountW::new(self, 16)
    }
}
#[doc = "CCCB_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`cccb_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccb_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccbCfg0Spec;
impl crate::RegisterSpec for CccbCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccb_cfg0::R`](R) reader structure"]
impl crate::Readable for CccbCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cccb_cfg0::W`](W) writer structure"]
impl crate::Writable for CccbCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCB_CFG0 to value 0"]
impl crate::Resettable for CccbCfg0Spec {
    const RESET_VALUE: u32 = 0;
}
