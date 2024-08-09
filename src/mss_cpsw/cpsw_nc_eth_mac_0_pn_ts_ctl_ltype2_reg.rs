#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_CTL_LTYPE2_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnTsCtlLtype2RegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_CTL_LTYPE2_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnTsCtlLtype2RegSpec>;
#[doc = "Field `TIME_SYNC_LTYPE2` reader - 15:0\\]
Time Sync LTYPE2"]
pub type TimeSyncLtype2R = crate::FieldReader<u16>;
#[doc = "Field `TIME_SYNC_LTYPE2` writer - 15:0\\]
Time Sync LTYPE2"]
pub type TimeSyncLtype2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIME_SYNC_DESTINATION` reader - 16:16\\]
Time Sync Destination IP Address 107 Enable"]
pub type TimeSyncDestinationR = crate::BitReader;
#[doc = "Field `TIME_SYNC_DESTINATION` writer - 16:16\\]
Time Sync Destination IP Address 107 Enable"]
pub type TimeSyncDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_DESTINATION` reader - 17:17\\]
Time Sync Destination IP Address 129 Enable"]
pub type TimeSyncDestinationR = crate::BitReader;
#[doc = "Field `TIME_SYNC_DESTINATION` writer - 17:17\\]
Time Sync Destination IP Address 129 Enable"]
pub type TimeSyncDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_DESTINATION` reader - 18:18\\]
Time Sync Destination IP Address 130 Enable"]
pub type TimeSyncDestinationR = crate::BitReader;
#[doc = "Field `TIME_SYNC_DESTINATION` writer - 18:18\\]
Time Sync Destination IP Address 130 Enable"]
pub type TimeSyncDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_DESTINATION` reader - 19:19\\]
Time Sync Destination IP Address 131 Enable"]
pub type TimeSyncDestinationR = crate::BitReader;
#[doc = "Field `TIME_SYNC_DESTINATION` writer - 19:19\\]
Time Sync Destination IP Address 131 Enable"]
pub type TimeSyncDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_DESTINATION` reader - 20:20\\]
Time Sync Destination IP Address 132 Enable"]
pub type TimeSyncDestinationR = crate::BitReader;
#[doc = "Field `TIME_SYNC_DESTINATION` writer - 20:20\\]
Time Sync Destination IP Address 132 Enable"]
pub type TimeSyncDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_DESTINATION` reader - 21:21\\]
Time Sync Destination IP Address 319 Enable"]
pub type TimeSyncDestinationR = crate::BitReader;
#[doc = "Field `TIME_SYNC_DESTINATION` writer - 21:21\\]
Time Sync Destination IP Address 319 Enable"]
pub type TimeSyncDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_DESTINATION` reader - 22:22\\]
Time Sync Destination IP Address 320 Enable"]
pub type TimeSyncDestinationR = crate::BitReader;
#[doc = "Field `TIME_SYNC_DESTINATION` writer - 22:22\\]
Time Sync Destination IP Address 320 Enable"]
pub type TimeSyncDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_TIME` reader - 23:23\\]
Time Sync Time to Live Non-zero Enable"]
pub type TimeSyncTimeR = crate::BitReader;
#[doc = "Field `TIME_SYNC_TIME` writer - 23:23\\]
Time Sync Time to Live Non-zero Enable"]
pub type TimeSyncTimeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_UNICAST` reader - 24:24\\]
Time Sync Unicast Enable"]
pub type TimeSyncUnicastR = crate::BitReader;
#[doc = "Field `TIME_SYNC_UNICAST` writer - 24:24\\]
Time Sync Unicast Enable"]
pub type TimeSyncUnicastW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Time Sync LTYPE2"]
    #[inline(always)]
    pub fn time_sync_ltype2(&self) -> TimeSyncLtype2R {
        TimeSyncLtype2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Time Sync Destination IP Address 107 Enable"]
    #[inline(always)]
    pub fn time_sync_destination(&self) -> TimeSyncDestinationR {
        TimeSyncDestinationR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Time Sync Destination IP Address 129 Enable"]
    #[inline(always)]
    pub fn time_sync_destination(&self) -> TimeSyncDestinationR {
        TimeSyncDestinationR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Time Sync Destination IP Address 130 Enable"]
    #[inline(always)]
    pub fn time_sync_destination(&self) -> TimeSyncDestinationR {
        TimeSyncDestinationR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Time Sync Destination IP Address 131 Enable"]
    #[inline(always)]
    pub fn time_sync_destination(&self) -> TimeSyncDestinationR {
        TimeSyncDestinationR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Time Sync Destination IP Address 132 Enable"]
    #[inline(always)]
    pub fn time_sync_destination(&self) -> TimeSyncDestinationR {
        TimeSyncDestinationR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Time Sync Destination IP Address 319 Enable"]
    #[inline(always)]
    pub fn time_sync_destination(&self) -> TimeSyncDestinationR {
        TimeSyncDestinationR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Time Sync Destination IP Address 320 Enable"]
    #[inline(always)]
    pub fn time_sync_destination(&self) -> TimeSyncDestinationR {
        TimeSyncDestinationR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Time Sync Time to Live Non-zero Enable"]
    #[inline(always)]
    pub fn time_sync_time(&self) -> TimeSyncTimeR {
        TimeSyncTimeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Time Sync Unicast Enable"]
    #[inline(always)]
    pub fn time_sync_unicast(&self) -> TimeSyncUnicastR {
        TimeSyncUnicastR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Time Sync LTYPE2"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_ltype2(&mut self) -> TimeSyncLtype2W<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncLtype2W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Time Sync Destination IP Address 107 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_destination(
        &mut self,
    ) -> TimeSyncDestinationW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncDestinationW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Time Sync Destination IP Address 129 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_destination(
        &mut self,
    ) -> TimeSyncDestinationW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncDestinationW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Time Sync Destination IP Address 130 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_destination(
        &mut self,
    ) -> TimeSyncDestinationW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncDestinationW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Time Sync Destination IP Address 131 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_destination(
        &mut self,
    ) -> TimeSyncDestinationW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncDestinationW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Time Sync Destination IP Address 132 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_destination(
        &mut self,
    ) -> TimeSyncDestinationW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncDestinationW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Time Sync Destination IP Address 319 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_destination(
        &mut self,
    ) -> TimeSyncDestinationW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncDestinationW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Time Sync Destination IP Address 320 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_destination(
        &mut self,
    ) -> TimeSyncDestinationW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncDestinationW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Time Sync Time to Live Non-zero Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_time(&mut self) -> TimeSyncTimeW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncTimeW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Time Sync Unicast Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_unicast(&mut self) -> TimeSyncUnicastW<CpswNcEthMac0PnTsCtlLtype2RegSpec> {
        TimeSyncUnicastW::new(self, 24)
    }
}
#[doc = "Enet Port N Time Sync Control and LTYPE 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnTsCtlLtype2RegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnTsCtlLtype2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnTsCtlLtype2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_ts_ctl_ltype2_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnTsCtlLtype2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_TS_CTL_LTYPE2_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnTsCtlLtype2RegSpec {
    const RESET_VALUE: u32 = 0;
}
