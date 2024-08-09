#[doc = "Register `ALE_POLICECFG3` reader"]
pub type R = crate::R<AlePolicecfg3Spec>;
#[doc = "Register `ALE_POLICECFG3` writer"]
pub type W = crate::W<AlePolicecfg3Spec>;
#[doc = "Field `IP_SOURCE_ADDRESS_1` reader - 4:0\\]
IP Source Address Table Entry Index - Specifies the ALE IP Source address lookup table index to match for the selected policing/classifier entry"]
pub type IpSourceAddress1R = crate::FieldReader;
#[doc = "Field `IP_SOURCE_ADDRESS_1` writer - 4:0\\]
IP Source Address Table Entry Index - Specifies the ALE IP Source address lookup table index to match for the selected policing/classifier entry"]
pub type IpSourceAddress1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IP_SOURCE_ADDRESS` reader - 15:15\\]
IP Source Address Match Enable - Enables frame IP Source address match for the selected policing/classifier entry"]
pub type IpSourceAddressR = crate::BitReader;
#[doc = "Field `IP_SOURCE_ADDRESS` writer - 15:15\\]
IP Source Address Match Enable - Enables frame IP Source address match for the selected policing/classifier entry"]
pub type IpSourceAddressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHERTYPE_TABLE_ENTRY` reader - 20:16\\]
EtherType Table Entry Index - Specifies the ALE Ether Type lookup table index to match for the selected policing/classifier entry"]
pub type EthertypeTableEntryR = crate::FieldReader;
#[doc = "Field `ETHERTYPE_TABLE_ENTRY` writer - 20:16\\]
EtherType Table Entry Index - Specifies the ALE Ether Type lookup table index to match for the selected policing/classifier entry"]
pub type EthertypeTableEntryW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ETHERTYPE_MATCH_ENABLE` reader - 31:31\\]
EtherType Match Enable - Enables frame Ether Type match for the selected policing/classifier entry"]
pub type EthertypeMatchEnableR = crate::BitReader;
#[doc = "Field `ETHERTYPE_MATCH_ENABLE` writer - 31:31\\]
EtherType Match Enable - Enables frame Ether Type match for the selected policing/classifier entry"]
pub type EthertypeMatchEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
IP Source Address Table Entry Index - Specifies the ALE IP Source address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn ip_source_address_1(&self) -> IpSourceAddress1R {
        IpSourceAddress1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
IP Source Address Match Enable - Enables frame IP Source address match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn ip_source_address(&self) -> IpSourceAddressR {
        IpSourceAddressR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
EtherType Table Entry Index - Specifies the ALE Ether Type lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn ethertype_table_entry(&self) -> EthertypeTableEntryR {
        EthertypeTableEntryR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
EtherType Match Enable - Enables frame Ether Type match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn ethertype_match_enable(&self) -> EthertypeMatchEnableR {
        EthertypeMatchEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
IP Source Address Table Entry Index - Specifies the ALE IP Source address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn ip_source_address_1(&mut self) -> IpSourceAddress1W<AlePolicecfg3Spec> {
        IpSourceAddress1W::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
IP Source Address Match Enable - Enables frame IP Source address match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn ip_source_address(&mut self) -> IpSourceAddressW<AlePolicecfg3Spec> {
        IpSourceAddressW::new(self, 15)
    }
    #[doc = "Bits 16:20 - 20:16\\]
EtherType Table Entry Index - Specifies the ALE Ether Type lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn ethertype_table_entry(&mut self) -> EthertypeTableEntryW<AlePolicecfg3Spec> {
        EthertypeTableEntryW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
EtherType Match Enable - Enables frame Ether Type match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn ethertype_match_enable(&mut self) -> EthertypeMatchEnableW<AlePolicecfg3Spec> {
        EthertypeMatchEnableW::new(self, 31)
    }
}
#[doc = "The Policing Config 3 holds the match enable/match index for the Ether Type and IP Source address\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicecfg3Spec;
impl crate::RegisterSpec for AlePolicecfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policecfg3::R`](R) reader structure"]
impl crate::Readable for AlePolicecfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_policecfg3::W`](W) writer structure"]
impl crate::Writable for AlePolicecfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICECFG3 to value 0"]
impl crate::Resettable for AlePolicecfg3Spec {
    const RESET_VALUE: u32 = 0;
}
