#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_SA_L_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnSaLRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_SA_L_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnSaLRegSpec>;
#[doc = "Field `SOURCE_ADDRESS_BITS` reader - 7:0\\]
Source Address bits 15:8"]
pub type SourceAddressBitsR = crate::FieldReader;
#[doc = "Field `SOURCE_ADDRESS_BITS` writer - 7:0\\]
Source Address bits 15:8"]
pub type SourceAddressBitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOURCE_ADDRESS_LOWER` reader - 15:8\\]
Source Address Lower 8 bits"]
pub type SourceAddressLowerR = crate::FieldReader;
#[doc = "Field `SOURCE_ADDRESS_LOWER` writer - 15:8\\]
Source Address Lower 8 bits"]
pub type SourceAddressLowerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Source Address bits 15:8"]
    #[inline(always)]
    pub fn source_address_bits(&self) -> SourceAddressBitsR {
        SourceAddressBitsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Source Address Lower 8 bits"]
    #[inline(always)]
    pub fn source_address_lower(&self) -> SourceAddressLowerR {
        SourceAddressLowerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Source Address bits 15:8"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_bits(&mut self) -> SourceAddressBitsW<CpswNcEthMac0PnSaLRegSpec> {
        SourceAddressBitsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Source Address Lower 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_lower(&mut self) -> SourceAddressLowerW<CpswNcEthMac0PnSaLRegSpec> {
        SourceAddressLowerW::new(self, 8)
    }
}
#[doc = "Enet Port N Tx Pause Frame Source Address Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_sa_l_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_sa_l_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnSaLRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnSaLRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_sa_l_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnSaLRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_sa_l_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnSaLRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_SA_L_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnSaLRegSpec {
    const RESET_VALUE: u32 = 0;
}
