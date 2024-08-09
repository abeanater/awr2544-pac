#[doc = "Register `CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_3` reader"]
pub type R = crate::R<CpswNcStat1EnetPnTxPriReg3Spec>;
#[doc = "Register `CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_3` writer"]
pub type W = crate::W<CpswNcStat1EnetPnTxPriReg3Spec>;
#[doc = "Field `ENET_TX_PRIORITY` reader - 31:0\\]
ENET TX Priority Packet Count"]
pub type EnetTxPriorityR = crate::FieldReader<u32>;
#[doc = "Field `ENET_TX_PRIORITY` writer - 31:0\\]
ENET TX Priority Packet Count"]
pub type EnetTxPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ENET TX Priority Packet Count"]
    #[inline(always)]
    pub fn enet_tx_priority(&self) -> EnetTxPriorityR {
        EnetTxPriorityR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ENET TX Priority Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn enet_tx_priority(&mut self) -> EnetTxPriorityW<CpswNcStat1EnetPnTxPriReg3Spec> {
        EnetTxPriorityW::new(self, 0)
    }
}
#[doc = "ENET Port n PRIORITY N Packet Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1EnetPnTxPriReg3Spec;
impl crate::RegisterSpec for CpswNcStat1EnetPnTxPriReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_3::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1EnetPnTxPriReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_enet_pn_tx_pri_reg_3::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1EnetPnTxPriReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_ENET_PN_TX_PRI_REG_3 to value 0"]
impl crate::Resettable for CpswNcStat1EnetPnTxPriReg3Spec {
    const RESET_VALUE: u32 = 0;
}
