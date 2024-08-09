#[doc = "Register `USER_GROUP0_USER_PHY_SEL_REG` reader"]
pub type R = crate::R<UserGroup0UserPhySelRegSpec>;
#[doc = "Register `USER_GROUP0_USER_PHY_SEL_REG` writer"]
pub type W = crate::W<UserGroup0UserPhySelRegSpec>;
#[doc = "Field `PHY_ADDRESS_WHOSE` reader - 4:0\\]
PHY address whose link status is monitored"]
pub type PhyAddressWhoseR = crate::FieldReader;
#[doc = "Field `PHY_ADDRESS_WHOSE` writer - 4:0\\]
PHY address whose link status is monitored"]
pub type PhyAddressWhoseW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LINK_CHANGE_INTERRUPT` reader - 6:6\\]
Link change interrupt enable"]
pub type LinkChangeInterruptR = crate::BitReader;
#[doc = "Field `LINK_CHANGE_INTERRUPT` writer - 6:6\\]
Link change interrupt enable"]
pub type LinkChangeInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINK_STATUS_DETERMINATION` reader - 7:7\\]
Link status determination select"]
pub type LinkStatusDeterminationR = crate::BitReader;
#[doc = "Field `LINK_STATUS_DETERMINATION` writer - 7:7\\]
Link status determination select"]
pub type LinkStatusDeterminationW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
PHY address whose link status is monitored"]
    #[inline(always)]
    pub fn phy_address_whose(&self) -> PhyAddressWhoseR {
        PhyAddressWhoseR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Link change interrupt enable"]
    #[inline(always)]
    pub fn link_change_interrupt(&self) -> LinkChangeInterruptR {
        LinkChangeInterruptR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Link status determination select"]
    #[inline(always)]
    pub fn link_status_determination(&self) -> LinkStatusDeterminationR {
        LinkStatusDeterminationR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
PHY address whose link status is monitored"]
    #[inline(always)]
    #[must_use]
    pub fn phy_address_whose(&mut self) -> PhyAddressWhoseW<UserGroup0UserPhySelRegSpec> {
        PhyAddressWhoseW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Link change interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn link_change_interrupt(&mut self) -> LinkChangeInterruptW<UserGroup0UserPhySelRegSpec> {
        LinkChangeInterruptW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Link status determination select"]
    #[inline(always)]
    #[must_use]
    pub fn link_status_determination(
        &mut self,
    ) -> LinkStatusDeterminationW<UserGroup0UserPhySelRegSpec> {
        LinkStatusDeterminationW::new(self, 7)
    }
}
#[doc = "MDIO User PHY Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user_group0_user_phy_sel_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_group0_user_phy_sel_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserGroup0UserPhySelRegSpec;
impl crate::RegisterSpec for UserGroup0UserPhySelRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user_group0_user_phy_sel_reg::R`](R) reader structure"]
impl crate::Readable for UserGroup0UserPhySelRegSpec {}
#[doc = "`write(|w| ..)` method takes [`user_group0_user_phy_sel_reg::W`](W) writer structure"]
impl crate::Writable for UserGroup0UserPhySelRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USER_GROUP0_USER_PHY_SEL_REG to value 0"]
impl crate::Resettable for UserGroup0UserPhySelRegSpec {
    const RESET_VALUE: u32 = 0;
}
