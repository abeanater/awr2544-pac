#[doc = "Register `ALE_ALE_UVLAN_UNTAG` reader"]
pub type R = crate::R<AleAleUvlanUntagSpec>;
#[doc = "Register `ALE_ALE_UVLAN_UNTAG` writer"]
pub type W = crate::W<AleAleUvlanUntagSpec>;
#[doc = "Field `UNKNOWN_VLAN_FORCE` reader - 1:0\\]
Unknown VLAN Force Untagged Egress Mask - Each bit represents the port where the VLAN will be removed for unregistered VLANs."]
pub type UnknownVlanForceR = crate::FieldReader;
#[doc = "Field `UNKNOWN_VLAN_FORCE` writer - 1:0\\]
Unknown VLAN Force Untagged Egress Mask - Each bit represents the port where the VLAN will be removed for unregistered VLANs."]
pub type UnknownVlanForceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Unknown VLAN Force Untagged Egress Mask - Each bit represents the port where the VLAN will be removed for unregistered VLANs."]
    #[inline(always)]
    pub fn unknown_vlan_force(&self) -> UnknownVlanForceR {
        UnknownVlanForceR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Unknown VLAN Force Untagged Egress Mask - Each bit represents the port where the VLAN will be removed for unregistered VLANs."]
    #[inline(always)]
    #[must_use]
    pub fn unknown_vlan_force(&mut self) -> UnknownVlanForceW<AleAleUvlanUntagSpec> {
        UnknownVlanForceW::new(self, 0)
    }
}
#[doc = "The ALE Unknown VLAN force Untagged Egress Mask Register is used to specify which egress ports the VLAN ID will be removed.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_uvlan_untag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_uvlan_untag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleUvlanUntagSpec;
impl crate::RegisterSpec for AleAleUvlanUntagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_uvlan_untag::R`](R) reader structure"]
impl crate::Readable for AleAleUvlanUntagSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_uvlan_untag::W`](W) writer structure"]
impl crate::Writable for AleAleUvlanUntagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_UVLAN_UNTAG to value 0"]
impl crate::Resettable for AleAleUvlanUntagSpec {
    const RESET_VALUE: u32 = 0;
}
