#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_EMCONTROL_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacEmcontrolRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_EMCONTROL_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacEmcontrolRegSpec>;
#[doc = "Field `EMULATION_FREE_BIT` reader - 0:0\\]
Emulation Free Bit"]
pub type EmulationFreeBitR = crate::BitReader;
#[doc = "Field `EMULATION_FREE_BIT` writer - 0:0\\]
Emulation Free Bit"]
pub type EmulationFreeBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMULATION_SOFT_BIT` reader - 1:1\\]
Emulation Soft Bit"]
pub type EmulationSoftBitR = crate::BitReader;
#[doc = "Field `EMULATION_SOFT_BIT` writer - 1:1\\]
Emulation Soft Bit"]
pub type EmulationSoftBitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Emulation Free Bit"]
    #[inline(always)]
    pub fn emulation_free_bit(&self) -> EmulationFreeBitR {
        EmulationFreeBitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Emulation Soft Bit"]
    #[inline(always)]
    pub fn emulation_soft_bit(&self) -> EmulationSoftBitR {
        EmulationSoftBitR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Emulation Free Bit"]
    #[inline(always)]
    #[must_use]
    pub fn emulation_free_bit(&mut self) -> EmulationFreeBitW<CpswNcEthMac0PnMacEmcontrolRegSpec> {
        EmulationFreeBitW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Emulation Soft Bit"]
    #[inline(always)]
    #[must_use]
    pub fn emulation_soft_bit(&mut self) -> EmulationSoftBitW<CpswNcEthMac0PnMacEmcontrolRegSpec> {
        EmulationSoftBitW::new(self, 1)
    }
}
#[doc = "Enet Port N Emulation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacEmcontrolRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacEmcontrolRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacEmcontrolRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_emcontrol_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacEmcontrolRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_EMCONTROL_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacEmcontrolRegSpec {
    const RESET_VALUE: u32 = 0;
}
