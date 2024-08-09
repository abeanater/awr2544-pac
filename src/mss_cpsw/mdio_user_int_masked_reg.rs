#[doc = "Register `MDIO_USER_INT_MASKED_REG` reader"]
pub type R = crate::R<MdioUserIntMaskedRegSpec>;
#[doc = "Register `MDIO_USER_INT_MASKED_REG` writer"]
pub type W = crate::W<MdioUserIntMaskedRegSpec>;
#[doc = "Field `USER_INTERRUPT_MASKED` reader - 1:0\\]
User interrupt masked"]
pub type UserInterruptMaskedR = crate::FieldReader;
#[doc = "Field `USER_INTERRUPT_MASKED` writer - 1:0\\]
User interrupt masked"]
pub type UserInterruptMaskedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt masked"]
    #[inline(always)]
    pub fn user_interrupt_masked(&self) -> UserInterruptMaskedR {
        UserInterruptMaskedR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt masked"]
    #[inline(always)]
    #[must_use]
    pub fn user_interrupt_masked(&mut self) -> UserInterruptMaskedW<MdioUserIntMaskedRegSpec> {
        UserInterruptMaskedW::new(self, 0)
    }
}
#[doc = "MDIO User Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_int_masked_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_int_masked_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioUserIntMaskedRegSpec;
impl crate::RegisterSpec for MdioUserIntMaskedRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_user_int_masked_reg::R`](R) reader structure"]
impl crate::Readable for MdioUserIntMaskedRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_user_int_masked_reg::W`](W) writer structure"]
impl crate::Writable for MdioUserIntMaskedRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_USER_INT_MASKED_REG to value 0"]
impl crate::Resettable for MdioUserIntMaskedRegSpec {
    const RESET_VALUE: u32 = 0;
}
