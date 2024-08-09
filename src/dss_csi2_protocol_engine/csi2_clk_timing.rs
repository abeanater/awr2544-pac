#[doc = "Register `CSI2_CLK_TIMING` reader"]
pub type R = crate::R<Csi2ClkTimingSpec>;
#[doc = "Register `CSI2_CLK_TIMING` writer"]
pub type W = crate::W<Csi2ClkTimingSpec>;
#[doc = "Field `DDR_CLK_POST` reader - 7:0\\]
Indicates the number of PPI Byte clock cycles after the de-assertion of the data request signal and the stop of the DDR clock. The values from 1 to 255 are valid. The value 0 is reserved. The value is not used if CSI2_CLK_CTRL.DDR_CLK_ALWAYS_ON is set to '1' since the DDR clock is always present."]
pub type DdrClkPostR = crate::FieldReader;
#[doc = "Field `DDR_CLK_POST` writer - 7:0\\]
Indicates the number of PPI Byte clock cycles after the de-assertion of the data request signal and the stop of the DDR clock. The values from 1 to 255 are valid. The value 0 is reserved. The value is not used if CSI2_CLK_CTRL.DDR_CLK_ALWAYS_ON is set to '1' since the DDR clock is always present."]
pub type DdrClkPostW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DDR_CLK_PRE` reader - 15:8\\]
Indicates the number of PPI Byte clock cycles between the start of the DDR clock and the assertion of the data request signal. The values from 1 to 255 are valid. The value 0 is reserved. The value is not used if CSI2_CLK_CTRL.DDR_CLK_ALWAYS_ON is set to '1' since the DDR clock is always present."]
pub type DdrClkPreR = crate::FieldReader;
#[doc = "Field `DDR_CLK_PRE` writer - 15:8\\]
Indicates the number of PPI Byte clock cycles between the start of the DDR clock and the assertion of the data request signal. The values from 1 to 255 are valid. The value 0 is reserved. The value is not used if CSI2_CLK_CTRL.DDR_CLK_ALWAYS_ON is set to '1' since the DDR clock is always present."]
pub type DdrClkPreW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates the number of PPI Byte clock cycles after the de-assertion of the data request signal and the stop of the DDR clock. The values from 1 to 255 are valid. The value 0 is reserved. The value is not used if CSI2_CLK_CTRL.DDR_CLK_ALWAYS_ON is set to '1' since the DDR clock is always present."]
    #[inline(always)]
    pub fn ddr_clk_post(&self) -> DdrClkPostR {
        DdrClkPostR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Indicates the number of PPI Byte clock cycles between the start of the DDR clock and the assertion of the data request signal. The values from 1 to 255 are valid. The value 0 is reserved. The value is not used if CSI2_CLK_CTRL.DDR_CLK_ALWAYS_ON is set to '1' since the DDR clock is always present."]
    #[inline(always)]
    pub fn ddr_clk_pre(&self) -> DdrClkPreR {
        DdrClkPreR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates the number of PPI Byte clock cycles after the de-assertion of the data request signal and the stop of the DDR clock. The values from 1 to 255 are valid. The value 0 is reserved. The value is not used if CSI2_CLK_CTRL.DDR_CLK_ALWAYS_ON is set to '1' since the DDR clock is always present."]
    #[inline(always)]
    #[must_use]
    pub fn ddr_clk_post(&mut self) -> DdrClkPostW<Csi2ClkTimingSpec> {
        DdrClkPostW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Indicates the number of PPI Byte clock cycles between the start of the DDR clock and the assertion of the data request signal. The values from 1 to 255 are valid. The value 0 is reserved. The value is not used if CSI2_CLK_CTRL.DDR_CLK_ALWAYS_ON is set to '1' since the DDR clock is always present."]
    #[inline(always)]
    #[must_use]
    pub fn ddr_clk_pre(&mut self) -> DdrClkPreW<Csi2ClkTimingSpec> {
        DdrClkPreW::new(self, 8)
    }
}
#[doc = "CLOCK TIMING REGISTER This register controls the CSI2 Protocol Engine module timers. This register shall not be modified while CSI2_CTRL.IF_EN is set to '1'.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_clk_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_clk_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2ClkTimingSpec;
impl crate::RegisterSpec for Csi2ClkTimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_clk_timing::R`](R) reader structure"]
impl crate::Readable for Csi2ClkTimingSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_clk_timing::W`](W) writer structure"]
impl crate::Writable for Csi2ClkTimingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_CLK_TIMING to value 0"]
impl crate::Resettable for Csi2ClkTimingSpec {
    const RESET_VALUE: u32 = 0;
}
