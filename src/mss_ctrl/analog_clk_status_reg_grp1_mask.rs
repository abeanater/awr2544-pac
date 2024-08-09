#[doc = "Register `ANALOG_CLK_STATUS_REG_GRP1_MASK` reader"]
pub type R = crate::R<AnalogClkStatusRegGrp1MaskSpec>;
#[doc = "Register `ANALOG_CLK_STATUS_REG_GRP1_MASK` writer"]
pub type W = crate::W<AnalogClkStatusRegGrp1MaskSpec>;
#[doc = "Field `mask` reader - 31:0\\]
Writing 1'b1 : Masks the corresponding status bit before generating a group 1 ESM error. 1'b0 : Unmasks the corresponding status bit before generating a group 1 ESM error."]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `mask` writer - 31:0\\]
Writing 1'b1 : Masks the corresponding status bit before generating a group 1 ESM error. 1'b0 : Unmasks the corresponding status bit before generating a group 1 ESM error."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Writing 1'b1 : Masks the corresponding status bit before generating a group 1 ESM error. 1'b0 : Unmasks the corresponding status bit before generating a group 1 ESM error."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Writing 1'b1 : Masks the corresponding status bit before generating a group 1 ESM error. 1'b0 : Unmasks the corresponding status bit before generating a group 1 ESM error."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<AnalogClkStatusRegGrp1MaskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "ANALOG_CLK_STATUS_REG_GRP1_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_clk_status_reg_grp1_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_clk_status_reg_grp1_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogClkStatusRegGrp1MaskSpec;
impl crate::RegisterSpec for AnalogClkStatusRegGrp1MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_clk_status_reg_grp1_mask::R`](R) reader structure"]
impl crate::Readable for AnalogClkStatusRegGrp1MaskSpec {}
#[doc = "`write(|w| ..)` method takes [`analog_clk_status_reg_grp1_mask::W`](W) writer structure"]
impl crate::Writable for AnalogClkStatusRegGrp1MaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CLK_STATUS_REG_GRP1_MASK to value 0"]
impl crate::Resettable for AnalogClkStatusRegGrp1MaskSpec {
    const RESET_VALUE: u32 = 0;
}
