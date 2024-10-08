#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_2` reader"]
pub type R = crate::R<CpswNcEthMac0PnRxDscpMapReg2Spec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_2` writer"]
pub type W = crate::W<CpswNcEthMac0PnRxDscpMapReg2Spec>;
#[doc = "Field `A_DSCP_IPV4_V6_7` reader - 2:0\\]
A DSCP IPV4/V6 packet TOS of N*8+0 is mapped to this received priority"]
pub type ADscpIpv4V6_7R = crate::FieldReader;
#[doc = "Field `A_DSCP_IPV4_V6_7` writer - 2:0\\]
A DSCP IPV4/V6 packet TOS of N*8+0 is mapped to this received priority"]
pub type ADscpIpv4V6_7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_DSCP_IPV4_V6_6` reader - 6:4\\]
A DSCP IPV4/V6 packet TOS of N*8+1 is mapped to this received priority"]
pub type ADscpIpv4V6_6R = crate::FieldReader;
#[doc = "Field `A_DSCP_IPV4_V6_6` writer - 6:4\\]
A DSCP IPV4/V6 packet TOS of N*8+1 is mapped to this received priority"]
pub type ADscpIpv4V6_6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_DSCP_IPV4_V6_5` reader - 10:8\\]
A DSCP IPV4/V6 packet TOS of N*8+2 is mapped to this received priority"]
pub type ADscpIpv4V6_5R = crate::FieldReader;
#[doc = "Field `A_DSCP_IPV4_V6_5` writer - 10:8\\]
A DSCP IPV4/V6 packet TOS of N*8+2 is mapped to this received priority"]
pub type ADscpIpv4V6_5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_DSCP_IPV4_V6_4` reader - 14:12\\]
A DSCP IPV4/V6 packet TOS of N*8+3 is mapped to this received priority"]
pub type ADscpIpv4V6_4R = crate::FieldReader;
#[doc = "Field `A_DSCP_IPV4_V6_4` writer - 14:12\\]
A DSCP IPV4/V6 packet TOS of N*8+3 is mapped to this received priority"]
pub type ADscpIpv4V6_4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_DSCP_IPV4_V6_3` reader - 18:16\\]
A DSCP IPV4/V6 packet TOS of N*8+4 is mapped to this received priority"]
pub type ADscpIpv4V6_3R = crate::FieldReader;
#[doc = "Field `A_DSCP_IPV4_V6_3` writer - 18:16\\]
A DSCP IPV4/V6 packet TOS of N*8+4 is mapped to this received priority"]
pub type ADscpIpv4V6_3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_DSCP_IPV4_V6_2` reader - 22:20\\]
A DSCP IPV4/V6 packet TOS of N*8+5 is mapped to this received priority"]
pub type ADscpIpv4V6_2R = crate::FieldReader;
#[doc = "Field `A_DSCP_IPV4_V6_2` writer - 22:20\\]
A DSCP IPV4/V6 packet TOS of N*8+5 is mapped to this received priority"]
pub type ADscpIpv4V6_2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_DSCP_IPV4_V6_1` reader - 26:24\\]
A DSCP IPV4/V6 packet TOS of N*8+6 is mapped to this received priority"]
pub type ADscpIpv4V6_1R = crate::FieldReader;
#[doc = "Field `A_DSCP_IPV4_V6_1` writer - 26:24\\]
A DSCP IPV4/V6 packet TOS of N*8+6 is mapped to this received priority"]
pub type ADscpIpv4V6_1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_DSCP_IPV4_V6` reader - 30:28\\]
A DSCP IPV4/V6 packet TOS of N*8+7 is mapped to this received priority"]
pub type ADscpIpv4V6R = crate::FieldReader;
#[doc = "Field `A_DSCP_IPV4_V6` writer - 30:28\\]
A DSCP IPV4/V6 packet TOS of N*8+7 is mapped to this received priority"]
pub type ADscpIpv4V6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
A DSCP IPV4/V6 packet TOS of N*8+0 is mapped to this received priority"]
    #[inline(always)]
    pub fn a_dscp_ipv4_v6_7(&self) -> ADscpIpv4V6_7R {
        ADscpIpv4V6_7R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
A DSCP IPV4/V6 packet TOS of N*8+1 is mapped to this received priority"]
    #[inline(always)]
    pub fn a_dscp_ipv4_v6_6(&self) -> ADscpIpv4V6_6R {
        ADscpIpv4V6_6R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
A DSCP IPV4/V6 packet TOS of N*8+2 is mapped to this received priority"]
    #[inline(always)]
    pub fn a_dscp_ipv4_v6_5(&self) -> ADscpIpv4V6_5R {
        ADscpIpv4V6_5R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
A DSCP IPV4/V6 packet TOS of N*8+3 is mapped to this received priority"]
    #[inline(always)]
    pub fn a_dscp_ipv4_v6_4(&self) -> ADscpIpv4V6_4R {
        ADscpIpv4V6_4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
A DSCP IPV4/V6 packet TOS of N*8+4 is mapped to this received priority"]
    #[inline(always)]
    pub fn a_dscp_ipv4_v6_3(&self) -> ADscpIpv4V6_3R {
        ADscpIpv4V6_3R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
A DSCP IPV4/V6 packet TOS of N*8+5 is mapped to this received priority"]
    #[inline(always)]
    pub fn a_dscp_ipv4_v6_2(&self) -> ADscpIpv4V6_2R {
        ADscpIpv4V6_2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
A DSCP IPV4/V6 packet TOS of N*8+6 is mapped to this received priority"]
    #[inline(always)]
    pub fn a_dscp_ipv4_v6_1(&self) -> ADscpIpv4V6_1R {
        ADscpIpv4V6_1R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
A DSCP IPV4/V6 packet TOS of N*8+7 is mapped to this received priority"]
    #[inline(always)]
    pub fn a_dscp_ipv4_v6(&self) -> ADscpIpv4V6R {
        ADscpIpv4V6R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
A DSCP IPV4/V6 packet TOS of N*8+0 is mapped to this received priority"]
    #[inline(always)]
    #[must_use]
    pub fn a_dscp_ipv4_v6_7(&mut self) -> ADscpIpv4V6_7W<CpswNcEthMac0PnRxDscpMapReg2Spec> {
        ADscpIpv4V6_7W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
A DSCP IPV4/V6 packet TOS of N*8+1 is mapped to this received priority"]
    #[inline(always)]
    #[must_use]
    pub fn a_dscp_ipv4_v6_6(&mut self) -> ADscpIpv4V6_6W<CpswNcEthMac0PnRxDscpMapReg2Spec> {
        ADscpIpv4V6_6W::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
A DSCP IPV4/V6 packet TOS of N*8+2 is mapped to this received priority"]
    #[inline(always)]
    #[must_use]
    pub fn a_dscp_ipv4_v6_5(&mut self) -> ADscpIpv4V6_5W<CpswNcEthMac0PnRxDscpMapReg2Spec> {
        ADscpIpv4V6_5W::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
A DSCP IPV4/V6 packet TOS of N*8+3 is mapped to this received priority"]
    #[inline(always)]
    #[must_use]
    pub fn a_dscp_ipv4_v6_4(&mut self) -> ADscpIpv4V6_4W<CpswNcEthMac0PnRxDscpMapReg2Spec> {
        ADscpIpv4V6_4W::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
A DSCP IPV4/V6 packet TOS of N*8+4 is mapped to this received priority"]
    #[inline(always)]
    #[must_use]
    pub fn a_dscp_ipv4_v6_3(&mut self) -> ADscpIpv4V6_3W<CpswNcEthMac0PnRxDscpMapReg2Spec> {
        ADscpIpv4V6_3W::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
A DSCP IPV4/V6 packet TOS of N*8+5 is mapped to this received priority"]
    #[inline(always)]
    #[must_use]
    pub fn a_dscp_ipv4_v6_2(&mut self) -> ADscpIpv4V6_2W<CpswNcEthMac0PnRxDscpMapReg2Spec> {
        ADscpIpv4V6_2W::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
A DSCP IPV4/V6 packet TOS of N*8+6 is mapped to this received priority"]
    #[inline(always)]
    #[must_use]
    pub fn a_dscp_ipv4_v6_1(&mut self) -> ADscpIpv4V6_1W<CpswNcEthMac0PnRxDscpMapReg2Spec> {
        ADscpIpv4V6_1W::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
A DSCP IPV4/V6 packet TOS of N*8+7 is mapped to this received priority"]
    #[inline(always)]
    #[must_use]
    pub fn a_dscp_ipv4_v6(&mut self) -> ADscpIpv4V6W<CpswNcEthMac0PnRxDscpMapReg2Spec> {
        ADscpIpv4V6W::new(self, 28)
    }
}
#[doc = "Enet Port N Receive IPV4/IPV6 DSCP Map M\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnRxDscpMapReg2Spec;
impl crate::RegisterSpec for CpswNcEthMac0PnRxDscpMapReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnRxDscpMapReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_rx_dscp_map_reg_2::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnRxDscpMapReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_RX_DSCP_MAP_REG_2 to value 0"]
impl crate::Resettable for CpswNcEthMac0PnRxDscpMapReg2Spec {
    const RESET_VALUE: u32 = 0;
}
