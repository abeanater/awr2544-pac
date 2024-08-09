#[doc = "Register `ALE_I0_ALE_PORTCTL0_1` reader"]
pub type R = crate::R<AleI0AlePortctl0_1Spec>;
#[doc = "Register `ALE_I0_ALE_PORTCTL0_1` writer"]
pub type W = crate::W<AleI0AlePortctl0_1Spec>;
#[doc = "Field `PORT_STATE_DEFINS` reader - 1:0\\]
Port State - Defins the current port state used for lookup operations.#br# 0 - Disabled#br# 1 - Blocked#br# 2 - Learning#br# 3 - Forwarding"]
pub type PortStateDefinsR = crate::FieldReader;
#[doc = "Field `PORT_STATE_DEFINS` writer - 1:0\\]
Port State - Defins the current port state used for lookup operations.#br# 0 - Disabled#br# 1 - Blocked#br# 2 - Learning#br# 3 - Forwarding"]
pub type PortStateDefinsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IF_DROP_UNTAGGED` reader - 2:2\\]
If Drop Untagged - When set will drop packets without a VLAN tag."]
pub type IfDropUntaggedR = crate::BitReader;
#[doc = "Field `IF_DROP_UNTAGGED` writer - 2:2\\]
If Drop Untagged - When set will drop packets without a VLAN tag."]
pub type IfDropUntaggedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLAN_INGRESS_CHECK` reader - 3:3\\]
VLAN Ingress Check - When set if a packet received is not a member of the VLAN, the packet will be dropped."]
pub type VlanIngressCheckR = crate::BitReader;
#[doc = "Field `VLAN_INGRESS_CHECK` writer - 3:3\\]
VLAN Ingress Check - When set if a packet received is not a member of the VLAN, the packet will be dropped."]
pub type VlanIngressCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_LEARN_WHEN` reader - 4:4\\]
No Learn - When set will not learn the source addresses for this port."]
pub type NoLearnWhenR = crate::BitReader;
#[doc = "Field `NO_LEARN_WHEN` writer - 4:4\\]
No Learn - When set will not learn the source addresses for this port."]
pub type NoLearnWhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_SOURCE_ADDRESS` reader - 5:5\\]
No Source Address Update - When set will not update the source addresses for this port."]
pub type NoSourceAddressR = crate::BitReader;
#[doc = "Field `NO_SOURCE_ADDRESS` writer - 5:5\\]
No Source Address Update - When set will not update the source addresses for this port."]
pub type NoSourceAddressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIRROR_SOURCE_PORT` reader - 7:7\\]
Mirror Source Port - This field enables the source port mirror option. When this bit is set any traffic received on the port with the reg_p0_mirror_sp bit set will have its received traffic also sent to the ~imirror_top port."]
pub type MirrorSourcePortR = crate::BitReader;
#[doc = "Field `MIRROR_SOURCE_PORT` writer - 7:7\\]
Mirror Source Port - This field enables the source port mirror option. When this bit is set any traffic received on the port with the reg_p0_mirror_sp bit set will have its received traffic also sent to the ~imirror_top port."]
pub type MirrorSourcePortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_NUMBER_THIS` reader - 9:8\\]
Trunk Number - This field is used as the trunk number when the ~ip0_trunken is also set. Ports with the same trunk number that have the ~ip0_trunken also set will have traffic distributed within the trunk based on the result of the hash function descrived above."]
pub type TrunkNumberThisR = crate::FieldReader;
#[doc = "Field `TRUNK_NUMBER_THIS` writer - 9:8\\]
Trunk Number - This field is used as the trunk number when the ~ip0_trunken is also set. Ports with the same trunk number that have the ~ip0_trunken also set will have traffic distributed within the trunk based on the result of the hash function descrived above."]
pub type TrunkNumberThisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRUNK_ENABLE_THIS` reader - 10:10\\]
Trunk Enable - This field is used to enable a port into a trunk. Any port can be used as a trunk port, any two or more ports with the ~ip0_trunken its set and having the same ~ip0_trunknum will be placed in the same trunk. There is no requirement for trunk ports to be adjacent. If all ports are enabled in the same trunk, no traffic can flow as traffic received within a trunk is never trasnmitted out the same trunk. If only a single port is a member of a trunk, it looks like a normal port with exception of entries in the look up table will be noted as a trunk entry."]
pub type TrunkEnableThisR = crate::BitReader;
#[doc = "Field `TRUNK_ENABLE_THIS` writer - 10:10\\]
Trunk Enable - This field is used to enable a port into a trunk. Any port can be used as a trunk port, any two or more ports with the ~ip0_trunken its set and having the same ~ip0_trunknum will be placed in the same trunk. There is no requirement for trunk ports to be adjacent. If all ports are enabled in the same trunk, no traffic can flow as traffic received within a trunk is never trasnmitted out the same trunk. If only a single port is a member of a trunk, it looks like a normal port with exception of entries in the look up table will be noted as a trunk entry."]
pub type TrunkEnableThisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAC_ONLY_WHEN` reader - 11:11\\]
MAC Only - When set enables this port be treated like a MAC port for the host. All traffic received is only sent to the host. The host must direct traffic to this port as the lookup engine will not send traffic to the ports with the ~ip0_maconly bit set and the ~ip0_no_learn also set. If ~ip0_maconly bit is set and the ~ip0_no_learn is not set, the host can send non-directed packets that can be sent to the destination of a MacOnly port. It is also possible that The host can broadcast to all ports including MacOnly ports in this mode."]
pub type MacOnlyWhenR = crate::BitReader;
#[doc = "Field `MAC_ONLY_WHEN` writer - 11:11\\]
MAC Only - When set enables this port be treated like a MAC port for the host. All traffic received is only sent to the host. The host must direct traffic to this port as the lookup engine will not send traffic to the ports with the ~ip0_maconly bit set and the ~ip0_no_learn also set. If ~ip0_maconly bit is set and the ~ip0_no_learn is not set, the host can send non-directed packets that can be sent to the destination of a MacOnly port. It is also possible that The host can broadcast to all ports including MacOnly ports in this mode."]
pub type MacOnlyWhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_PORT_AUTHORIZATION` reader - 12:12\\]
Disable Port authorization - When set will allow unknown addresses to arrive on a switch in authorization mode. It is intended for device to device network connection on ports which do not require MACSEC encryption."]
pub type DisablePortAuthorizationR = crate::BitReader;
#[doc = "Field `DISABLE_PORT_AUTHORIZATION` writer - 12:12\\]
Disable Port authorization - When set will allow unknown addresses to arrive on a switch in authorization mode. It is intended for device to device network connection on ports which do not require MACSEC encryption."]
pub type DisablePortAuthorizationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAC_ONLY_COPY` reader - 13:13\\]
Mac Only Copy All Frames - When set a Mac Only port will transfer all received good frames to the host. When clear a Mac Only port will transfer packets to the host based on ALE destination address lookup operation (which operates more like an Ethernet Mac).#br#A Mac Only port is a port with ~imaconly set."]
pub type MacOnlyCopyR = crate::BitReader;
#[doc = "Field `MAC_ONLY_COPY` writer - 13:13\\]
Mac Only Copy All Frames - When set a Mac Only port will transfer all received good frames to the host. When clear a Mac Only port will transfer packets to the host based on ALE destination address lookup operation (which operates more like an Ethernet Mac).#br#A Mac Only port is a port with ~imaconly set."]
pub type MacOnlyCopyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DROP_DUAL_VLAN` reader - 14:14\\]
Drop Dual VLAN - When set will cause any received packet with dual VLAN stag followed by ctag to be dropped."]
pub type DropDualVlanR = crate::BitReader;
#[doc = "Field `DROP_DUAL_VLAN` writer - 14:14\\]
Drop Dual VLAN - When set will cause any received packet with dual VLAN stag followed by ctag to be dropped."]
pub type DropDualVlanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DROP_DOUBLE_VLAN` reader - 15:15\\]
Drop Double VLAN - When set cause any received packet with double VLANs to be dropped. That is if there are two ctag or two stag fields in the packet it will be dropped."]
pub type DropDoubleVlanR = crate::BitReader;
#[doc = "Field `DROP_DOUBLE_VLAN` writer - 15:15\\]
Drop Double VLAN - When set cause any received packet with double VLANs to be dropped. That is if there are two ctag or two stag fields in the packet it will be dropped."]
pub type DropDoubleVlanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICAST_PACKET_RATE` reader - 23:16\\]
Multicast Packet Rate Limit - Each prescale pulse loads this field into the port multicast rate limit counter. The port counters are decremented with each packet received or transmitted depending on whether the mode is transmit or receive. If the counters decrement to zero, then further packets are rate limited until the next prescale pulse.#br#Multicast rate limiting is enabled by a non-zero value in this field.The ~imcast_limit is the number of Multicast packets that will be forwaded per ~iale_prescale time."]
pub type MulticastPacketRateR = crate::FieldReader;
#[doc = "Field `MULTICAST_PACKET_RATE` writer - 23:16\\]
Multicast Packet Rate Limit - Each prescale pulse loads this field into the port multicast rate limit counter. The port counters are decremented with each packet received or transmitted depending on whether the mode is transmit or receive. If the counters decrement to zero, then further packets are rate limited until the next prescale pulse.#br#Multicast rate limiting is enabled by a non-zero value in this field.The ~imcast_limit is the number of Multicast packets that will be forwaded per ~iale_prescale time."]
pub type MulticastPacketRateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BROADCAST_PACKET_RATE` reader - 31:24\\]
Broadcast Packet Rate Limit - Each prescale pulse loads this field into the port broadcast rate limit counter. The port counters are decremented with each packet received or transmitted depending on whether the mode is transmit or receive. If the counters decrement to zero, then further packets are rate limited until the next prescale pulse.#br#Broadcast rate limiting is enabled by a non-zero value in this field."]
pub type BroadcastPacketRateR = crate::FieldReader;
#[doc = "Field `BROADCAST_PACKET_RATE` writer - 31:24\\]
Broadcast Packet Rate Limit - Each prescale pulse loads this field into the port broadcast rate limit counter. The port counters are decremented with each packet received or transmitted depending on whether the mode is transmit or receive. If the counters decrement to zero, then further packets are rate limited until the next prescale pulse.#br#Broadcast rate limiting is enabled by a non-zero value in this field."]
pub type BroadcastPacketRateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Port State - Defins the current port state used for lookup operations.#br# 0 - Disabled#br# 1 - Blocked#br# 2 - Learning#br# 3 - Forwarding"]
    #[inline(always)]
    pub fn port_state_defins(&self) -> PortStateDefinsR {
        PortStateDefinsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
If Drop Untagged - When set will drop packets without a VLAN tag."]
    #[inline(always)]
    pub fn if_drop_untagged(&self) -> IfDropUntaggedR {
        IfDropUntaggedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
VLAN Ingress Check - When set if a packet received is not a member of the VLAN, the packet will be dropped."]
    #[inline(always)]
    pub fn vlan_ingress_check(&self) -> VlanIngressCheckR {
        VlanIngressCheckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
No Learn - When set will not learn the source addresses for this port."]
    #[inline(always)]
    pub fn no_learn_when(&self) -> NoLearnWhenR {
        NoLearnWhenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
No Source Address Update - When set will not update the source addresses for this port."]
    #[inline(always)]
    pub fn no_source_address(&self) -> NoSourceAddressR {
        NoSourceAddressR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Mirror Source Port - This field enables the source port mirror option. When this bit is set any traffic received on the port with the reg_p0_mirror_sp bit set will have its received traffic also sent to the ~imirror_top port."]
    #[inline(always)]
    pub fn mirror_source_port(&self) -> MirrorSourcePortR {
        MirrorSourcePortR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Trunk Number - This field is used as the trunk number when the ~ip0_trunken is also set. Ports with the same trunk number that have the ~ip0_trunken also set will have traffic distributed within the trunk based on the result of the hash function descrived above."]
    #[inline(always)]
    pub fn trunk_number_this(&self) -> TrunkNumberThisR {
        TrunkNumberThisR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
Trunk Enable - This field is used to enable a port into a trunk. Any port can be used as a trunk port, any two or more ports with the ~ip0_trunken its set and having the same ~ip0_trunknum will be placed in the same trunk. There is no requirement for trunk ports to be adjacent. If all ports are enabled in the same trunk, no traffic can flow as traffic received within a trunk is never trasnmitted out the same trunk. If only a single port is a member of a trunk, it looks like a normal port with exception of entries in the look up table will be noted as a trunk entry."]
    #[inline(always)]
    pub fn trunk_enable_this(&self) -> TrunkEnableThisR {
        TrunkEnableThisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
MAC Only - When set enables this port be treated like a MAC port for the host. All traffic received is only sent to the host. The host must direct traffic to this port as the lookup engine will not send traffic to the ports with the ~ip0_maconly bit set and the ~ip0_no_learn also set. If ~ip0_maconly bit is set and the ~ip0_no_learn is not set, the host can send non-directed packets that can be sent to the destination of a MacOnly port. It is also possible that The host can broadcast to all ports including MacOnly ports in this mode."]
    #[inline(always)]
    pub fn mac_only_when(&self) -> MacOnlyWhenR {
        MacOnlyWhenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Disable Port authorization - When set will allow unknown addresses to arrive on a switch in authorization mode. It is intended for device to device network connection on ports which do not require MACSEC encryption."]
    #[inline(always)]
    pub fn disable_port_authorization(&self) -> DisablePortAuthorizationR {
        DisablePortAuthorizationR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Mac Only Copy All Frames - When set a Mac Only port will transfer all received good frames to the host. When clear a Mac Only port will transfer packets to the host based on ALE destination address lookup operation (which operates more like an Ethernet Mac).#br#A Mac Only port is a port with ~imaconly set."]
    #[inline(always)]
    pub fn mac_only_copy(&self) -> MacOnlyCopyR {
        MacOnlyCopyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Drop Dual VLAN - When set will cause any received packet with dual VLAN stag followed by ctag to be dropped."]
    #[inline(always)]
    pub fn drop_dual_vlan(&self) -> DropDualVlanR {
        DropDualVlanR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Drop Double VLAN - When set cause any received packet with double VLANs to be dropped. That is if there are two ctag or two stag fields in the packet it will be dropped."]
    #[inline(always)]
    pub fn drop_double_vlan(&self) -> DropDoubleVlanR {
        DropDoubleVlanR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Multicast Packet Rate Limit - Each prescale pulse loads this field into the port multicast rate limit counter. The port counters are decremented with each packet received or transmitted depending on whether the mode is transmit or receive. If the counters decrement to zero, then further packets are rate limited until the next prescale pulse.#br#Multicast rate limiting is enabled by a non-zero value in this field.The ~imcast_limit is the number of Multicast packets that will be forwaded per ~iale_prescale time."]
    #[inline(always)]
    pub fn multicast_packet_rate(&self) -> MulticastPacketRateR {
        MulticastPacketRateR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Broadcast Packet Rate Limit - Each prescale pulse loads this field into the port broadcast rate limit counter. The port counters are decremented with each packet received or transmitted depending on whether the mode is transmit or receive. If the counters decrement to zero, then further packets are rate limited until the next prescale pulse.#br#Broadcast rate limiting is enabled by a non-zero value in this field."]
    #[inline(always)]
    pub fn broadcast_packet_rate(&self) -> BroadcastPacketRateR {
        BroadcastPacketRateR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Port State - Defins the current port state used for lookup operations.#br# 0 - Disabled#br# 1 - Blocked#br# 2 - Learning#br# 3 - Forwarding"]
    #[inline(always)]
    #[must_use]
    pub fn port_state_defins(&mut self) -> PortStateDefinsW<AleI0AlePortctl0_1Spec> {
        PortStateDefinsW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
If Drop Untagged - When set will drop packets without a VLAN tag."]
    #[inline(always)]
    #[must_use]
    pub fn if_drop_untagged(&mut self) -> IfDropUntaggedW<AleI0AlePortctl0_1Spec> {
        IfDropUntaggedW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
VLAN Ingress Check - When set if a packet received is not a member of the VLAN, the packet will be dropped."]
    #[inline(always)]
    #[must_use]
    pub fn vlan_ingress_check(&mut self) -> VlanIngressCheckW<AleI0AlePortctl0_1Spec> {
        VlanIngressCheckW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
No Learn - When set will not learn the source addresses for this port."]
    #[inline(always)]
    #[must_use]
    pub fn no_learn_when(&mut self) -> NoLearnWhenW<AleI0AlePortctl0_1Spec> {
        NoLearnWhenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
No Source Address Update - When set will not update the source addresses for this port."]
    #[inline(always)]
    #[must_use]
    pub fn no_source_address(&mut self) -> NoSourceAddressW<AleI0AlePortctl0_1Spec> {
        NoSourceAddressW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Mirror Source Port - This field enables the source port mirror option. When this bit is set any traffic received on the port with the reg_p0_mirror_sp bit set will have its received traffic also sent to the ~imirror_top port."]
    #[inline(always)]
    #[must_use]
    pub fn mirror_source_port(&mut self) -> MirrorSourcePortW<AleI0AlePortctl0_1Spec> {
        MirrorSourcePortW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Trunk Number - This field is used as the trunk number when the ~ip0_trunken is also set. Ports with the same trunk number that have the ~ip0_trunken also set will have traffic distributed within the trunk based on the result of the hash function descrived above."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_number_this(&mut self) -> TrunkNumberThisW<AleI0AlePortctl0_1Spec> {
        TrunkNumberThisW::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
Trunk Enable - This field is used to enable a port into a trunk. Any port can be used as a trunk port, any two or more ports with the ~ip0_trunken its set and having the same ~ip0_trunknum will be placed in the same trunk. There is no requirement for trunk ports to be adjacent. If all ports are enabled in the same trunk, no traffic can flow as traffic received within a trunk is never trasnmitted out the same trunk. If only a single port is a member of a trunk, it looks like a normal port with exception of entries in the look up table will be noted as a trunk entry."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_enable_this(&mut self) -> TrunkEnableThisW<AleI0AlePortctl0_1Spec> {
        TrunkEnableThisW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
MAC Only - When set enables this port be treated like a MAC port for the host. All traffic received is only sent to the host. The host must direct traffic to this port as the lookup engine will not send traffic to the ports with the ~ip0_maconly bit set and the ~ip0_no_learn also set. If ~ip0_maconly bit is set and the ~ip0_no_learn is not set, the host can send non-directed packets that can be sent to the destination of a MacOnly port. It is also possible that The host can broadcast to all ports including MacOnly ports in this mode."]
    #[inline(always)]
    #[must_use]
    pub fn mac_only_when(&mut self) -> MacOnlyWhenW<AleI0AlePortctl0_1Spec> {
        MacOnlyWhenW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Disable Port authorization - When set will allow unknown addresses to arrive on a switch in authorization mode. It is intended for device to device network connection on ports which do not require MACSEC encryption."]
    #[inline(always)]
    #[must_use]
    pub fn disable_port_authorization(
        &mut self,
    ) -> DisablePortAuthorizationW<AleI0AlePortctl0_1Spec> {
        DisablePortAuthorizationW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Mac Only Copy All Frames - When set a Mac Only port will transfer all received good frames to the host. When clear a Mac Only port will transfer packets to the host based on ALE destination address lookup operation (which operates more like an Ethernet Mac).#br#A Mac Only port is a port with ~imaconly set."]
    #[inline(always)]
    #[must_use]
    pub fn mac_only_copy(&mut self) -> MacOnlyCopyW<AleI0AlePortctl0_1Spec> {
        MacOnlyCopyW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Drop Dual VLAN - When set will cause any received packet with dual VLAN stag followed by ctag to be dropped."]
    #[inline(always)]
    #[must_use]
    pub fn drop_dual_vlan(&mut self) -> DropDualVlanW<AleI0AlePortctl0_1Spec> {
        DropDualVlanW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Drop Double VLAN - When set cause any received packet with double VLANs to be dropped. That is if there are two ctag or two stag fields in the packet it will be dropped."]
    #[inline(always)]
    #[must_use]
    pub fn drop_double_vlan(&mut self) -> DropDoubleVlanW<AleI0AlePortctl0_1Spec> {
        DropDoubleVlanW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Multicast Packet Rate Limit - Each prescale pulse loads this field into the port multicast rate limit counter. The port counters are decremented with each packet received or transmitted depending on whether the mode is transmit or receive. If the counters decrement to zero, then further packets are rate limited until the next prescale pulse.#br#Multicast rate limiting is enabled by a non-zero value in this field.The ~imcast_limit is the number of Multicast packets that will be forwaded per ~iale_prescale time."]
    #[inline(always)]
    #[must_use]
    pub fn multicast_packet_rate(&mut self) -> MulticastPacketRateW<AleI0AlePortctl0_1Spec> {
        MulticastPacketRateW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Broadcast Packet Rate Limit - Each prescale pulse loads this field into the port broadcast rate limit counter. The port counters are decremented with each packet received or transmitted depending on whether the mode is transmit or receive. If the counters decrement to zero, then further packets are rate limited until the next prescale pulse.#br#Broadcast rate limiting is enabled by a non-zero value in this field."]
    #[inline(always)]
    #[must_use]
    pub fn broadcast_packet_rate(&mut self) -> BroadcastPacketRateW<AleI0AlePortctl0_1Spec> {
        BroadcastPacketRateW::new(self, 24)
    }
}
#[doc = "The ALE Port Control Register sets the port specific modes of operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_i0_ale_portctl0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_i0_ale_portctl0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleI0AlePortctl0_1Spec;
impl crate::RegisterSpec for AleI0AlePortctl0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_i0_ale_portctl0_1::R`](R) reader structure"]
impl crate::Readable for AleI0AlePortctl0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_i0_ale_portctl0_1::W`](W) writer structure"]
impl crate::Writable for AleI0AlePortctl0_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_I0_ALE_PORTCTL0_1 to value 0"]
impl crate::Resettable for AleI0AlePortctl0_1Spec {
    const RESET_VALUE: u32 = 0;
}
