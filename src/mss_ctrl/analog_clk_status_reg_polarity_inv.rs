#[doc = "Register `ANALOG_CLK_STATUS_REG_POLARITY_INV` reader"]
pub type R = crate::R<AnalogClkStatusRegPolarityInvSpec>;
#[doc = "Register `ANALOG_CLK_STATUS_REG_POLARITY_INV` writer"]
pub type W = crate::W<AnalogClkStatusRegPolarityInvSpec>;
#[doc = "Field `inv_ctrl` reader - 31:0\\]
This register decides the polarity of each status bit before providing to the MSS_ESM. Each bit controls the respective status bit."]
pub type InvCtrlR = crate::FieldReader<u32>;
#[doc = "Field `inv_ctrl` writer - 31:0\\]
This register decides the polarity of each status bit before providing to the MSS_ESM. Each bit controls the respective status bit."]
pub type InvCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register decides the polarity of each status bit before providing to the MSS_ESM. Each bit controls the respective status bit."]
    #[inline(always)]
    pub fn inv_ctrl(&self) -> InvCtrlR {
        InvCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register decides the polarity of each status bit before providing to the MSS_ESM. Each bit controls the respective status bit."]
    #[inline(always)]
    #[must_use]
    pub fn inv_ctrl(&mut self) -> InvCtrlW<AnalogClkStatusRegPolarityInvSpec> {
        InvCtrlW::new(self, 0)
    }
}
#[doc = "ANALOG_CLK_STATUS_REG_POLARITY_INV\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_clk_status_reg_polarity_inv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_clk_status_reg_polarity_inv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogClkStatusRegPolarityInvSpec;
impl crate::RegisterSpec for AnalogClkStatusRegPolarityInvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_clk_status_reg_polarity_inv::R`](R) reader structure"]
impl crate::Readable for AnalogClkStatusRegPolarityInvSpec {}
#[doc = "`write(|w| ..)` method takes [`analog_clk_status_reg_polarity_inv::W`](W) writer structure"]
impl crate::Writable for AnalogClkStatusRegPolarityInvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CLK_STATUS_REG_POLARITY_INV to value 0"]
impl crate::Resettable for AnalogClkStatusRegPolarityInvSpec {
    const RESET_VALUE: u32 = 0;
}
