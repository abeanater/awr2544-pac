#[doc = "Register `ANALOG_WU_STATUS_REG_POLARITY_INV` reader"]
pub type R = crate::R<AnalogWuStatusRegPolarityInvSpec>;
#[doc = "Register `ANALOG_WU_STATUS_REG_POLARITY_INV` writer"]
pub type W = crate::W<AnalogWuStatusRegPolarityInvSpec>;
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
    pub fn inv_ctrl(&mut self) -> InvCtrlW<AnalogWuStatusRegPolarityInvSpec> {
        InvCtrlW::new(self, 0)
    }
}
#[doc = "ANALOG_WU_STATUS_REG_POLARITY_INV\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_wu_status_reg_polarity_inv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_wu_status_reg_polarity_inv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogWuStatusRegPolarityInvSpec;
impl crate::RegisterSpec for AnalogWuStatusRegPolarityInvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_wu_status_reg_polarity_inv::R`](R) reader structure"]
impl crate::Readable for AnalogWuStatusRegPolarityInvSpec {}
#[doc = "`write(|w| ..)` method takes [`analog_wu_status_reg_polarity_inv::W`](W) writer structure"]
impl crate::Writable for AnalogWuStatusRegPolarityInvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_WU_STATUS_REG_POLARITY_INV to value 0"]
impl crate::Resettable for AnalogWuStatusRegPolarityInvSpec {
    const RESET_VALUE: u32 = 0;
}
