#[doc = "Register `MDIO_USER_INT_RAW_REG` reader"]
pub type R = crate::R<MdioUserIntRawRegSpec>;
#[doc = "Register `MDIO_USER_INT_RAW_REG` writer"]
pub type W = crate::W<MdioUserIntRawRegSpec>;
#[doc = "Field `USER_INTERRUPT_RAW` reader - 1:0\\]
User interrupt raw"]
pub type UserInterruptRawR = crate::FieldReader;
#[doc = "Field `USER_INTERRUPT_RAW` writer - 1:0\\]
User interrupt raw"]
pub type UserInterruptRawW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt raw"]
    #[inline(always)]
    pub fn user_interrupt_raw(&self) -> UserInterruptRawR {
        UserInterruptRawR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn user_interrupt_raw(&mut self) -> UserInterruptRawW<MdioUserIntRawRegSpec> {
        UserInterruptRawW::new(self, 0)
    }
}
#[doc = "MDIO User Interrupt Raw Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_int_raw_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_int_raw_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioUserIntRawRegSpec;
impl crate::RegisterSpec for MdioUserIntRawRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_user_int_raw_reg::R`](R) reader structure"]
impl crate::Readable for MdioUserIntRawRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_user_int_raw_reg::W`](W) writer structure"]
impl crate::Writable for MdioUserIntRawRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_USER_INT_RAW_REG to value 0"]
impl crate::Resettable for MdioUserIntRawRegSpec {
    const RESET_VALUE: u32 = 0;
}
