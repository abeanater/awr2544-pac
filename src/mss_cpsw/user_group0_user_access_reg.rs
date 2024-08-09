#[doc = "Register `USER_GROUP0_USER_ACCESS_REG` reader"]
pub type R = crate::R<UserGroup0UserAccessRegSpec>;
#[doc = "Register `USER_GROUP0_USER_ACCESS_REG` writer"]
pub type W = crate::W<UserGroup0UserAccessRegSpec>;
#[doc = "Field `USER_DATA` reader - 15:0\\]
User data"]
pub type UserDataR = crate::FieldReader<u16>;
#[doc = "Field `USER_DATA` writer - 15:0\\]
User data"]
pub type UserDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_ADDRESS` reader - 20:16\\]
PHY address"]
pub type PhyAddressR = crate::FieldReader;
#[doc = "Field `PHY_ADDRESS` writer - 20:16\\]
PHY address"]
pub type PhyAddressW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REGISTER_ADDRESS` reader - 25:21\\]
Register address"]
pub type RegisterAddressR = crate::FieldReader;
#[doc = "Field `REGISTER_ADDRESS` writer - 25:21\\]
Register address"]
pub type RegisterAddressW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ACKNOWLEDGE` reader - 29:29\\]
Acknowledge"]
pub type AcknowledgeR = crate::BitReader;
#[doc = "Field `ACKNOWLEDGE` writer - 29:29\\]
Acknowledge"]
pub type AcknowledgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - 30:30\\]
Write"]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - 30:30\\]
Write"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GO` reader - 31:31\\]
Go"]
pub type GoR = crate::BitReader;
#[doc = "Field `GO` writer - 31:31\\]
Go"]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
User data"]
    #[inline(always)]
    pub fn user_data(&self) -> UserDataR {
        UserDataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
PHY address"]
    #[inline(always)]
    pub fn phy_address(&self) -> PhyAddressR {
        PhyAddressR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Register address"]
    #[inline(always)]
    pub fn register_address(&self) -> RegisterAddressR {
        RegisterAddressR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Acknowledge"]
    #[inline(always)]
    pub fn acknowledge(&self) -> AcknowledgeR {
        AcknowledgeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Write"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Go"]
    #[inline(always)]
    pub fn go(&self) -> GoR {
        GoR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
User data"]
    #[inline(always)]
    #[must_use]
    pub fn user_data(&mut self) -> UserDataW<UserGroup0UserAccessRegSpec> {
        UserDataW::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
PHY address"]
    #[inline(always)]
    #[must_use]
    pub fn phy_address(&mut self) -> PhyAddressW<UserGroup0UserAccessRegSpec> {
        PhyAddressW::new(self, 16)
    }
    #[doc = "Bits 21:25 - 25:21\\]
Register address"]
    #[inline(always)]
    #[must_use]
    pub fn register_address(&mut self) -> RegisterAddressW<UserGroup0UserAccessRegSpec> {
        RegisterAddressW::new(self, 21)
    }
    #[doc = "Bit 29 - 29:29\\]
Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn acknowledge(&mut self) -> AcknowledgeW<UserGroup0UserAccessRegSpec> {
        AcknowledgeW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Write"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<UserGroup0UserAccessRegSpec> {
        WriteW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Go"]
    #[inline(always)]
    #[must_use]
    pub fn go(&mut self) -> GoW<UserGroup0UserAccessRegSpec> {
        GoW::new(self, 31)
    }
}
#[doc = "MDIO User Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user_group0_user_access_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_group0_user_access_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserGroup0UserAccessRegSpec;
impl crate::RegisterSpec for UserGroup0UserAccessRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user_group0_user_access_reg::R`](R) reader structure"]
impl crate::Readable for UserGroup0UserAccessRegSpec {}
#[doc = "`write(|w| ..)` method takes [`user_group0_user_access_reg::W`](W) writer structure"]
impl crate::Writable for UserGroup0UserAccessRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USER_GROUP0_USER_ACCESS_REG to value 0"]
impl crate::Resettable for UserGroup0UserAccessRegSpec {
    const RESET_VALUE: u32 = 0;
}
