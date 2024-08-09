#[doc = "Register `CSI2_VM_TIMING1` reader"]
pub type R = crate::R<Csi2VmTiming1Spec>;
#[doc = "Register `CSI2_VM_TIMING1` writer"]
pub type W = crate::W<Csi2VmTiming1Spec>;
#[doc = "Field `HBP` reader - 11:0\\]
Defines the horizontal back porch used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 4095"]
pub type HbpR = crate::FieldReader<u16>;
#[doc = "Field `HBP` writer - 11:0\\]
Defines the horizontal back porch used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 4095"]
pub type HbpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HFP` reader - 23:12\\]
Defines the horizontal front porch used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 4095"]
pub type HfpR = crate::FieldReader<u16>;
#[doc = "Field `HFP` writer - 23:12\\]
Defines the horizontal front porch used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 4095"]
pub type HfpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HSA` reader - 31:24\\]
Defines the horizontal Sync active period used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 255."]
pub type HsaR = crate::FieldReader;
#[doc = "Field `HSA` writer - 31:24\\]
Defines the horizontal Sync active period used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 255."]
pub type HsaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the horizontal back porch used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 4095"]
    #[inline(always)]
    pub fn hbp(&self) -> HbpR {
        HbpR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - 23:12\\]
Defines the horizontal front porch used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 4095"]
    #[inline(always)]
    pub fn hfp(&self) -> HfpR {
        HfpR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the horizontal Sync active period used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 255."]
    #[inline(always)]
    pub fn hsa(&self) -> HsaR {
        HsaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the horizontal back porch used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 4095"]
    #[inline(always)]
    #[must_use]
    pub fn hbp(&mut self) -> HbpW<Csi2VmTiming1Spec> {
        HbpW::new(self, 0)
    }
    #[doc = "Bits 12:23 - 23:12\\]
Defines the horizontal front porch used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 4095"]
    #[inline(always)]
    #[must_use]
    pub fn hfp(&mut self) -> HfpW<Csi2VmTiming1Spec> {
        HfpW::new(self, 12)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the horizontal Sync active period used in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 255."]
    #[inline(always)]
    #[must_use]
    pub fn hsa(&mut self) -> HsaW<Csi2VmTiming1Spec> {
        HsaW::new(self, 24)
    }
}
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VmTiming1Spec;
impl crate::RegisterSpec for Csi2VmTiming1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vm_timing1::R`](R) reader structure"]
impl crate::Readable for Csi2VmTiming1Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vm_timing1::W`](W) writer structure"]
impl crate::Writable for Csi2VmTiming1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VM_TIMING1 to value 0"]
impl crate::Resettable for Csi2VmTiming1Spec {
    const RESET_VALUE: u32 = 0;
}
