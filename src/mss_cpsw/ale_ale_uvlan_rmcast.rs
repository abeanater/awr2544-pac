#[doc = "Register `ALE_ALE_UVLAN_RMCAST` reader"]
pub type R = crate::R<AleAleUvlanRmcastSpec>;
#[doc = "Register `ALE_ALE_UVLAN_RMCAST` writer"]
pub type W = crate::W<AleAleUvlanRmcastSpec>;
#[doc = "Field `UNKNOWN_VLAN_REGISTER` reader - 1:0\\]
Unknown VLAN Register Multicast Flood Mask - Each bit represents the port to which registered multicast are sent for unregistered VLANs. This field is ANDed with the registered multicast mask to determine the destinations for unregistered VLANs."]
pub type UnknownVlanRegisterR = crate::FieldReader;
#[doc = "Field `UNKNOWN_VLAN_REGISTER` writer - 1:0\\]
Unknown VLAN Register Multicast Flood Mask - Each bit represents the port to which registered multicast are sent for unregistered VLANs. This field is ANDed with the registered multicast mask to determine the destinations for unregistered VLANs."]
pub type UnknownVlanRegisterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Unknown VLAN Register Multicast Flood Mask - Each bit represents the port to which registered multicast are sent for unregistered VLANs. This field is ANDed with the registered multicast mask to determine the destinations for unregistered VLANs."]
    #[inline(always)]
    pub fn unknown_vlan_register(&self) -> UnknownVlanRegisterR {
        UnknownVlanRegisterR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Unknown VLAN Register Multicast Flood Mask - Each bit represents the port to which registered multicast are sent for unregistered VLANs. This field is ANDed with the registered multicast mask to determine the destinations for unregistered VLANs."]
    #[inline(always)]
    #[must_use]
    pub fn unknown_vlan_register(&mut self) -> UnknownVlanRegisterW<AleAleUvlanRmcastSpec> {
        UnknownVlanRegisterW::new(self, 0)
    }
}
#[doc = "The ALE Unknown VLAN Registered Multicast Flood Mask Register is used to specify which egress ports registered multicast addresses egress for the unregistered VLAN ID.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_uvlan_rmcast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_uvlan_rmcast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleUvlanRmcastSpec;
impl crate::RegisterSpec for AleAleUvlanRmcastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_uvlan_rmcast::R`](R) reader structure"]
impl crate::Readable for AleAleUvlanRmcastSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_uvlan_rmcast::W`](W) writer structure"]
impl crate::Writable for AleAleUvlanRmcastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_UVLAN_RMCAST to value 0"]
impl crate::Resettable for AleAleUvlanRmcastSpec {
    const RESET_VALUE: u32 = 0;
}
