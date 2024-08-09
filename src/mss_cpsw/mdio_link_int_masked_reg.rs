#[doc = "Register `MDIO_LINK_INT_MASKED_REG` reader"]
pub type R = crate::R<MdioLinkIntMaskedRegSpec>;
#[doc = "Register `MDIO_LINK_INT_MASKED_REG` writer"]
pub type W = crate::W<MdioLinkIntMaskedRegSpec>;
#[doc = "Field `MDIO_LINK_CHANGE` reader - 1:0\\]
MDIO link change interrupt masked value"]
pub type MdioLinkChangeR = crate::FieldReader;
#[doc = "Field `MDIO_LINK_CHANGE` writer - 1:0\\]
MDIO link change interrupt masked value"]
pub type MdioLinkChangeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO link change interrupt masked value"]
    #[inline(always)]
    pub fn mdio_link_change(&self) -> MdioLinkChangeR {
        MdioLinkChangeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO link change interrupt masked value"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_link_change(&mut self) -> MdioLinkChangeW<MdioLinkIntMaskedRegSpec> {
        MdioLinkChangeW::new(self, 0)
    }
}
#[doc = "MDIO Link Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_link_int_masked_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_link_int_masked_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioLinkIntMaskedRegSpec;
impl crate::RegisterSpec for MdioLinkIntMaskedRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_link_int_masked_reg::R`](R) reader structure"]
impl crate::Readable for MdioLinkIntMaskedRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_link_int_masked_reg::W`](W) writer structure"]
impl crate::Writable for MdioLinkIntMaskedRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_LINK_INT_MASKED_REG to value 0"]
impl crate::Resettable for MdioLinkIntMaskedRegSpec {
    const RESET_VALUE: u32 = 0;
}
