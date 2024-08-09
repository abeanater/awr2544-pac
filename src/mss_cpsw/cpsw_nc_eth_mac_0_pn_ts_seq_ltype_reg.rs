#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_SEQ_LTYPE_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnTsSeqLtypeRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TS_SEQ_LTYPE_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnTsSeqLtypeRegSpec>;
#[doc = "Field `TIME_SYNC_LTYPE1` reader - 15:0\\]
Time Sync LTYPE1"]
pub type TimeSyncLtype1R = crate::FieldReader<u16>;
#[doc = "Field `TIME_SYNC_LTYPE1` writer - 15:0\\]
Time Sync LTYPE1"]
pub type TimeSyncLtype1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIME_SYNC_SEQUENCE` reader - 21:16\\]
Time Sync Sequence ID Offset"]
pub type TimeSyncSequenceR = crate::FieldReader;
#[doc = "Field `TIME_SYNC_SEQUENCE` writer - 21:16\\]
Time Sync Sequence ID Offset"]
pub type TimeSyncSequenceW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Time Sync LTYPE1"]
    #[inline(always)]
    pub fn time_sync_ltype1(&self) -> TimeSyncLtype1R {
        TimeSyncLtype1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Time Sync Sequence ID Offset"]
    #[inline(always)]
    pub fn time_sync_sequence(&self) -> TimeSyncSequenceR {
        TimeSyncSequenceR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Time Sync LTYPE1"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_ltype1(&mut self) -> TimeSyncLtype1W<CpswNcEthMac0PnTsSeqLtypeRegSpec> {
        TimeSyncLtype1W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Time Sync Sequence ID Offset"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_sequence(&mut self) -> TimeSyncSequenceW<CpswNcEthMac0PnTsSeqLtypeRegSpec> {
        TimeSyncSequenceW::new(self, 16)
    }
}
#[doc = "Enet Port N Time Sync LTYPE (and SEQ_ID_OFFSET)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnTsSeqLtypeRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnTsSeqLtypeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnTsSeqLtypeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_ts_seq_ltype_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnTsSeqLtypeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_TS_SEQ_LTYPE_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnTsSeqLtypeRegSpec {
    const RESET_VALUE: u32 = 0;
}
