#[doc = "Register `MDIO_LINK_INT_MASK_CLEAR_REG` reader"]
pub type R = crate::R<MdioLinkIntMaskClearRegSpec>;
#[doc = "Register `MDIO_LINK_INT_MASK_CLEAR_REG` writer"]
pub type W = crate::W<MdioLinkIntMaskClearRegSpec>;
#[doc = "Field `MDIO_LINK_INTERRUPT` reader - 0:0\\]
MDIO link interrupt mask clear"]
pub type MdioLinkInterruptR = crate::BitReader;
#[doc = "Field `MDIO_LINK_INTERRUPT` writer - 0:0\\]
MDIO link interrupt mask clear"]
pub type MdioLinkInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MDIO link interrupt mask clear"]
    #[inline(always)]
    pub fn mdio_link_interrupt(&self) -> MdioLinkInterruptR {
        MdioLinkInterruptR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MDIO link interrupt mask clear"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_link_interrupt(&mut self) -> MdioLinkInterruptW<MdioLinkIntMaskClearRegSpec> {
        MdioLinkInterruptW::new(self, 0)
    }
}
#[doc = "MDIO Link Interrupt Mask Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_link_int_mask_clear_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_link_int_mask_clear_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioLinkIntMaskClearRegSpec;
impl crate::RegisterSpec for MdioLinkIntMaskClearRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_link_int_mask_clear_reg::R`](R) reader structure"]
impl crate::Readable for MdioLinkIntMaskClearRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_link_int_mask_clear_reg::W`](W) writer structure"]
impl crate::Writable for MdioLinkIntMaskClearRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_LINK_INT_MASK_CLEAR_REG to value 0"]
impl crate::Resettable for MdioLinkIntMaskClearRegSpec {
    const RESET_VALUE: u32 = 0;
}
