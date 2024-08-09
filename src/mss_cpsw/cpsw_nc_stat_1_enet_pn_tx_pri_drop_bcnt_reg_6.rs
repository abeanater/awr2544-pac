#[doc = "Register `CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_6` reader"]
pub type R = crate::R<CpswNcStat1EnetPnTxPriDropBcntReg6Spec>;
#[doc = "Register `CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_6` writer"]
pub type W = crate::W<CpswNcStat1EnetPnTxPriDropBcntReg6Spec>;
#[doc = "Field `ENET_PORT_N` reader - 31:0\\]
ENET Port n PRIORITY N Packet Drop Byte Count"]
pub type EnetPortNR = crate::FieldReader<u32>;
#[doc = "Field `ENET_PORT_N` writer - 31:0\\]
ENET Port n PRIORITY N Packet Drop Byte Count"]
pub type EnetPortNW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    pub fn enet_port_n(&self) -> EnetPortNR {
        EnetPortNR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ENET Port n PRIORITY N Packet Drop Byte Count"]
    #[inline(always)]
    #[must_use]
    pub fn enet_port_n(&mut self) -> EnetPortNW<CpswNcStat1EnetPnTxPriDropBcntReg6Spec> {
        EnetPortNW::new(self, 0)
    }
}
#[doc = "ENET Port n PRIORITY N Packet Drop Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1EnetPnTxPriDropBcntReg6Spec;
impl crate::RegisterSpec for CpswNcStat1EnetPnTxPriDropBcntReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1EnetPnTxPriDropBcntReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_enet_pn_tx_pri_drop_bcnt_reg_6::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1EnetPnTxPriDropBcntReg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_ENET_PN_TX_PRI_DROP_BCNT_REG_6 to value 0"]
impl crate::Resettable for CpswNcStat1EnetPnTxPriDropBcntReg6Spec {
    const RESET_VALUE: u32 = 0;
}
