#[doc = "Register `MDIO_USER_INT_MASK_CLEAR_REG` reader"]
pub type R = crate::R<MdioUserIntMaskClearRegSpec>;
#[doc = "Register `MDIO_USER_INT_MASK_CLEAR_REG` writer"]
pub type W = crate::W<MdioUserIntMaskClearRegSpec>;
#[doc = "Field `MDIO_USER_INTERRUPT` reader - 1:0\\]
MDIO user interrupt mask clear"]
pub type MdioUserInterruptR = crate::FieldReader;
#[doc = "Field `MDIO_USER_INTERRUPT` writer - 1:0\\]
MDIO user interrupt mask clear"]
pub type MdioUserInterruptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO user interrupt mask clear"]
    #[inline(always)]
    pub fn mdio_user_interrupt(&self) -> MdioUserInterruptR {
        MdioUserInterruptR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO user interrupt mask clear"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_user_interrupt(&mut self) -> MdioUserInterruptW<MdioUserIntMaskClearRegSpec> {
        MdioUserInterruptW::new(self, 0)
    }
}
#[doc = "MDIO User Interrupt Mask Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_int_mask_clear_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_int_mask_clear_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioUserIntMaskClearRegSpec;
impl crate::RegisterSpec for MdioUserIntMaskClearRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_user_int_mask_clear_reg::R`](R) reader structure"]
impl crate::Readable for MdioUserIntMaskClearRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_user_int_mask_clear_reg::W`](W) writer structure"]
impl crate::Writable for MdioUserIntMaskClearRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_USER_INT_MASK_CLEAR_REG to value 0"]
impl crate::Resettable for MdioUserIntMaskClearRegSpec {
    const RESET_VALUE: u32 = 0;
}
