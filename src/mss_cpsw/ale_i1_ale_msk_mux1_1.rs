#[doc = "Register `ALE_I1_ALE_MSK_MUX1_1` reader"]
pub type R = crate::R<AleI1AleMskMux1_1Spec>;
#[doc = "Register `ALE_I1_ALE_MSK_MUX1_1` writer"]
pub type W = crate::W<AleI1AleMskMux1_1Spec>;
#[doc = "Field `VLAN_MASK_MUX` reader - 1:0\\]
VLAN Mask Mux x - When selected by the VLAN lookup table entry FwdUnRegIdx or FwdAllRegIdx is used as the FwdUnRegMask or FwdUnRegMask values anded with the member list to determine the forwarding of packets. The Value of vlan_mask_mux_0 is read only and set to all ones for all ports."]
pub type VlanMaskMuxR = crate::FieldReader;
#[doc = "Field `VLAN_MASK_MUX` writer - 1:0\\]
VLAN Mask Mux x - When selected by the VLAN lookup table entry FwdUnRegIdx or FwdAllRegIdx is used as the FwdUnRegMask or FwdUnRegMask values anded with the member list to determine the forwarding of packets. The Value of vlan_mask_mux_0 is read only and set to all ones for all ports."]
pub type VlanMaskMuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
VLAN Mask Mux x - When selected by the VLAN lookup table entry FwdUnRegIdx or FwdAllRegIdx is used as the FwdUnRegMask or FwdUnRegMask values anded with the member list to determine the forwarding of packets. The Value of vlan_mask_mux_0 is read only and set to all ones for all ports."]
    #[inline(always)]
    pub fn vlan_mask_mux(&self) -> VlanMaskMuxR {
        VlanMaskMuxR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
VLAN Mask Mux x - When selected by the VLAN lookup table entry FwdUnRegIdx or FwdAllRegIdx is used as the FwdUnRegMask or FwdUnRegMask values anded with the member list to determine the forwarding of packets. The Value of vlan_mask_mux_0 is read only and set to all ones for all ports."]
    #[inline(always)]
    #[must_use]
    pub fn vlan_mask_mux(&mut self) -> VlanMaskMuxW<AleI1AleMskMux1_1Spec> {
        VlanMaskMuxW::new(self, 0)
    }
}
#[doc = "VLAN Mask Mux x - The ALE Mask Mux registers are used along with the VLAN registered/unregistered index selectors from the Lookup Table to determine the value for vlan registered and unregister mask respectively.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_i1_ale_msk_mux1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_i1_ale_msk_mux1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleI1AleMskMux1_1Spec;
impl crate::RegisterSpec for AleI1AleMskMux1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_i1_ale_msk_mux1_1::R`](R) reader structure"]
impl crate::Readable for AleI1AleMskMux1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_i1_ale_msk_mux1_1::W`](W) writer structure"]
impl crate::Writable for AleI1AleMskMux1_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_I1_ALE_MSK_MUX1_1 to value 0"]
impl crate::Resettable for AleI1AleMskMux1_1Spec {
    const RESET_VALUE: u32 = 0;
}
