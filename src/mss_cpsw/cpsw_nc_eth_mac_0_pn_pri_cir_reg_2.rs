#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_2` reader"]
pub type R = crate::R<CpswNcEthMac0PnPriCirReg2Spec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_2` writer"]
pub type W = crate::W<CpswNcEthMac0PnPriCirReg2Spec>;
#[doc = "Field `PRIORITY_N_COMMITTED` reader - 27:0\\]
Priority N committed information rate"]
pub type PriorityNCommittedR = crate::FieldReader<u32>;
#[doc = "Field `PRIORITY_N_COMMITTED` writer - 27:0\\]
Priority N committed information rate"]
pub type PriorityNCommittedW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Priority N committed information rate"]
    #[inline(always)]
    pub fn priority_n_committed(&self) -> PriorityNCommittedR {
        PriorityNCommittedR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Priority N committed information rate"]
    #[inline(always)]
    #[must_use]
    pub fn priority_n_committed(&mut self) -> PriorityNCommittedW<CpswNcEthMac0PnPriCirReg2Spec> {
        PriorityNCommittedW::new(self, 0)
    }
}
#[doc = "Enet Port N Rx Priority P Committed Information Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnPriCirReg2Spec;
impl crate::RegisterSpec for CpswNcEthMac0PnPriCirReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_2::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnPriCirReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_pri_cir_reg_2::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnPriCirReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_PRI_CIR_REG_2 to value 0"]
impl crate::Resettable for CpswNcEthMac0PnPriCirReg2Spec {
    const RESET_VALUE: u32 = 0;
}
