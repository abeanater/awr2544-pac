#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_CTL2_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnTsCtl2RegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_CTL2_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnTsCtl2RegSpec>;
#[doc = "Field `TIME_SYNC_MULTICAST` reader - 15:0\\]
Time Sync Multicast Destination Address Type Enable"]
pub type TimeSyncMulticastR = crate::FieldReader<u16>;
#[doc = "Field `TIME_SYNC_MULTICAST` writer - 15:0\\]
Time Sync Multicast Destination Address Type Enable"]
pub type TimeSyncMulticastW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIME_SYNC_DOMAIN` reader - 21:16\\]
Time Sync Domain Offset"]
pub type TimeSyncDomainR = crate::FieldReader;
#[doc = "Field `TIME_SYNC_DOMAIN` writer - 21:16\\]
Time Sync Domain Offset"]
pub type TimeSyncDomainW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Time Sync Multicast Destination Address Type Enable"]
    #[inline(always)]
    pub fn time_sync_multicast(&self) -> TimeSyncMulticastR {
        TimeSyncMulticastR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Time Sync Domain Offset"]
    #[inline(always)]
    pub fn time_sync_domain(&self) -> TimeSyncDomainR {
        TimeSyncDomainR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Time Sync Multicast Destination Address Type Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_multicast(&mut self) -> TimeSyncMulticastW<CpswNcEthMac0PnTsCtl2RegSpec> {
        TimeSyncMulticastW::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Time Sync Domain Offset"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_domain(&mut self) -> TimeSyncDomainW<CpswNcEthMac0PnTsCtl2RegSpec> {
        TimeSyncDomainW::new(self, 16)
    }
}
#[doc = "Enet Port N Time Sync Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_ctl2_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_ctl2_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnTsCtl2RegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnTsCtl2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_ts_ctl2_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnTsCtl2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_ts_ctl2_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnTsCtl2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_TS_CTL2_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnTsCtl2RegSpec {
    const RESET_VALUE: u32 = 0;
}
