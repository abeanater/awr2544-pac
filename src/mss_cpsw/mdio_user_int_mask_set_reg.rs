#[doc = "Register `MDIO_USER_INT_MASK_SET_REG` reader"]
pub type R = crate::R<MdioUserIntMaskSetRegSpec>;
#[doc = "Register `MDIO_USER_INT_MASK_SET_REG` writer"]
pub type W = crate::W<MdioUserIntMaskSetRegSpec>;
#[doc = "Field `MDIO_USER_INTERRUPT` reader - 1:0\\]
MDIO user interrupt mask set"]
pub type MdioUserInterruptR = crate::FieldReader;
#[doc = "Field `MDIO_USER_INTERRUPT` writer - 1:0\\]
MDIO user interrupt mask set"]
pub type MdioUserInterruptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO user interrupt mask set"]
    #[inline(always)]
    pub fn mdio_user_interrupt(&self) -> MdioUserInterruptR {
        MdioUserInterruptR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO user interrupt mask set"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_user_interrupt(&mut self) -> MdioUserInterruptW<MdioUserIntMaskSetRegSpec> {
        MdioUserInterruptW::new(self, 0)
    }
}
#[doc = "MDIO User Interrupt Mask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_int_mask_set_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_int_mask_set_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioUserIntMaskSetRegSpec;
impl crate::RegisterSpec for MdioUserIntMaskSetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_user_int_mask_set_reg::R`](R) reader structure"]
impl crate::Readable for MdioUserIntMaskSetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_user_int_mask_set_reg::W`](W) writer structure"]
impl crate::Writable for MdioUserIntMaskSetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_USER_INT_MASK_SET_REG to value 0"]
impl crate::Resettable for MdioUserIntMaskSetRegSpec {
    const RESET_VALUE: u32 = 0;
}
