#[doc = "Register `CSI2_VM_TIMING4` reader"]
pub type R = crate::R<Csi2VmTiming4Spec>;
#[doc = "Register `CSI2_VM_TIMING4` writer"]
pub type W = crate::W<Csi2VmTiming4Spec>;
#[doc = "Field `HBP_HS_INTERLEAVING` reader - 7:0\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HBP blanking period. The supported values are from 0 to 255"]
pub type HbpHsInterleavingR = crate::FieldReader;
#[doc = "Field `HBP_HS_INTERLEAVING` writer - 7:0\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HBP blanking period. The supported values are from 0 to 255"]
pub type HbpHsInterleavingW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HFP_HS_INTERLEAVING` reader - 15:8\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HFP blanking period. The supported values are from 0 to 255"]
pub type HfpHsInterleavingR = crate::FieldReader;
#[doc = "Field `HFP_HS_INTERLEAVING` writer - 15:8\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HFP blanking period. The supported values are from 0 to 255"]
pub type HfpHsInterleavingW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSA_HS_INTERLEAVING` reader - 23:16\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HSA blanking period. The supported values are from 0 to 255."]
pub type HsaHsInterleavingR = crate::FieldReader;
#[doc = "Field `HSA_HS_INTERLEAVING` writer - 23:16\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HSA blanking period. The supported values are from 0 to 255."]
pub type HsaHsInterleavingW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HBP blanking period. The supported values are from 0 to 255"]
    #[inline(always)]
    pub fn hbp_hs_interleaving(&self) -> HbpHsInterleavingR {
        HbpHsInterleavingR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HFP blanking period. The supported values are from 0 to 255"]
    #[inline(always)]
    pub fn hfp_hs_interleaving(&self) -> HfpHsInterleavingR {
        HfpHsInterleavingR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HSA blanking period. The supported values are from 0 to 255."]
    #[inline(always)]
    pub fn hsa_hs_interleaving(&self) -> HsaHsInterleavingR {
        HsaHsInterleavingR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HBP blanking period. The supported values are from 0 to 255"]
    #[inline(always)]
    #[must_use]
    pub fn hbp_hs_interleaving(&mut self) -> HbpHsInterleavingW<Csi2VmTiming4Spec> {
        HbpHsInterleavingW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HFP blanking period. The supported values are from 0 to 255"]
    #[inline(always)]
    #[must_use]
    pub fn hfp_hs_interleaving(&mut self) -> HfpHsInterleavingW<Csi2VmTiming4Spec> {
        HfpHsInterleavingW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the number of HS byte clock cycles that can be used for interleaving High Speed command mode packet into Video Mode stream during HSA blanking period. The supported values are from 0 to 255."]
    #[inline(always)]
    #[must_use]
    pub fn hsa_hs_interleaving(&mut self) -> HsaHsInterleavingW<Csi2VmTiming4Spec> {
        HsaHsInterleavingW::new(self, 16)
    }
}
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VmTiming4Spec;
impl crate::RegisterSpec for Csi2VmTiming4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vm_timing4::R`](R) reader structure"]
impl crate::Readable for Csi2VmTiming4Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vm_timing4::W`](W) writer structure"]
impl crate::Writable for Csi2VmTiming4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VM_TIMING4 to value 0"]
impl crate::Resettable for Csi2VmTiming4Spec {
    const RESET_VALUE: u32 = 0;
}
