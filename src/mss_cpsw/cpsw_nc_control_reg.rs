#[doc = "Register `CPSW_NC_CONTROL_REG` reader"]
pub type R = crate::R<CpswNcControlRegSpec>;
#[doc = "Register `CPSW_NC_CONTROL_REG` writer"]
pub type W = crate::W<CpswNcControlRegSpec>;
#[doc = "Field `VLAN_AWARE_MODE_1` reader - 0:0\\]
VLAN Aware Mode"]
pub type VlanAwareMode1R = crate::BitReader;
#[doc = "Field `VLAN_AWARE_MODE_1` writer - 0:0\\]
VLAN Aware Mode"]
pub type VlanAwareMode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLAN_AWARE_MODE` reader - 1:1\\]
VLAN Aware Mode"]
pub type VlanAwareModeR = crate::BitReader;
#[doc = "Field `VLAN_AWARE_MODE` writer - 1:1\\]
VLAN Aware Mode"]
pub type VlanAwareModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_ENABLE` reader - 2:2\\]
Port 0 Enable"]
pub type Port0EnableR = crate::BitReader;
#[doc = "Field `PORT_0_ENABLE` writer - 2:2\\]
Port 0 Enable"]
pub type Port0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_PASS_1` reader - 3:3\\]
Port 0 Pass Priority Tagged"]
pub type Port0Pass1R = crate::BitReader;
#[doc = "Field `PORT_0_PASS_1` writer - 3:3\\]
Port 0 Pass Priority Tagged"]
pub type Port0Pass1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_1_PASS` reader - 4:4\\]
Port 1 Pass Priority Tagged"]
pub type Port1PassR = crate::BitReader;
#[doc = "Field `PORT_1_PASS` writer - 4:4\\]
Port 1 Pass Priority Tagged"]
pub type Port1PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_2_PASS` reader - 5:5\\]
Port 2 Pass Priority Tagged"]
pub type Port2PassR = crate::BitReader;
#[doc = "Field `PORT_2_PASS` writer - 5:5\\]
Port 2 Pass Priority Tagged"]
pub type Port2PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_3_PASS` reader - 6:6\\]
Port 3 Pass Priority Tagged"]
pub type Port3PassR = crate::BitReader;
#[doc = "Field `PORT_3_PASS` writer - 6:6\\]
Port 3 Pass Priority Tagged"]
pub type Port3PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_4_PASS` reader - 7:7\\]
Port 4 Pass Priority Tagged"]
pub type Port4PassR = crate::BitReader;
#[doc = "Field `PORT_4_PASS` writer - 7:7\\]
Port 4 Pass Priority Tagged"]
pub type Port4PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_5_PASS` reader - 8:8\\]
Port 5 Pass Priority Tagged"]
pub type Port5PassR = crate::BitReader;
#[doc = "Field `PORT_5_PASS` writer - 8:8\\]
Port 5 Pass Priority Tagged"]
pub type Port5PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_6_PASS` reader - 9:9\\]
Port 6 Pass Priority Tagged"]
pub type Port6PassR = crate::BitReader;
#[doc = "Field `PORT_6_PASS` writer - 9:9\\]
Port 6 Pass Priority Tagged"]
pub type Port6PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_7_PASS` reader - 10:10\\]
Port 7 Pass Priority Tagged"]
pub type Port7PassR = crate::BitReader;
#[doc = "Field `PORT_7_PASS` writer - 10:10\\]
Port 7 Pass Priority Tagged"]
pub type Port7PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_8_PASS` reader - 11:11\\]
Port 8 Pass Priority Tagged"]
pub type Port8PassR = crate::BitReader;
#[doc = "Field `PORT_8_PASS` writer - 11:11\\]
Port 8 Pass Priority Tagged"]
pub type Port8PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_TRANSMIT_1` reader - 12:12\\]
Port 0 Transmit CRC Type"]
pub type Port0Transmit1R = crate::BitReader;
#[doc = "Field `PORT_0_TRANSMIT_1` writer - 12:12\\]
Port 0 Transmit CRC Type"]
pub type Port0Transmit1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_TRANSMIT` reader - 13:13\\]
Port 0 Transmit CRC remove"]
pub type Port0TransmitR = crate::BitReader;
#[doc = "Field `PORT_0_TRANSMIT` writer - 13:13\\]
Port 0 Transmit CRC remove"]
pub type Port0TransmitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_RECEIVE` reader - 14:14\\]
Port 0 Receive Short Packet Pad"]
pub type Port0ReceiveR = crate::BitReader;
#[doc = "Field `PORT_0_RECEIVE` writer - 14:14\\]
Port 0 Receive Short Packet Pad"]
pub type Port0ReceiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_0_PASS` reader - 15:15\\]
Port 0 Pass Received CRC errors"]
pub type Port0PassR = crate::BitReader;
#[doc = "Field `PORT_0_PASS` writer - 15:15\\]
Port 0 Pass Received CRC errors"]
pub type Port0PassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENERGY_EFFICIENT_ETHERNET` reader - 16:16\\]
Energy Efficient Ethernet enable"]
pub type EnergyEfficientEthernetR = crate::BitReader;
#[doc = "Field `ENERGY_EFFICIENT_ETHERNET` writer - 16:16\\]
Energy Efficient Ethernet enable"]
pub type EnergyEfficientEthernetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERSPERCED_EXPRESS_TRAFFIC` reader - 18:18\\]
Intersperced Express Traffic enable"]
pub type InterspercedExpressTrafficR = crate::BitReader;
#[doc = "Field `INTERSPERCED_EXPRESS_TRAFFIC` writer - 18:18\\]
Intersperced Express Traffic enable"]
pub type InterspercedExpressTrafficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CRC_MODE` reader - 31:31\\]
ECC CRC Mode"]
pub type EccCrcModeR = crate::BitReader;
#[doc = "Field `ECC_CRC_MODE` writer - 31:31\\]
ECC CRC Mode"]
pub type EccCrcModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
VLAN Aware Mode"]
    #[inline(always)]
    pub fn vlan_aware_mode_1(&self) -> VlanAwareMode1R {
        VlanAwareMode1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VLAN Aware Mode"]
    #[inline(always)]
    pub fn vlan_aware_mode(&self) -> VlanAwareModeR {
        VlanAwareModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 0 Enable"]
    #[inline(always)]
    pub fn port_0_enable(&self) -> Port0EnableR {
        Port0EnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 0 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_0_pass_1(&self) -> Port0Pass1R {
        Port0Pass1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Port 1 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_1_pass(&self) -> Port1PassR {
        Port1PassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Port 2 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_2_pass(&self) -> Port2PassR {
        Port2PassR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Port 3 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_3_pass(&self) -> Port3PassR {
        Port3PassR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Port 4 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_4_pass(&self) -> Port4PassR {
        Port4PassR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 5 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_5_pass(&self) -> Port5PassR {
        Port5PassR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Port 6 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_6_pass(&self) -> Port6PassR {
        Port6PassR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Port 7 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_7_pass(&self) -> Port7PassR {
        Port7PassR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Port 8 Pass Priority Tagged"]
    #[inline(always)]
    pub fn port_8_pass(&self) -> Port8PassR {
        Port8PassR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Port 0 Transmit CRC Type"]
    #[inline(always)]
    pub fn port_0_transmit_1(&self) -> Port0Transmit1R {
        Port0Transmit1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Port 0 Transmit CRC remove"]
    #[inline(always)]
    pub fn port_0_transmit(&self) -> Port0TransmitR {
        Port0TransmitR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 0 Receive Short Packet Pad"]
    #[inline(always)]
    pub fn port_0_receive(&self) -> Port0ReceiveR {
        Port0ReceiveR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 0 Pass Received CRC errors"]
    #[inline(always)]
    pub fn port_0_pass(&self) -> Port0PassR {
        Port0PassR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Energy Efficient Ethernet enable"]
    #[inline(always)]
    pub fn energy_efficient_ethernet(&self) -> EnergyEfficientEthernetR {
        EnergyEfficientEthernetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Intersperced Express Traffic enable"]
    #[inline(always)]
    pub fn intersperced_express_traffic(&self) -> InterspercedExpressTrafficR {
        InterspercedExpressTrafficR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
ECC CRC Mode"]
    #[inline(always)]
    pub fn ecc_crc_mode(&self) -> EccCrcModeR {
        EccCrcModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
VLAN Aware Mode"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_aware_mode_1(&mut self) -> VlanAwareMode1W<CpswNcControlRegSpec> {
        VlanAwareMode1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VLAN Aware Mode"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_aware_mode(&mut self) -> VlanAwareModeW<CpswNcControlRegSpec> {
        VlanAwareModeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_enable(&mut self) -> Port0EnableW<CpswNcControlRegSpec> {
        Port0EnableW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 0 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_pass_1(&mut self) -> Port0Pass1W<CpswNcControlRegSpec> {
        Port0Pass1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Port 1 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_1_pass(&mut self) -> Port1PassW<CpswNcControlRegSpec> {
        Port1PassW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Port 2 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_2_pass(&mut self) -> Port2PassW<CpswNcControlRegSpec> {
        Port2PassW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Port 3 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_3_pass(&mut self) -> Port3PassW<CpswNcControlRegSpec> {
        Port3PassW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Port 4 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_4_pass(&mut self) -> Port4PassW<CpswNcControlRegSpec> {
        Port4PassW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 5 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_5_pass(&mut self) -> Port5PassW<CpswNcControlRegSpec> {
        Port5PassW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Port 6 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_6_pass(&mut self) -> Port6PassW<CpswNcControlRegSpec> {
        Port6PassW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Port 7 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_7_pass(&mut self) -> Port7PassW<CpswNcControlRegSpec> {
        Port7PassW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Port 8 Pass Priority Tagged"]
    #[inline(always)]
    #[must_use]
    pub fn port_8_pass(&mut self) -> Port8PassW<CpswNcControlRegSpec> {
        Port8PassW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Port 0 Transmit CRC Type"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_transmit_1(&mut self) -> Port0Transmit1W<CpswNcControlRegSpec> {
        Port0Transmit1W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Port 0 Transmit CRC remove"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_transmit(&mut self) -> Port0TransmitW<CpswNcControlRegSpec> {
        Port0TransmitW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Port 0 Receive Short Packet Pad"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_receive(&mut self) -> Port0ReceiveW<CpswNcControlRegSpec> {
        Port0ReceiveW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Port 0 Pass Received CRC errors"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_pass(&mut self) -> Port0PassW<CpswNcControlRegSpec> {
        Port0PassW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Energy Efficient Ethernet enable"]
    #[inline(always)]
    #[must_use]
    pub fn energy_efficient_ethernet(&mut self) -> EnergyEfficientEthernetW<CpswNcControlRegSpec> {
        EnergyEfficientEthernetW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
Intersperced Express Traffic enable"]
    #[inline(always)]
    #[must_use]
    pub fn intersperced_express_traffic(
        &mut self,
    ) -> InterspercedExpressTrafficW<CpswNcControlRegSpec> {
        InterspercedExpressTrafficW::new(self, 18)
    }
    #[doc = "Bit 31 - 31:31\\]
ECC CRC Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_crc_mode(&mut self) -> EccCrcModeW<CpswNcControlRegSpec> {
        EccCrcModeW::new(self, 31)
    }
}
#[doc = "CPSW Switch Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcControlRegSpec;
impl crate::RegisterSpec for CpswNcControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CONTROL_REG to value 0"]
impl crate::Resettable for CpswNcControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
