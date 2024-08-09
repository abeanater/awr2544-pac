#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_CTL_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnTsCtlRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_CTL_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnTsCtlRegSpec>;
#[doc = "Field `TIME_SYNCE_RECEIVE_2` reader - 0:0\\]
Time Synce Receive Annex F Enable"]
pub type TimeSynceReceive2R = crate::BitReader;
#[doc = "Field `TIME_SYNCE_RECEIVE_2` writer - 0:0\\]
Time Synce Receive Annex F Enable"]
pub type TimeSynceReceive2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_RECEIVE_1` reader - 1:1\\]
Time Sync Receive VLAN LTYPE 1 enable"]
pub type TimeSyncReceive1R = crate::BitReader;
#[doc = "Field `TIME_SYNC_RECEIVE_1` writer - 1:1\\]
Time Sync Receive VLAN LTYPE 1 enable"]
pub type TimeSyncReceive1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_RECEIVE` reader - 2:2\\]
Time Sync Receive VLAN LTYPE 2 enable"]
pub type TimeSyncReceiveR = crate::BitReader;
#[doc = "Field `TIME_SYNC_RECEIVE` writer - 2:2\\]
Time Sync Receive VLAN LTYPE 2 enable"]
pub type TimeSyncReceiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNCE_RECEIVE_1` reader - 3:3\\]
Time Synce Receive Annex D Enable"]
pub type TimeSynceReceive1R = crate::BitReader;
#[doc = "Field `TIME_SYNCE_RECEIVE_1` writer - 3:3\\]
Time Synce Receive Annex D Enable"]
pub type TimeSynceReceive1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNCE_TRANSMIT_2` reader - 4:4\\]
Time Synce Transmit Annex F Enable"]
pub type TimeSynceTransmit2R = crate::BitReader;
#[doc = "Field `TIME_SYNCE_TRANSMIT_2` writer - 4:4\\]
Time Synce Transmit Annex F Enable"]
pub type TimeSynceTransmit2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_TRANSMIT_2` reader - 5:5\\]
Time Sync Transmit VLAN LTYPE 1 enable"]
pub type TimeSyncTransmit2R = crate::BitReader;
#[doc = "Field `TIME_SYNC_TRANSMIT_2` writer - 5:5\\]
Time Sync Transmit VLAN LTYPE 1 enable"]
pub type TimeSyncTransmit2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_TRANSMIT_1` reader - 6:6\\]
Time Sync Transmit VLAN LTYPE 2 enable"]
pub type TimeSyncTransmit1R = crate::BitReader;
#[doc = "Field `TIME_SYNC_TRANSMIT_1` writer - 6:6\\]
Time Sync Transmit VLAN LTYPE 2 enable"]
pub type TimeSyncTransmit1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNCE_TRANSMIT_1` reader - 7:7\\]
Time Synce Transmit Annex D Enable"]
pub type TimeSynceTransmit1R = crate::BitReader;
#[doc = "Field `TIME_SYNCE_TRANSMIT_1` writer - 7:7\\]
Time Synce Transmit Annex D Enable"]
pub type TimeSynceTransmit1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_LTYPE` reader - 8:8\\]
Time Sync LTYPE 2 enable transmit and receive"]
pub type TimeSyncLtypeR = crate::BitReader;
#[doc = "Field `TIME_SYNC_LTYPE` writer - 8:8\\]
Time Sync LTYPE 2 enable transmit and receive"]
pub type TimeSyncLtypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNCE_RECEIVE` reader - 9:9\\]
Time Synce Receive Annex E Enable"]
pub type TimeSynceReceiveR = crate::BitReader;
#[doc = "Field `TIME_SYNCE_RECEIVE` writer - 9:9\\]
Time Synce Receive Annex E Enable"]
pub type TimeSynceReceiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNCE_TRANSMIT` reader - 10:10\\]
Time Synce Transmit Annex E Enable"]
pub type TimeSynceTransmitR = crate::BitReader;
#[doc = "Field `TIME_SYNCE_TRANSMIT` writer - 10:10\\]
Time Synce Transmit Annex E Enable"]
pub type TimeSynceTransmitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_TRANSMIT` reader - 11:11\\]
Time Sync Transmit Host Time Stamp Enable"]
pub type TimeSyncTransmitR = crate::BitReader;
#[doc = "Field `TIME_SYNC_TRANSMIT` writer - 11:11\\]
Time Sync Transmit Host Time Stamp Enable"]
pub type TimeSyncTransmitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_SYNC_MESSAGE` reader - 31:16\\]
Time Sync Message Type Enable"]
pub type TimeSyncMessageR = crate::FieldReader<u16>;
#[doc = "Field `TIME_SYNC_MESSAGE` writer - 31:16\\]
Time Sync Message Type Enable"]
pub type TimeSyncMessageW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time Synce Receive Annex F Enable"]
    #[inline(always)]
    pub fn time_synce_receive_2(&self) -> TimeSynceReceive2R {
        TimeSynceReceive2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time Sync Receive VLAN LTYPE 1 enable"]
    #[inline(always)]
    pub fn time_sync_receive_1(&self) -> TimeSyncReceive1R {
        TimeSyncReceive1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Time Sync Receive VLAN LTYPE 2 enable"]
    #[inline(always)]
    pub fn time_sync_receive(&self) -> TimeSyncReceiveR {
        TimeSyncReceiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Time Synce Receive Annex D Enable"]
    #[inline(always)]
    pub fn time_synce_receive_1(&self) -> TimeSynceReceive1R {
        TimeSynceReceive1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Time Synce Transmit Annex F Enable"]
    #[inline(always)]
    pub fn time_synce_transmit_2(&self) -> TimeSynceTransmit2R {
        TimeSynceTransmit2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Time Sync Transmit VLAN LTYPE 1 enable"]
    #[inline(always)]
    pub fn time_sync_transmit_2(&self) -> TimeSyncTransmit2R {
        TimeSyncTransmit2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Time Sync Transmit VLAN LTYPE 2 enable"]
    #[inline(always)]
    pub fn time_sync_transmit_1(&self) -> TimeSyncTransmit1R {
        TimeSyncTransmit1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Time Synce Transmit Annex D Enable"]
    #[inline(always)]
    pub fn time_synce_transmit_1(&self) -> TimeSynceTransmit1R {
        TimeSynceTransmit1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Time Sync LTYPE 2 enable transmit and receive"]
    #[inline(always)]
    pub fn time_sync_ltype(&self) -> TimeSyncLtypeR {
        TimeSyncLtypeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Time Synce Receive Annex E Enable"]
    #[inline(always)]
    pub fn time_synce_receive(&self) -> TimeSynceReceiveR {
        TimeSynceReceiveR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Time Synce Transmit Annex E Enable"]
    #[inline(always)]
    pub fn time_synce_transmit(&self) -> TimeSynceTransmitR {
        TimeSynceTransmitR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Time Sync Transmit Host Time Stamp Enable"]
    #[inline(always)]
    pub fn time_sync_transmit(&self) -> TimeSyncTransmitR {
        TimeSyncTransmitR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Time Sync Message Type Enable"]
    #[inline(always)]
    pub fn time_sync_message(&self) -> TimeSyncMessageR {
        TimeSyncMessageR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time Synce Receive Annex F Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_synce_receive_2(&mut self) -> TimeSynceReceive2W<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSynceReceive2W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time Sync Receive VLAN LTYPE 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_receive_1(&mut self) -> TimeSyncReceive1W<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSyncReceive1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Time Sync Receive VLAN LTYPE 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_receive(&mut self) -> TimeSyncReceiveW<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSyncReceiveW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Time Synce Receive Annex D Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_synce_receive_1(&mut self) -> TimeSynceReceive1W<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSynceReceive1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Time Synce Transmit Annex F Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_synce_transmit_2(&mut self) -> TimeSynceTransmit2W<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSynceTransmit2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Time Sync Transmit VLAN LTYPE 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_transmit_2(&mut self) -> TimeSyncTransmit2W<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSyncTransmit2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Time Sync Transmit VLAN LTYPE 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_transmit_1(&mut self) -> TimeSyncTransmit1W<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSyncTransmit1W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Time Synce Transmit Annex D Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_synce_transmit_1(&mut self) -> TimeSynceTransmit1W<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSynceTransmit1W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Time Sync LTYPE 2 enable transmit and receive"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_ltype(&mut self) -> TimeSyncLtypeW<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSyncLtypeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Time Synce Receive Annex E Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_synce_receive(&mut self) -> TimeSynceReceiveW<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSynceReceiveW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Time Synce Transmit Annex E Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_synce_transmit(&mut self) -> TimeSynceTransmitW<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSynceTransmitW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Time Sync Transmit Host Time Stamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_transmit(&mut self) -> TimeSyncTransmitW<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSyncTransmitW::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Time Sync Message Type Enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_message(&mut self) -> TimeSyncMessageW<CpswNcEthMac0PnTsCtlRegSpec> {
        TimeSyncMessageW::new(self, 16)
    }
}
#[doc = "Enet Port N Time Sync Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_ctl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_ctl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnTsCtlRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnTsCtlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_ts_ctl_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnTsCtlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_ts_ctl_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnTsCtlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_TS_CTL_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnTsCtlRegSpec {
    const RESET_VALUE: u32 = 0;
}
