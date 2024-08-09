#[doc = "Register `ALE_ALE_UVLAN_URCAST` reader"]
pub type R = crate::R<AleAleUvlanUrcastSpec>;
#[doc = "Register `ALE_ALE_UVLAN_URCAST` writer"]
pub type W = crate::W<AleAleUvlanUrcastSpec>;
#[doc = "Field `UNKNOWN_VLAN_UNREGISTER` reader - 1:0\\]
Unknown VLAN Unregister Multicast Flood Mask - Each bit represents the port to which unregistered multicast are sent for unregistered VLANs."]
pub type UnknownVlanUnregisterR = crate::FieldReader;
#[doc = "Field `UNKNOWN_VLAN_UNREGISTER` writer - 1:0\\]
Unknown VLAN Unregister Multicast Flood Mask - Each bit represents the port to which unregistered multicast are sent for unregistered VLANs."]
pub type UnknownVlanUnregisterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Unknown VLAN Unregister Multicast Flood Mask - Each bit represents the port to which unregistered multicast are sent for unregistered VLANs."]
    #[inline(always)]
    pub fn unknown_vlan_unregister(&self) -> UnknownVlanUnregisterR {
        UnknownVlanUnregisterR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Unknown VLAN Unregister Multicast Flood Mask - Each bit represents the port to which unregistered multicast are sent for unregistered VLANs."]
    #[inline(always)]
    #[must_use]
    pub fn unknown_vlan_unregister(&mut self) -> UnknownVlanUnregisterW<AleAleUvlanUrcastSpec> {
        UnknownVlanUnregisterW::new(self, 0)
    }
}
#[doc = "The ALE Unknown VLAN Unregistered Multicast Flood Mask Register is used to specify which egress ports unregistered multicast addresses egress for the unregistered VLAN ID.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_uvlan_urcast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_uvlan_urcast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleUvlanUrcastSpec;
impl crate::RegisterSpec for AleAleUvlanUrcastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_uvlan_urcast::R`](R) reader structure"]
impl crate::Readable for AleAleUvlanUrcastSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_uvlan_urcast::W`](W) writer structure"]
impl crate::Writable for AleAleUvlanUrcastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_UVLAN_URCAST to value 0"]
impl crate::Resettable for AleAleUvlanUrcastSpec {
    const RESET_VALUE: u32 = 0;
}
