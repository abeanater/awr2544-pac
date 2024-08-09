#[doc = "Register `ALE_POLICECFG4` reader"]
pub type R = crate::R<AlePolicecfg4Spec>;
#[doc = "Register `ALE_POLICECFG4` writer"]
pub type W = crate::W<AlePolicecfg4Spec>;
#[doc = "Field `IP_DESTINATION_ADDRESS_1` reader - 20:16\\]
IP Destination Address Table Entry Index - Specifies the ALE IP Destination address lookup table index to match for the selected policing/classifier entry"]
pub type IpDestinationAddress1R = crate::FieldReader;
#[doc = "Field `IP_DESTINATION_ADDRESS_1` writer - 20:16\\]
IP Destination Address Table Entry Index - Specifies the ALE IP Destination address lookup table index to match for the selected policing/classifier entry"]
pub type IpDestinationAddress1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IP_DESTINATION_ADDRESS` reader - 31:31\\]
IP Destination Address Match Enable - Enables frame IP Destination address match for the selected policing/classifier entry"]
pub type IpDestinationAddressR = crate::BitReader;
#[doc = "Field `IP_DESTINATION_ADDRESS` writer - 31:31\\]
IP Destination Address Match Enable - Enables frame IP Destination address match for the selected policing/classifier entry"]
pub type IpDestinationAddressW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:20 - 20:16\\]
IP Destination Address Table Entry Index - Specifies the ALE IP Destination address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn ip_destination_address_1(&self) -> IpDestinationAddress1R {
        IpDestinationAddress1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
IP Destination Address Match Enable - Enables frame IP Destination address match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn ip_destination_address(&self) -> IpDestinationAddressR {
        IpDestinationAddressR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - 20:16\\]
IP Destination Address Table Entry Index - Specifies the ALE IP Destination address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn ip_destination_address_1(&mut self) -> IpDestinationAddress1W<AlePolicecfg4Spec> {
        IpDestinationAddress1W::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
IP Destination Address Match Enable - Enables frame IP Destination address match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn ip_destination_address(&mut self) -> IpDestinationAddressW<AlePolicecfg4Spec> {
        IpDestinationAddressW::new(self, 31)
    }
}
#[doc = "The Policing Config 4 holds the match enable/match index for the IP Destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicecfg4Spec;
impl crate::RegisterSpec for AlePolicecfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policecfg4::R`](R) reader structure"]
impl crate::Readable for AlePolicecfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_policecfg4::W`](W) writer structure"]
impl crate::Writable for AlePolicecfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICECFG4 to value 0"]
impl crate::Resettable for AlePolicecfg4Spec {
    const RESET_VALUE: u32 = 0;
}
