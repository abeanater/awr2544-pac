#[doc = "Register `MDIO_USER_ADDR0_REG` reader"]
pub type R = crate::R<MdioUserAddr0RegSpec>;
#[doc = "Register `MDIO_USER_ADDR0_REG` writer"]
pub type W = crate::W<MdioUserAddr0RegSpec>;
#[doc = "Field `MDIO_USER_ADDRESS` reader - 15:0\\]
MDIO USER Address 0"]
pub type MdioUserAddressR = crate::FieldReader<u16>;
#[doc = "Field `MDIO_USER_ADDRESS` writer - 15:0\\]
MDIO USER Address 0"]
pub type MdioUserAddressW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
MDIO USER Address 0"]
    #[inline(always)]
    pub fn mdio_user_address(&self) -> MdioUserAddressR {
        MdioUserAddressR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
MDIO USER Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_user_address(&mut self) -> MdioUserAddressW<MdioUserAddr0RegSpec> {
        MdioUserAddressW::new(self, 0)
    }
}
#[doc = "MDIO Address 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_user_addr0_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_user_addr0_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioUserAddr0RegSpec;
impl crate::RegisterSpec for MdioUserAddr0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_user_addr0_reg::R`](R) reader structure"]
impl crate::Readable for MdioUserAddr0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_user_addr0_reg::W`](W) writer structure"]
impl crate::Writable for MdioUserAddr0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_USER_ADDR0_REG to value 0"]
impl crate::Resettable for MdioUserAddr0RegSpec {
    const RESET_VALUE: u32 = 0;
}
