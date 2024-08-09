#[doc = "Register `MDIO_ALIVE_REG` reader"]
pub type R = crate::R<MdioAliveRegSpec>;
#[doc = "Register `MDIO_ALIVE_REG` writer"]
pub type W = crate::W<MdioAliveRegSpec>;
#[doc = "Field `MDIO_ALIVE` reader - 31:0\\]
MDIO alive"]
pub type MdioAliveR = crate::FieldReader<u32>;
#[doc = "Field `MDIO_ALIVE` writer - 31:0\\]
MDIO alive"]
pub type MdioAliveW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO alive"]
    #[inline(always)]
    pub fn mdio_alive(&self) -> MdioAliveR {
        MdioAliveR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO alive"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_alive(&mut self) -> MdioAliveW<MdioAliveRegSpec> {
        MdioAliveW::new(self, 0)
    }
}
#[doc = "MDIO Alive Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_alive_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_alive_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioAliveRegSpec;
impl crate::RegisterSpec for MdioAliveRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_alive_reg::R`](R) reader structure"]
impl crate::Readable for MdioAliveRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_alive_reg::W`](W) writer structure"]
impl crate::Writable for MdioAliveRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_ALIVE_REG to value 0"]
impl crate::Resettable for MdioAliveRegSpec {
    const RESET_VALUE: u32 = 0;
}
