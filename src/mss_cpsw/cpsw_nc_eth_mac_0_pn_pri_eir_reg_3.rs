#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_3` reader"]
pub type R = crate::R<CpswNcEthMac0PnPriEirReg3Spec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_3` writer"]
pub type W = crate::W<CpswNcEthMac0PnPriEirReg3Spec>;
#[doc = "Field `PRIORITY_N_EXCESS` reader - 27:0\\]
Priority N Excess Information Rate count"]
pub type PriorityNExcessR = crate::FieldReader<u32>;
#[doc = "Field `PRIORITY_N_EXCESS` writer - 27:0\\]
Priority N Excess Information Rate count"]
pub type PriorityNExcessW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Priority N Excess Information Rate count"]
    #[inline(always)]
    pub fn priority_n_excess(&self) -> PriorityNExcessR {
        PriorityNExcessR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Priority N Excess Information Rate count"]
    #[inline(always)]
    #[must_use]
    pub fn priority_n_excess(&mut self) -> PriorityNExcessW<CpswNcEthMac0PnPriEirReg3Spec> {
        PriorityNExcessW::new(self, 0)
    }
}
#[doc = "Enet Port N Rx Priority P Excess Informatoin Rate Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnPriEirReg3Spec;
impl crate::RegisterSpec for CpswNcEthMac0PnPriEirReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_3::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnPriEirReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_pri_eir_reg_3::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnPriEirReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_PRI_EIR_REG_3 to value 0"]
impl crate::Resettable for CpswNcEthMac0PnPriEirReg3Spec {
    const RESET_VALUE: u32 = 0;
}
