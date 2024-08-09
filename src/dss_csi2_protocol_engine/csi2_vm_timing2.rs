#[doc = "Register `CSI2_VM_TIMING2` reader"]
pub type R = crate::R<Csi2VmTiming2Spec>;
#[doc = "Register `CSI2_VM_TIMING2` writer"]
pub type W = crate::W<Csi2VmTiming2Spec>;
#[doc = "Field `VBP` reader - 7:0\\]
Defines the vertical back porch used in video mode in number of lines. The supported values are from 0 to 255"]
pub type VbpR = crate::FieldReader;
#[doc = "Field `VBP` writer - 7:0\\]
Defines the vertical back porch used in video mode in number of lines. The supported values are from 0 to 255"]
pub type VbpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VFP` reader - 15:8\\]
Defines the vertical front porch used in video mode in number of lines. The supported values are from 0 to 255"]
pub type VfpR = crate::FieldReader;
#[doc = "Field `VFP` writer - 15:8\\]
Defines the vertical front porch used in video mode in number of lines. The supported values are from 0 to 255"]
pub type VfpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VSA` reader - 23:16\\]
Defines the vertical Sync active period used in video mode in number of lines. The supported values are from 0 to 255 It is used to generate the short packet for End of Vertical synchronization."]
pub type VsaR = crate::FieldReader;
#[doc = "Field `VSA` writer - 23:16\\]
Defines the vertical Sync active period used in video mode in number of lines. The supported values are from 0 to 255 It is used to generate the short packet for End of Vertical synchronization."]
pub type VsaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WINDOW_SYNC` reader - 27:24\\]
Number of BYTE clock cycles for the synchronization window. An interrupt for synchronization lost is generated when the received synchornization on video port is not inside the window. CSI2 does not change its own timings if the synch is inside the window. The valid values are from 4 to 15."]
pub type WindowSyncR = crate::FieldReader;
#[doc = "Field `WINDOW_SYNC` writer - 27:24\\]
Number of BYTE clock cycles for the synchronization window. An interrupt for synchronization lost is generated when the received synchornization on video port is not inside the window. CSI2 does not change its own timings if the synch is inside the window. The valid values are from 4 to 15."]
pub type WindowSyncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the vertical back porch used in video mode in number of lines. The supported values are from 0 to 255"]
    #[inline(always)]
    pub fn vbp(&self) -> VbpR {
        VbpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the vertical front porch used in video mode in number of lines. The supported values are from 0 to 255"]
    #[inline(always)]
    pub fn vfp(&self) -> VfpR {
        VfpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the vertical Sync active period used in video mode in number of lines. The supported values are from 0 to 255 It is used to generate the short packet for End of Vertical synchronization."]
    #[inline(always)]
    pub fn vsa(&self) -> VsaR {
        VsaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Number of BYTE clock cycles for the synchronization window. An interrupt for synchronization lost is generated when the received synchornization on video port is not inside the window. CSI2 does not change its own timings if the synch is inside the window. The valid values are from 4 to 15."]
    #[inline(always)]
    pub fn window_sync(&self) -> WindowSyncR {
        WindowSyncR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the vertical back porch used in video mode in number of lines. The supported values are from 0 to 255"]
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VbpW<Csi2VmTiming2Spec> {
        VbpW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the vertical front porch used in video mode in number of lines. The supported values are from 0 to 255"]
    #[inline(always)]
    #[must_use]
    pub fn vfp(&mut self) -> VfpW<Csi2VmTiming2Spec> {
        VfpW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the vertical Sync active period used in video mode in number of lines. The supported values are from 0 to 255 It is used to generate the short packet for End of Vertical synchronization."]
    #[inline(always)]
    #[must_use]
    pub fn vsa(&mut self) -> VsaW<Csi2VmTiming2Spec> {
        VsaW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Number of BYTE clock cycles for the synchronization window. An interrupt for synchronization lost is generated when the received synchornization on video port is not inside the window. CSI2 does not change its own timings if the synch is inside the window. The valid values are from 4 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn window_sync(&mut self) -> WindowSyncW<Csi2VmTiming2Spec> {
        WindowSyncW::new(self, 24)
    }
}
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VmTiming2Spec;
impl crate::RegisterSpec for Csi2VmTiming2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vm_timing2::R`](R) reader structure"]
impl crate::Readable for Csi2VmTiming2Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vm_timing2::W`](W) writer structure"]
impl crate::Writable for Csi2VmTiming2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VM_TIMING2 to value 0"]
impl crate::Resettable for Csi2VmTiming2Spec {
    const RESET_VALUE: u32 = 0;
}
