#[doc = "Register `ALE_POLICECFG1` reader"]
pub type R = crate::R<AlePolicecfg1Spec>;
#[doc = "Register `ALE_POLICECFG1` writer"]
pub type W = crate::W<AlePolicecfg1Spec>;
#[doc = "Field `SOURCE_ADDRESS_TABLE` reader - 4:0\\]
Source Address Table Entry Index - Specifies the ALE L2 source address lookup table index to match for the selected policing/classifier entry"]
pub type SourceAddressTableR = crate::FieldReader;
#[doc = "Field `SOURCE_ADDRESS_TABLE` writer - 4:0\\]
Source Address Table Entry Index - Specifies the ALE L2 source address lookup table index to match for the selected policing/classifier entry"]
pub type SourceAddressTableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SOURCE_ADDRESS_MATCH` reader - 15:15\\]
Source Address Match Enable - Enables frame L2 source address match for the selected policing/classifier entry"]
pub type SourceAddressMatchR = crate::BitReader;
#[doc = "Field `SOURCE_ADDRESS_MATCH` writer - 15:15\\]
Source Address Match Enable - Enables frame L2 source address match for the selected policing/classifier entry"]
pub type SourceAddressMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESTINATION_ADDRESS_TABLE` reader - 20:16\\]
Destination Address Table Entry Index - Specifies the ALE L2 destination address lookup table index to match for the selected policing/classifier entry"]
pub type DestinationAddressTableR = crate::FieldReader;
#[doc = "Field `DESTINATION_ADDRESS_TABLE` writer - 20:16\\]
Destination Address Table Entry Index - Specifies the ALE L2 destination address lookup table index to match for the selected policing/classifier entry"]
pub type DestinationAddressTableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DESTINATION_ADDRESS_MATCH` reader - 31:31\\]
Destination Address Match Enable - Enables frame L2 destination address match for the selected policing/classifier entry"]
pub type DestinationAddressMatchR = crate::BitReader;
#[doc = "Field `DESTINATION_ADDRESS_MATCH` writer - 31:31\\]
Destination Address Match Enable - Enables frame L2 destination address match for the selected policing/classifier entry"]
pub type DestinationAddressMatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Source Address Table Entry Index - Specifies the ALE L2 source address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn source_address_table(&self) -> SourceAddressTableR {
        SourceAddressTableR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Source Address Match Enable - Enables frame L2 source address match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn source_address_match(&self) -> SourceAddressMatchR {
        SourceAddressMatchR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Destination Address Table Entry Index - Specifies the ALE L2 destination address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn destination_address_table(&self) -> DestinationAddressTableR {
        DestinationAddressTableR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Destination Address Match Enable - Enables frame L2 destination address match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn destination_address_match(&self) -> DestinationAddressMatchR {
        DestinationAddressMatchR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Source Address Table Entry Index - Specifies the ALE L2 source address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_table(&mut self) -> SourceAddressTableW<AlePolicecfg1Spec> {
        SourceAddressTableW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Source Address Match Enable - Enables frame L2 source address match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_match(&mut self) -> SourceAddressMatchW<AlePolicecfg1Spec> {
        SourceAddressMatchW::new(self, 15)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Destination Address Table Entry Index - Specifies the ALE L2 destination address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_table(&mut self) -> DestinationAddressTableW<AlePolicecfg1Spec> {
        DestinationAddressTableW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Destination Address Match Enable - Enables frame L2 destination address match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_match(&mut self) -> DestinationAddressMatchW<AlePolicecfg1Spec> {
        DestinationAddressMatchW::new(self, 31)
    }
}
#[doc = "The Policing Config 1 holds the match enable/match index for the L2 Destination and L2 source addresses\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicecfg1Spec;
impl crate::RegisterSpec for AlePolicecfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policecfg1::R`](R) reader structure"]
impl crate::Readable for AlePolicecfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_policecfg1::W`](W) writer structure"]
impl crate::Writable for AlePolicecfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICECFG1 to value 0"]
impl crate::Resettable for AlePolicecfg1Spec {
    const RESET_VALUE: u32 = 0;
}
