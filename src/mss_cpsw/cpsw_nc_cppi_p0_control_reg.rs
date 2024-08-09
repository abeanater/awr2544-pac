#[doc = "Register `CPSW_NC_CPPI_P0_CONTROL_REG` reader"]
pub type R = crate::R<CpswNcCppiP0ControlRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_CONTROL_REG` writer"]
pub type W = crate::W<CpswNcCppiP0ControlRegSpec>;
#[doc = "Field `PORT_0_RECEIVE_1` reader - 0:0\\]
Port 0 Receive Checksum Enable"]
pub type Port0Receive1R = crate::BitReader;
#[doc = "Field `PORT_0_RECEIVE_1` writer - 0:0\\]
Port 0 Receive Checksum Enable"]
pub type Port0Receive1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_IPV4` reader - 1:1\\]
Port 0 IPv4 DSCP enable"]
pub type Port0Ipv4R = crate::BitReader;
#[doc = "Field `PORT_0_IPV4` writer - 1:1\\]
Port 0 IPv4 DSCP enable"]
pub type Port0Ipv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_IPV6` reader - 2:2\\]
Port 0 IPv6 DSCP enable"]
pub type Port0Ipv6R = crate::BitReader;
#[doc = "Field `PORT_0_IPV6` writer - 2:2\\]
Port 0 IPv6 DSCP enable"]
pub type Port0Ipv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_TRANSMIT_1` reader - 3:3\\]
Port 0 Transmit Checksum Enable"]
pub type Port0Transmit1R = crate::BitReader;
#[doc = "Field `PORT_0_TRANSMIT_1` writer - 3:3\\]
Port 0 Transmit Checksum Enable"]
pub type Port0Transmit1W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `PORT_0_REMAP_2` reader - 16:16\\]
Port 0 Remap VLAN Enable"]
pub type Port0Remap2R = crate::BitReader;
#[doc = "Field `PORT_0_REMAP_2` writer - 16:16\\]
Port 0 Remap VLAN Enable"]
pub type Port0Remap2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_REMAP_1` reader - 17:17\\]
Port 0 Remap DSCP_V4 Enable"]
pub type Port0Remap1R = crate::BitReader;
#[doc = "Field `PORT_0_REMAP_1` writer - 17:17\\]
Port 0 Remap DSCP_V4 Enable"]
pub type Port0Remap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_REMAP` reader - 18:18\\]
Port 0 Remap DSCP_V6 Enable"]
pub type Port0RemapR = crate::BitReader;
#[doc = "Field `PORT_0_REMAP` writer - 18:18\\]
Port 0 Remap DSCP_V6 Enable"]
pub type Port0RemapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Port 0 Receive Checksum Enable"]
    #[inline(always)]
    pub fn port_0_receive_1(&self) -> Port0Receive1R {
        Port0Receive1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Port 0 IPv4 DSCP enable"]
    #[inline(always)]
    pub fn port_0_ipv4(&self) -> Port0Ipv4R {
        Port0Ipv4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 0 IPv6 DSCP enable"]
    #[inline(always)]
    pub fn port_0_ipv6(&self) -> Port0Ipv6R {
        Port0Ipv6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 0 Transmit Checksum Enable"]
    #[inline(always)]
    pub fn port_0_transmit_1(&self) -> Port0Transmit1R {
        Port0Transmit1R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 16 - 16:16\\]
Port 0 Remap VLAN Enable"]
    #[inline(always)]
    pub fn port_0_remap_2(&self) -> Port0Remap2R {
        Port0Remap2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Port 0 Remap DSCP_V4 Enable"]
    #[inline(always)]
    pub fn port_0_remap_1(&self) -> Port0Remap1R {
        Port0Remap1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Port 0 Remap DSCP_V6 Enable"]
    #[inline(always)]
    pub fn port_0_remap(&self) -> Port0RemapR {
        Port0RemapR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Port 0 Receive Checksum Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_receive_1(&mut self) -> Port0Receive1W<CpswNcCppiP0ControlRegSpec> {
        Port0Receive1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Port 0 IPv4 DSCP enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_ipv4(&mut self) -> Port0Ipv4W<CpswNcCppiP0ControlRegSpec> {
        Port0Ipv4W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 0 IPv6 DSCP enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_ipv6(&mut self) -> Port0Ipv6W<CpswNcCppiP0ControlRegSpec> {
        Port0Ipv6W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 0 Transmit Checksum Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_transmit_1(&mut self) -> Port0Transmit1W<CpswNcCppiP0ControlRegSpec> {
        Port0Transmit1W::new(self, 3)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 0 Transmit ECC Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_transmit(&mut self) -> Port0TransmitW<CpswNcCppiP0ControlRegSpec> {
        Port0TransmitW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 0 Receive ECC Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_receive(&mut self) -> Port0ReceiveW<CpswNcCppiP0ControlRegSpec> {
        Port0ReceiveW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Port 0 Remap VLAN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_remap_2(&mut self) -> Port0Remap2W<CpswNcCppiP0ControlRegSpec> {
        Port0Remap2W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Port 0 Remap DSCP_V4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_remap_1(&mut self) -> Port0Remap1W<CpswNcCppiP0ControlRegSpec> {
        Port0Remap1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Port 0 Remap DSCP_V6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_remap(&mut self) -> Port0RemapW<CpswNcCppiP0ControlRegSpec> {
        Port0RemapW::new(self, 18)
    }
}
#[doc = "CPPI Port 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0ControlRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0ControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0ControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0ControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_CONTROL_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0ControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
