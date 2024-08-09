#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_PORT_CONFIG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacPortConfigSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_PORT_CONFIG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacPortConfigSpec>;
#[doc = "Field `THE_NUMBER_OF` reader - 7:0\\]
The number of InterVLAN routes"]
pub type TheNumberOfR = crate::FieldReader;
#[doc = "Field `THE_NUMBER_OF` writer - 7:0\\]
The number of InterVLAN routes"]
pub type TheNumberOfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NO_XGMII_SUPPORT` reader - 8:8\\]
No XGMII support"]
pub type NoXgmiiSupportR = crate::BitReader;
#[doc = "Field `NO_XGMII_SUPPORT` writer - 8:8\\]
No XGMII support"]
pub type NoXgmiiSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IET_SUPPORT` reader - 9:9\\]
IET support"]
pub type IetSupportR = crate::BitReader;
#[doc = "Field `IET_SUPPORT` writer - 9:9\\]
IET support"]
pub type IetSupportW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The number of InterVLAN routes"]
    #[inline(always)]
    pub fn the_number_of(&self) -> TheNumberOfR {
        TheNumberOfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
No XGMII support"]
    #[inline(always)]
    pub fn no_xgmii_support(&self) -> NoXgmiiSupportR {
        NoXgmiiSupportR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
IET support"]
    #[inline(always)]
    pub fn iet_support(&self) -> IetSupportR {
        IetSupportR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The number of InterVLAN routes"]
    #[inline(always)]
    #[must_use]
    pub fn the_number_of(&mut self) -> TheNumberOfW<CpswNcEthMac0PnMacPortConfigSpec> {
        TheNumberOfW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
No XGMII support"]
    #[inline(always)]
    #[must_use]
    pub fn no_xgmii_support(&mut self) -> NoXgmiiSupportW<CpswNcEthMac0PnMacPortConfigSpec> {
        NoXgmiiSupportW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
IET support"]
    #[inline(always)]
    #[must_use]
    pub fn iet_support(&mut self) -> IetSupportW<CpswNcEthMac0PnMacPortConfigSpec> {
        IetSupportW::new(self, 9)
    }
}
#[doc = "Enet Port N Port Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_port_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_port_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacPortConfigSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacPortConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_port_config::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacPortConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_port_config::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacPortConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_PORT_CONFIG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacPortConfigSpec {
    const RESET_VALUE: u32 = 0;
}
