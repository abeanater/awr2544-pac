#[doc = "Register `CCCA_CFG0` reader"]
pub type R = crate::R<CccaCfg0Spec>;
#[doc = "Register `CCCA_CFG0` writer"]
pub type W = crate::W<CccaCfg0Spec>;
#[doc = "Field `ccca_clk0_sel` reader - 2:0\\]
Selection for Clock 0 0: Select clock0_src0 as source for counter0 1: Select clock0_src1 as source for counter0 2: Select clock0_src2 as source for counter0 ... 7: Select clock0_src7 as source for counter0"]
pub type CccaClk0SelR = crate::FieldReader;
#[doc = "Field `ccca_clk0_sel` writer - 2:0\\]
Selection for Clock 0 0: Select clock0_src0 as source for counter0 1: Select clock0_src1 as source for counter0 2: Select clock0_src2 as source for counter0 ... 7: Select clock0_src7 as source for counter0"]
pub type CccaClk0SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ccca_clk1_sel` reader - 5:3\\]
Selection for Clock 1 0: Select clock0_src0 as source for counter1 1: Select clock0_src1 as source for counter1 2: Select clock0_src2 as source for counter1 ... 7: Select clock0_src7 as source for counter1"]
pub type CccaClk1SelR = crate::FieldReader;
#[doc = "Field `ccca_clk1_sel` writer - 5:3\\]
Selection for Clock 1 0: Select clock0_src0 as source for counter1 1: Select clock0_src1 as source for counter1 2: Select clock0_src2 as source for counter1 ... 7: Select clock0_src7 as source for counter1"]
pub type CccaClk1SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ccca_disable_clocks` reader - 6:6\\]
1: Clock gated to counter0 and counter1 0: Normal mode"]
pub type CccaDisableClocksR = crate::BitReader;
#[doc = "Field `ccca_disable_clocks` writer - 6:6\\]
1: Clock gated to counter0 and counter1 0: Normal mode"]
pub type CccaDisableClocksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ccca_enable_module` reader - 7:7\\]
1'b1: Enables CCCA 1'b0: Disables CCCA"]
pub type CccaEnableModuleR = crate::BitReader;
#[doc = "Field `ccca_enable_module` writer - 7:7\\]
1'b1: Enables CCCA 1'b0: Disables CCCA"]
pub type CccaEnableModuleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ccca_single_shot_mode` reader - 8:8\\]
1: Single shot mode 0: Continuous mode"]
pub type CccaSingleShotModeR = crate::BitReader;
#[doc = "Field `ccca_single_shot_mode` writer - 8:8\\]
1: Single shot mode 0: Continuous mode"]
pub type CccaSingleShotModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ccca_margin_count` reader - 31:16\\]
Margin value for clock comparison in terms of counter1 clock.CCC error will not be generated if counter1 counter value is within count1_expected_val +/- MARGIN_COUNT"]
pub type CccaMarginCountR = crate::FieldReader<u16>;
#[doc = "Field `ccca_margin_count` writer - 31:16\\]
Margin value for clock comparison in terms of counter1 clock.CCC error will not be generated if counter1 counter value is within count1_expected_val +/- MARGIN_COUNT"]
pub type CccaMarginCountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selection for Clock 0 0: Select clock0_src0 as source for counter0 1: Select clock0_src1 as source for counter0 2: Select clock0_src2 as source for counter0 ... 7: Select clock0_src7 as source for counter0"]
    #[inline(always)]
    pub fn ccca_clk0_sel(&self) -> CccaClk0SelR {
        CccaClk0SelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Selection for Clock 1 0: Select clock0_src0 as source for counter1 1: Select clock0_src1 as source for counter1 2: Select clock0_src2 as source for counter1 ... 7: Select clock0_src7 as source for counter1"]
    #[inline(always)]
    pub fn ccca_clk1_sel(&self) -> CccaClk1SelR {
        CccaClk1SelR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
1: Clock gated to counter0 and counter1 0: Normal mode"]
    #[inline(always)]
    pub fn ccca_disable_clocks(&self) -> CccaDisableClocksR {
        CccaDisableClocksR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
1'b1: Enables CCCA 1'b0: Disables CCCA"]
    #[inline(always)]
    pub fn ccca_enable_module(&self) -> CccaEnableModuleR {
        CccaEnableModuleR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
1: Single shot mode 0: Continuous mode"]
    #[inline(always)]
    pub fn ccca_single_shot_mode(&self) -> CccaSingleShotModeR {
        CccaSingleShotModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Margin value for clock comparison in terms of counter1 clock.CCC error will not be generated if counter1 counter value is within count1_expected_val +/- MARGIN_COUNT"]
    #[inline(always)]
    pub fn ccca_margin_count(&self) -> CccaMarginCountR {
        CccaMarginCountR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selection for Clock 0 0: Select clock0_src0 as source for counter0 1: Select clock0_src1 as source for counter0 2: Select clock0_src2 as source for counter0 ... 7: Select clock0_src7 as source for counter0"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_clk0_sel(&mut self) -> CccaClk0SelW<CccaCfg0Spec> {
        CccaClk0SelW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Selection for Clock 1 0: Select clock0_src0 as source for counter1 1: Select clock0_src1 as source for counter1 2: Select clock0_src2 as source for counter1 ... 7: Select clock0_src7 as source for counter1"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_clk1_sel(&mut self) -> CccaClk1SelW<CccaCfg0Spec> {
        CccaClk1SelW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
1: Clock gated to counter0 and counter1 0: Normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_disable_clocks(&mut self) -> CccaDisableClocksW<CccaCfg0Spec> {
        CccaDisableClocksW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
1'b1: Enables CCCA 1'b0: Disables CCCA"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_enable_module(&mut self) -> CccaEnableModuleW<CccaCfg0Spec> {
        CccaEnableModuleW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
1: Single shot mode 0: Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_single_shot_mode(&mut self) -> CccaSingleShotModeW<CccaCfg0Spec> {
        CccaSingleShotModeW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Margin value for clock comparison in terms of counter1 clock.CCC error will not be generated if counter1 counter value is within count1_expected_val +/- MARGIN_COUNT"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_margin_count(&mut self) -> CccaMarginCountW<CccaCfg0Spec> {
        CccaMarginCountW::new(self, 16)
    }
}
#[doc = "CCCA_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ccca_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccca_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccaCfg0Spec;
impl crate::RegisterSpec for CccaCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccca_cfg0::R`](R) reader structure"]
impl crate::Readable for CccaCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ccca_cfg0::W`](W) writer structure"]
impl crate::Writable for CccaCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCA_CFG0 to value 0"]
impl crate::Resettable for CccaCfg0Spec {
    const RESET_VALUE: u32 = 0;
}
