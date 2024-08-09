#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_CONTROL_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnControlRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_CONTROL_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnControlRegSpec>;
#[doc = "Field `IPV4_DSCP_ENABLE` reader - 1:1\\]
IPv4 DSCP enable"]
pub type Ipv4DscpEnableR = crate::BitReader;
#[doc = "Field `IPV4_DSCP_ENABLE` writer - 1:1\\]
IPv4 DSCP enable"]
pub type Ipv4DscpEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPV6_DSCP_ENABLE` reader - 2:2\\]
IPv6 DSCP enable"]
pub type Ipv6DscpEnableR = crate::BitReader;
#[doc = "Field `IPV6_DSCP_ENABLE` writer - 2:2\\]
IPv6 DSCP enable"]
pub type Ipv6DscpEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_LPI_CLOCKSTOP` reader - 12:12\\]
Transmit LPI clockstop enable"]
pub type TransmitLpiClockstopR = crate::BitReader;
#[doc = "Field `TRANSMIT_LPI_CLOCKSTOP` writer - 12:12\\]
Transmit LPI clockstop enable"]
pub type TransmitLpiClockstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_TRANSMIT` reader - 14:14\\]
Port 0 Transmit ECC Error Enable"]
pub type Port0TransmitR = crate::BitReader;
#[doc = "Field `PORT_0_TRANSMIT` writer - 14:14\\]
Port 0 Transmit ECC Error Enable"]
pub type Port0TransmitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_RECEIVE` reader - 15:15\\]
Port 0 Receive ECC Error Enable"]
pub type Port0ReceiveR = crate::BitReader;
#[doc = "Field `PORT_0_RECEIVE` writer - 15:15\\]
Port 0 Receive ECC Error Enable"]
pub type Port0ReceiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EST_PORT_ENABLE` reader - 17:17\\]
EST Port Enable"]
pub type EstPortEnableR = crate::BitReader;
#[doc = "Field `EST_PORT_ENABLE` writer - 17:17\\]
EST Port Enable"]
pub type EstPortEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
IPv4 DSCP enable"]
    #[inline(always)]
    pub fn ipv4_dscp_enable(&self) -> Ipv4DscpEnableR {
        Ipv4DscpEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
IPv6 DSCP enable"]
    #[inline(always)]
    pub fn ipv6_dscp_enable(&self) -> Ipv6DscpEnableR {
        Ipv6DscpEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Transmit LPI clockstop enable"]
    #[inline(always)]
    pub fn transmit_lpi_clockstop(&self) -> TransmitLpiClockstopR {
        TransmitLpiClockstopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 0 Transmit ECC Error Enable"]
    #[inline(always)]
    pub fn port_0_transmit(&self) -> Port0TransmitR {
        Port0TransmitR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 0 Receive ECC Error Enable"]
    #[inline(always)]
    pub fn port_0_receive(&self) -> Port0ReceiveR {
        Port0ReceiveR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
EST Port Enable"]
    #[inline(always)]
    pub fn est_port_enable(&self) -> EstPortEnableR {
        EstPortEnableR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
IPv4 DSCP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipv4_dscp_enable(&mut self) -> Ipv4DscpEnableW<CpswNcEthMac0PnControlRegSpec> {
        Ipv4DscpEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
IPv6 DSCP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipv6_dscp_enable(&mut self) -> Ipv6DscpEnableW<CpswNcEthMac0PnControlRegSpec> {
        Ipv6DscpEnableW::new(self, 2)
    }
    #[doc = "Bit 12 - 12:12\\]
Transmit LPI clockstop enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_lpi_clockstop(
        &mut self,
    ) -> TransmitLpiClockstopW<CpswNcEthMac0PnControlRegSpec> {
        TransmitLpiClockstopW::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 0 Transmit ECC Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_transmit(&mut self) -> Port0TransmitW<CpswNcEthMac0PnControlRegSpec> {
        Port0TransmitW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 0 Receive ECC Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_receive(&mut self) -> Port0ReceiveW<CpswNcEthMac0PnControlRegSpec> {
        Port0ReceiveW::new(self, 15)
    }
    #[doc = "Bit 17 - 17:17\\]
EST Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn est_port_enable(&mut self) -> EstPortEnableW<CpswNcEthMac0PnControlRegSpec> {
        EstPortEnableW::new(self, 17)
    }
}
#[doc = "Enet Port N Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnControlRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_CONTROL_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
