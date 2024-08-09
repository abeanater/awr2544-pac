#[doc = "Register `ALE_POLICECFG2` reader"]
pub type R = crate::R<AlePolicecfg2Spec>;
#[doc = "Register `ALE_POLICECFG2` writer"]
pub type W = crate::W<AlePolicecfg2Spec>;
#[doc = "Field `INNER_VLAN_TABLE` reader - 4:0\\]
Inner VLAN Table Entry Index - Specifies the ALE Inner VLAN address lookup table index to match for the selected policing/classifier entry#br# Note this index assumes the VLANID is in the packet, it does not use the port VLAN if the packet in untagged or priority tagged."]
pub type InnerVlanTableR = crate::FieldReader;
#[doc = "Field `INNER_VLAN_TABLE` writer - 4:0\\]
Inner VLAN Table Entry Index - Specifies the ALE Inner VLAN address lookup table index to match for the selected policing/classifier entry#br# Note this index assumes the VLANID is in the packet, it does not use the port VLAN if the packet in untagged or priority tagged."]
pub type InnerVlanTableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INNER_VLAN_MATCH` reader - 15:15\\]
Inner VLAN Match Enable - Enables frame Inner VLAN address match for the selected policing/classifier entry"]
pub type InnerVlanMatchR = crate::BitReader;
#[doc = "Field `INNER_VLAN_MATCH` writer - 15:15\\]
Inner VLAN Match Enable - Enables frame Inner VLAN address match for the selected policing/classifier entry"]
pub type InnerVlanMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTER_VLAN_TABLE` reader - 20:16\\]
Outer VLAN Table Entry Index - Specifies the ALE Outer VLAN address lookup table index to match for the selected policing/classifier entry#br# Note this index assumes the VLANID is in the packet, it does not use the port VLAN if the packet in untagged or priority tagged."]
pub type OuterVlanTableR = crate::FieldReader;
#[doc = "Field `OUTER_VLAN_TABLE` writer - 20:16\\]
Outer VLAN Table Entry Index - Specifies the ALE Outer VLAN address lookup table index to match for the selected policing/classifier entry#br# Note this index assumes the VLANID is in the packet, it does not use the port VLAN if the packet in untagged or priority tagged."]
pub type OuterVlanTableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OUTER_VLAN_MATCH` reader - 31:31\\]
Outer VLAN Match Enable - Enables frame Outer VLAN address match for the selected policing/classifier entry"]
pub type OuterVlanMatchR = crate::BitReader;
#[doc = "Field `OUTER_VLAN_MATCH` writer - 31:31\\]
Outer VLAN Match Enable - Enables frame Outer VLAN address match for the selected policing/classifier entry"]
pub type OuterVlanMatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Inner VLAN Table Entry Index - Specifies the ALE Inner VLAN address lookup table index to match for the selected policing/classifier entry#br# Note this index assumes the VLANID is in the packet, it does not use the port VLAN if the packet in untagged or priority tagged."]
    #[inline(always)]
    pub fn inner_vlan_table(&self) -> InnerVlanTableR {
        InnerVlanTableR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Inner VLAN Match Enable - Enables frame Inner VLAN address match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn inner_vlan_match(&self) -> InnerVlanMatchR {
        InnerVlanMatchR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Outer VLAN Table Entry Index - Specifies the ALE Outer VLAN address lookup table index to match for the selected policing/classifier entry#br# Note this index assumes the VLANID is in the packet, it does not use the port VLAN if the packet in untagged or priority tagged."]
    #[inline(always)]
    pub fn outer_vlan_table(&self) -> OuterVlanTableR {
        OuterVlanTableR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Outer VLAN Match Enable - Enables frame Outer VLAN address match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn outer_vlan_match(&self) -> OuterVlanMatchR {
        OuterVlanMatchR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Inner VLAN Table Entry Index - Specifies the ALE Inner VLAN address lookup table index to match for the selected policing/classifier entry#br# Note this index assumes the VLANID is in the packet, it does not use the port VLAN if the packet in untagged or priority tagged."]
    #[inline(always)]
    #[must_use]
    pub fn inner_vlan_table(&mut self) -> InnerVlanTableW<AlePolicecfg2Spec> {
        InnerVlanTableW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Inner VLAN Match Enable - Enables frame Inner VLAN address match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn inner_vlan_match(&mut self) -> InnerVlanMatchW<AlePolicecfg2Spec> {
        InnerVlanMatchW::new(self, 15)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Outer VLAN Table Entry Index - Specifies the ALE Outer VLAN address lookup table index to match for the selected policing/classifier entry#br# Note this index assumes the VLANID is in the packet, it does not use the port VLAN if the packet in untagged or priority tagged."]
    #[inline(always)]
    #[must_use]
    pub fn outer_vlan_table(&mut self) -> OuterVlanTableW<AlePolicecfg2Spec> {
        OuterVlanTableW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Outer VLAN Match Enable - Enables frame Outer VLAN address match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn outer_vlan_match(&mut self) -> OuterVlanMatchW<AlePolicecfg2Spec> {
        OuterVlanMatchW::new(self, 31)
    }
}
#[doc = "The Policing Config 2 holds the match enable/match index for the Outer VLAN and Inner VLAN addresses\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicecfg2Spec;
impl crate::RegisterSpec for AlePolicecfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policecfg2::R`](R) reader structure"]
impl crate::Readable for AlePolicecfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_policecfg2::W`](W) writer structure"]
impl crate::Writable for AlePolicecfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICECFG2 to value 0"]
impl crate::Resettable for AlePolicecfg2Spec {
    const RESET_VALUE: u32 = 0;
}
