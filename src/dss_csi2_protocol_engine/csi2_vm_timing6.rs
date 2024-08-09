#[doc = "Register `CSI2_VM_TIMING6` reader"]
pub type R = crate::R<Csi2VmTiming6Spec>;
#[doc = "Register `CSI2_VM_TIMING6` writer"]
pub type W = crate::W<Csi2VmTiming6Spec>;
#[doc = "Field `BL_LP_INTERLEAVING` reader - 15:0\\]
Defines the maximum number of bytes of Low Power command mode packets that can be sent on PPI link during blanking periods during VSA, VBP or VFP periods inside one video frame on PPI link. The supported values are from 0 to 65535"]
pub type BlLpInterleavingR = crate::FieldReader<u16>;
#[doc = "Field `BL_LP_INTERLEAVING` writer - 15:0\\]
Defines the maximum number of bytes of Low Power command mode packets that can be sent on PPI link during blanking periods during VSA, VBP or VFP periods inside one video frame on PPI link. The supported values are from 0 to 65535"]
pub type BlLpInterleavingW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BL_HS_INTERLEAVING` reader - 31:16\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during blanking periods during VSA, VBP, VFP periods inside one video frame on PPI link. The supported values are from 0 to 65535 ."]
pub type BlHsInterleavingR = crate::FieldReader<u16>;
#[doc = "Field `BL_HS_INTERLEAVING` writer - 31:16\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during blanking periods during VSA, VBP, VFP periods inside one video frame on PPI link. The supported values are from 0 to 65535 ."]
pub type BlHsInterleavingW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the maximum number of bytes of Low Power command mode packets that can be sent on PPI link during blanking periods during VSA, VBP or VFP periods inside one video frame on PPI link. The supported values are from 0 to 65535"]
    #[inline(always)]
    pub fn bl_lp_interleaving(&self) -> BlLpInterleavingR {
        BlLpInterleavingR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during blanking periods during VSA, VBP, VFP periods inside one video frame on PPI link. The supported values are from 0 to 65535 ."]
    #[inline(always)]
    pub fn bl_hs_interleaving(&self) -> BlHsInterleavingR {
        BlHsInterleavingR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the maximum number of bytes of Low Power command mode packets that can be sent on PPI link during blanking periods during VSA, VBP or VFP periods inside one video frame on PPI link. The supported values are from 0 to 65535"]
    #[inline(always)]
    #[must_use]
    pub fn bl_lp_interleaving(&mut self) -> BlLpInterleavingW<Csi2VmTiming6Spec> {
        BlLpInterleavingW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during blanking periods during VSA, VBP, VFP periods inside one video frame on PPI link. The supported values are from 0 to 65535 ."]
    #[inline(always)]
    #[must_use]
    pub fn bl_hs_interleaving(&mut self) -> BlHsInterleavingW<Csi2VmTiming6Spec> {
        BlHsInterleavingW::new(self, 16)
    }
}
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VmTiming6Spec;
impl crate::RegisterSpec for Csi2VmTiming6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vm_timing6::R`](R) reader structure"]
impl crate::Readable for Csi2VmTiming6Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vm_timing6::W`](W) writer structure"]
impl crate::Writable for Csi2VmTiming6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VM_TIMING6 to value 0"]
impl crate::Resettable for Csi2VmTiming6Spec {
    const RESET_VALUE: u32 = 0;
}
