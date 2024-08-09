#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_VLAN_LTYPE_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnTsVlanLtypeRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_VLAN_LTYPE_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnTsVlanLtypeRegSpec>;
#[doc = "Field `TIME_SYNC_VLAN` reader - 15:0\\]
Time Sync VLAN LTYPE1"]
pub type TimeSyncVlanR = crate::FieldReader<u16>;
#[doc = "Field `TIME_SYNC_VLAN` writer - 15:0\\]
Time Sync VLAN LTYPE1"]
pub type TimeSyncVlanW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIME_SYNC_VLAN` reader - 31:16\\]
Time Sync VLAN LTYPE2"]
pub type TimeSyncVlanR = crate::FieldReader<u16>;
#[doc = "Field `TIME_SYNC_VLAN` writer - 31:16\\]
Time Sync VLAN LTYPE2"]
pub type TimeSyncVlanW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Time Sync VLAN LTYPE1"]
    #[inline(always)]
    pub fn time_sync_vlan(&self) -> TimeSyncVlanR {
        TimeSyncVlanR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Time Sync VLAN LTYPE2"]
    #[inline(always)]
    pub fn time_sync_vlan(&self) -> TimeSyncVlanR {
        TimeSyncVlanR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Time Sync VLAN LTYPE1"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_vlan(&mut self) -> TimeSyncVlanW<CpswNcEthMac0PnTsVlanLtypeRegSpec> {
        TimeSyncVlanW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Time Sync VLAN LTYPE2"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_vlan(&mut self) -> TimeSyncVlanW<CpswNcEthMac0PnTsVlanLtypeRegSpec> {
        TimeSyncVlanW::new(self, 16)
    }
}
#[doc = "Enet Port N Time Sync VLAN2 and VLAN2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnTsVlanLtypeRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnTsVlanLtypeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnTsVlanLtypeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_ts_vlan_ltype_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnTsVlanLtypeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_TS_VLAN_LTYPE_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnTsVlanLtypeRegSpec {
    const RESET_VALUE: u32 = 0;
}
