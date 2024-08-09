#[doc = "Register `ALE_EGRESSOP` reader"]
pub type R = crate::R<AleEgressopSpec>;
#[doc = "Register `ALE_EGRESSOP` writer"]
pub type W = crate::W<AleEgressopSpec>;
#[doc = "Field `THE_DESTINATION_PORTS` reader - 1:0\\]
The Destination Ports is a list of the ports the classified packet will be set to. If a destination is a Trunk, all the port bits for that trunck must be set."]
pub type TheDestinationPortsR = crate::FieldReader;
#[doc = "Field `THE_DESTINATION_PORTS` writer - 1:0\\]
The Destination Ports is a list of the ports the classified packet will be set to. If a destination is a Trunk, all the port bits for that trunck must be set."]
pub type TheDestinationPortsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `THE_TTL_CHECK` reader - 20:20\\]
The TTL Check will cause any packet that fails TTL checks to not be routed to the Inter VLAN Routing sub functions. The packet will be routed to the host it was destined to."]
pub type TheTtlCheckR = crate::BitReader;
#[doc = "Field `THE_TTL_CHECK` writer - 20:20\\]
The TTL Check will cause any packet that fails TTL checks to not be routed to the Inter VLAN Routing sub functions. The packet will be routed to the host it was destined to."]
pub type TheTtlCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THE_EGRESS_TRUNK` reader - 23:21\\]
The Egress Trunk Index is the calculated trunk index from the SA, DA or VLAN if modified to that InterVLAN routing will work on trunks as well. The DA, SA and VLAN are ignored for trunk generation on InterVLAN Routing so that this field is the index generated from the Egress Op replacements elclusive or'd together into a three bit index."]
pub type TheEgressTrunkR = crate::FieldReader;
#[doc = "Field `THE_EGRESS_TRUNK` writer - 23:21\\]
The Egress Trunk Index is the calculated trunk index from the SA, DA or VLAN if modified to that InterVLAN routing will work on trunks as well. The DA, SA and VLAN are ignored for trunk generation on InterVLAN Routing so that this field is the index generated from the Egress Op replacements elclusive or'd together into a three bit index."]
pub type TheEgressTrunkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `THE_EGRESS_OPERATION` reader - 31:24\\]
The Egress Operation defines the operation performed by the CPSW Egress Packet Operations#br# 0: NOP#br#: 1-n: Defines which egress Operation will be performed. This allows Inter VLAN routing to be configured for high bandwidth traffic, reducing CPU load.#br# 0xff: Swap SA and DA of packet, this is intended to allow OAM diagnostics for a link."]
pub type TheEgressOperationR = crate::FieldReader;
#[doc = "Field `THE_EGRESS_OPERATION` writer - 31:24\\]
The Egress Operation defines the operation performed by the CPSW Egress Packet Operations#br# 0: NOP#br#: 1-n: Defines which egress Operation will be performed. This allows Inter VLAN routing to be configured for high bandwidth traffic, reducing CPU load.#br# 0xff: Swap SA and DA of packet, this is intended to allow OAM diagnostics for a link."]
pub type TheEgressOperationW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
The Destination Ports is a list of the ports the classified packet will be set to. If a destination is a Trunk, all the port bits for that trunck must be set."]
    #[inline(always)]
    pub fn the_destination_ports(&self) -> TheDestinationPortsR {
        TheDestinationPortsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
The TTL Check will cause any packet that fails TTL checks to not be routed to the Inter VLAN Routing sub functions. The packet will be routed to the host it was destined to."]
    #[inline(always)]
    pub fn the_ttl_check(&self) -> TheTtlCheckR {
        TheTtlCheckR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
The Egress Trunk Index is the calculated trunk index from the SA, DA or VLAN if modified to that InterVLAN routing will work on trunks as well. The DA, SA and VLAN are ignored for trunk generation on InterVLAN Routing so that this field is the index generated from the Egress Op replacements elclusive or'd together into a three bit index."]
    #[inline(always)]
    pub fn the_egress_trunk(&self) -> TheEgressTrunkR {
        TheEgressTrunkR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
The Egress Operation defines the operation performed by the CPSW Egress Packet Operations#br# 0: NOP#br#: 1-n: Defines which egress Operation will be performed. This allows Inter VLAN routing to be configured for high bandwidth traffic, reducing CPU load.#br# 0xff: Swap SA and DA of packet, this is intended to allow OAM diagnostics for a link."]
    #[inline(always)]
    pub fn the_egress_operation(&self) -> TheEgressOperationR {
        TheEgressOperationR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
The Destination Ports is a list of the ports the classified packet will be set to. If a destination is a Trunk, all the port bits for that trunck must be set."]
    #[inline(always)]
    #[must_use]
    pub fn the_destination_ports(&mut self) -> TheDestinationPortsW<AleEgressopSpec> {
        TheDestinationPortsW::new(self, 0)
    }
    #[doc = "Bit 20 - 20:20\\]
The TTL Check will cause any packet that fails TTL checks to not be routed to the Inter VLAN Routing sub functions. The packet will be routed to the host it was destined to."]
    #[inline(always)]
    #[must_use]
    pub fn the_ttl_check(&mut self) -> TheTtlCheckW<AleEgressopSpec> {
        TheTtlCheckW::new(self, 20)
    }
    #[doc = "Bits 21:23 - 23:21\\]
The Egress Trunk Index is the calculated trunk index from the SA, DA or VLAN if modified to that InterVLAN routing will work on trunks as well. The DA, SA and VLAN are ignored for trunk generation on InterVLAN Routing so that this field is the index generated from the Egress Op replacements elclusive or'd together into a three bit index."]
    #[inline(always)]
    #[must_use]
    pub fn the_egress_trunk(&mut self) -> TheEgressTrunkW<AleEgressopSpec> {
        TheEgressTrunkW::new(self, 21)
    }
    #[doc = "Bits 24:31 - 31:24\\]
The Egress Operation defines the operation performed by the CPSW Egress Packet Operations#br# 0: NOP#br#: 1-n: Defines which egress Operation will be performed. This allows Inter VLAN routing to be configured for high bandwidth traffic, reducing CPU load.#br# 0xff: Swap SA and DA of packet, this is intended to allow OAM diagnostics for a link."]
    #[inline(always)]
    #[must_use]
    pub fn the_egress_operation(&mut self) -> TheEgressOperationW<AleEgressopSpec> {
        TheEgressOperationW::new(self, 24)
    }
}
#[doc = "The Egress Operation register allows enabled classifiers with any match like IPSA or IPDA match to use the CPSW Egress Packet Operations Inter VLAN Routing sub functions. If the packet was destined for the host or is destined to any port without any errors, but matches a clasifier that has a programmed egress opcode, it will be forwarded to the destination ports where the destination ports will use the thier egress opcode entry to modify the packet. InterVLAN Routing and mirroring need to be understood, they are orthogonal functions. Care must be taken not to violate VLAN rules as this can redirect packets based on classifier matches.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_egressop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_egressop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleEgressopSpec;
impl crate::RegisterSpec for AleEgressopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_egressop::R`](R) reader structure"]
impl crate::Readable for AleEgressopSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_egressop::W`](W) writer structure"]
impl crate::Writable for AleEgressopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_EGRESSOP to value 0"]
impl crate::Resettable for AleEgressopSpec {
    const RESET_VALUE: u32 = 0;
}
