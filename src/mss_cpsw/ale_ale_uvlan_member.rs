#[doc = "Register `ALE_ALE_UVLAN_MEMBER` reader"]
pub type R = crate::R<AleAleUvlanMemberSpec>;
#[doc = "Register `ALE_ALE_UVLAN_MEMBER` writer"]
pub type W = crate::W<AleAleUvlanMemberSpec>;
#[doc = "Field `UNKNOWN_VLAN_MEMBER` reader - 1:0\\]
Unknown VLAN Member List - Each bit represents the port member status for unknown VLANs."]
pub type UnknownVlanMemberR = crate::FieldReader;
#[doc = "Field `UNKNOWN_VLAN_MEMBER` writer - 1:0\\]
Unknown VLAN Member List - Each bit represents the port member status for unknown VLANs."]
pub type UnknownVlanMemberW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Unknown VLAN Member List - Each bit represents the port member status for unknown VLANs."]
    #[inline(always)]
    pub fn unknown_vlan_member(&self) -> UnknownVlanMemberR {
        UnknownVlanMemberR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Unknown VLAN Member List - Each bit represents the port member status for unknown VLANs."]
    #[inline(always)]
    #[must_use]
    pub fn unknown_vlan_member(&mut self) -> UnknownVlanMemberW<AleAleUvlanMemberSpec> {
        UnknownVlanMemberW::new(self, 0)
    }
}
#[doc = "The ALE Unknown VLAN Member Mask Register is used to specify the member list for unknown VLAN ID.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_uvlan_member::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_uvlan_member::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleUvlanMemberSpec;
impl crate::RegisterSpec for AleAleUvlanMemberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_uvlan_member::R`](R) reader structure"]
impl crate::Readable for AleAleUvlanMemberSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_uvlan_member::W`](W) writer structure"]
impl crate::Writable for AleAleUvlanMemberSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_UVLAN_MEMBER to value 0"]
impl crate::Resettable for AleAleUvlanMemberSpec {
    const RESET_VALUE: u32 = 0;
}
