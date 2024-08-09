#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_C_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnIntervlanOpxCRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_C_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnIntervlanOpxCRegSpec>;
#[doc = "Field `SOURCE_ADDRESS_BITS_3` reader - 7:0\\]
Source Address bits 31:24"]
pub type SourceAddressBits3R = crate::FieldReader;
#[doc = "Field `SOURCE_ADDRESS_BITS_3` writer - 7:0\\]
Source Address bits 31:24"]
pub type SourceAddressBits3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOURCE_ADDRESS_BITS_2` reader - 15:8\\]
Source Address bits 23:16"]
pub type SourceAddressBits2R = crate::FieldReader;
#[doc = "Field `SOURCE_ADDRESS_BITS_2` writer - 15:8\\]
Source Address bits 23:16"]
pub type SourceAddressBits2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOURCE_ADDRESS_BITS_1` reader - 23:16\\]
Source Address bits 15:8"]
pub type SourceAddressBits1R = crate::FieldReader;
#[doc = "Field `SOURCE_ADDRESS_BITS_1` writer - 23:16\\]
Source Address bits 15:8"]
pub type SourceAddressBits1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOURCE_ADDRESS_BITS` reader - 31:24\\]
Source Address bits 7:0"]
pub type SourceAddressBitsR = crate::FieldReader;
#[doc = "Field `SOURCE_ADDRESS_BITS` writer - 31:24\\]
Source Address bits 7:0"]
pub type SourceAddressBitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Source Address bits 31:24"]
    #[inline(always)]
    pub fn source_address_bits_3(&self) -> SourceAddressBits3R {
        SourceAddressBits3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Source Address bits 23:16"]
    #[inline(always)]
    pub fn source_address_bits_2(&self) -> SourceAddressBits2R {
        SourceAddressBits2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Source Address bits 15:8"]
    #[inline(always)]
    pub fn source_address_bits_1(&self) -> SourceAddressBits1R {
        SourceAddressBits1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Source Address bits 7:0"]
    #[inline(always)]
    pub fn source_address_bits(&self) -> SourceAddressBitsR {
        SourceAddressBitsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Source Address bits 31:24"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_bits_3(
        &mut self,
    ) -> SourceAddressBits3W<CpswNcEthMac0PnIntervlanOpxCRegSpec> {
        SourceAddressBits3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Source Address bits 23:16"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_bits_2(
        &mut self,
    ) -> SourceAddressBits2W<CpswNcEthMac0PnIntervlanOpxCRegSpec> {
        SourceAddressBits2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Source Address bits 15:8"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_bits_1(
        &mut self,
    ) -> SourceAddressBits1W<CpswNcEthMac0PnIntervlanOpxCRegSpec> {
        SourceAddressBits1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Source Address bits 7:0"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_bits(
        &mut self,
    ) -> SourceAddressBitsW<CpswNcEthMac0PnIntervlanOpxCRegSpec> {
        SourceAddressBitsW::new(self, 24)
    }
}
#[doc = "Enet Port N Tx Egress InterVLAN C\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnIntervlanOpxCRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnIntervlanOpxCRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnIntervlanOpxCRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_intervlan_opx_c_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnIntervlanOpxCRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_C_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnIntervlanOpxCRegSpec {
    const RESET_VALUE: u32 = 0;
}
