#[doc = "Register `MDIO_LINK_REG` reader"]
pub type R = crate::R<MdioLinkRegSpec>;
#[doc = "Register `MDIO_LINK_REG` writer"]
pub type W = crate::W<MdioLinkRegSpec>;
#[doc = "Field `MDIO_LINK_STATE` reader - 31:0\\]
MDIO link state"]
pub type MdioLinkStateR = crate::FieldReader<u32>;
#[doc = "Field `MDIO_LINK_STATE` writer - 31:0\\]
MDIO link state"]
pub type MdioLinkStateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO link state"]
    #[inline(always)]
    pub fn mdio_link_state(&self) -> MdioLinkStateR {
        MdioLinkStateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO link state"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_link_state(&mut self) -> MdioLinkStateW<MdioLinkRegSpec> {
        MdioLinkStateW::new(self, 0)
    }
}
#[doc = "MDIO Link Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_link_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_link_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioLinkRegSpec;
impl crate::RegisterSpec for MdioLinkRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_link_reg::R`](R) reader structure"]
impl crate::Readable for MdioLinkRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_link_reg::W`](W) writer structure"]
impl crate::Writable for MdioLinkRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_LINK_REG to value 0"]
impl crate::Resettable for MdioLinkRegSpec {
    const RESET_VALUE: u32 = 0;
}
