#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_A_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnIntervlanOpxARegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_A_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnIntervlanOpxARegSpec>;
#[doc = "Field `DESTINATION_ADDRESS_BITS_3` reader - 7:0\\]
Destination Address bits 47:40"]
pub type DestinationAddressBits3R = crate::FieldReader;
#[doc = "Field `DESTINATION_ADDRESS_BITS_3` writer - 7:0\\]
Destination Address bits 47:40"]
pub type DestinationAddressBits3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DESTINATION_ADDRESS_BITS_2` reader - 15:8\\]
Destination Address bits 39:32"]
pub type DestinationAddressBits2R = crate::FieldReader;
#[doc = "Field `DESTINATION_ADDRESS_BITS_2` writer - 15:8\\]
Destination Address bits 39:32"]
pub type DestinationAddressBits2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DESTINATION_ADDRESS_BITS_1` reader - 23:16\\]
Destination Address bits 31:24"]
pub type DestinationAddressBits1R = crate::FieldReader;
#[doc = "Field `DESTINATION_ADDRESS_BITS_1` writer - 23:16\\]
Destination Address bits 31:24"]
pub type DestinationAddressBits1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DESTINATION_ADDRESS_BITS` reader - 31:24\\]
Destination Address bits 23:16"]
pub type DestinationAddressBitsR = crate::FieldReader;
#[doc = "Field `DESTINATION_ADDRESS_BITS` writer - 31:24\\]
Destination Address bits 23:16"]
pub type DestinationAddressBitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Destination Address bits 47:40"]
    #[inline(always)]
    pub fn destination_address_bits_3(&self) -> DestinationAddressBits3R {
        DestinationAddressBits3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Destination Address bits 39:32"]
    #[inline(always)]
    pub fn destination_address_bits_2(&self) -> DestinationAddressBits2R {
        DestinationAddressBits2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Destination Address bits 31:24"]
    #[inline(always)]
    pub fn destination_address_bits_1(&self) -> DestinationAddressBits1R {
        DestinationAddressBits1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Destination Address bits 23:16"]
    #[inline(always)]
    pub fn destination_address_bits(&self) -> DestinationAddressBitsR {
        DestinationAddressBitsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Destination Address bits 47:40"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_bits_3(
        &mut self,
    ) -> DestinationAddressBits3W<CpswNcEthMac0PnIntervlanOpxARegSpec> {
        DestinationAddressBits3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Destination Address bits 39:32"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_bits_2(
        &mut self,
    ) -> DestinationAddressBits2W<CpswNcEthMac0PnIntervlanOpxARegSpec> {
        DestinationAddressBits2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Destination Address bits 31:24"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_bits_1(
        &mut self,
    ) -> DestinationAddressBits1W<CpswNcEthMac0PnIntervlanOpxARegSpec> {
        DestinationAddressBits1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Destination Address bits 23:16"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_bits(
        &mut self,
    ) -> DestinationAddressBitsW<CpswNcEthMac0PnIntervlanOpxARegSpec> {
        DestinationAddressBitsW::new(self, 24)
    }
}
#[doc = "Enet Port N Tx Egress InterVLAN A\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnIntervlanOpxARegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnIntervlanOpxARegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnIntervlanOpxARegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_intervlan_opx_a_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnIntervlanOpxARegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_A_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnIntervlanOpxARegSpec {
    const RESET_VALUE: u32 = 0;
}
