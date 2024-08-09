#[doc = "Register `CSI2_VM_TIMING3` reader"]
pub type R = crate::R<Csi2VmTiming3Spec>;
#[doc = "Register `CSI2_VM_TIMING3` writer"]
pub type W = crate::W<Csi2VmTiming3Spec>;
#[doc = "Field `VACT` reader - 15:0\\]
Defines the number of active lines used in video mode. The supported values are from 0 to 65535"]
pub type VactR = crate::FieldReader<u16>;
#[doc = "Field `VACT` writer - 15:0\\]
Defines the number of active lines used in video mode. The supported values are from 0 to 65535"]
pub type VactW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TL` reader - 31:16\\]
Defines the number of length of the line in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 8192. The values from 8193 to 65535 are not supported."]
pub type TlR = crate::FieldReader<u16>;
#[doc = "Field `TL` writer - 31:16\\]
Defines the number of length of the line in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 8192. The values from 8193 to 65535 are not supported."]
pub type TlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the number of active lines used in video mode. The supported values are from 0 to 65535"]
    #[inline(always)]
    pub fn vact(&self) -> VactR {
        VactR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Defines the number of length of the line in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 8192. The values from 8193 to 65535 are not supported."]
    #[inline(always)]
    pub fn tl(&self) -> TlR {
        TlR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the number of active lines used in video mode. The supported values are from 0 to 65535"]
    #[inline(always)]
    #[must_use]
    pub fn vact(&mut self) -> VactW<Csi2VmTiming3Spec> {
        VactW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Defines the number of length of the line in video mode in number of byte clock cycles (PPI clock) The supported values are from 0 to 8192. The values from 8193 to 65535 are not supported."]
    #[inline(always)]
    #[must_use]
    pub fn tl(&mut self) -> TlW<Csi2VmTiming3Spec> {
        TlW::new(self, 16)
    }
}
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VmTiming3Spec;
impl crate::RegisterSpec for Csi2VmTiming3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vm_timing3::R`](R) reader structure"]
impl crate::Readable for Csi2VmTiming3Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vm_timing3::W`](W) writer structure"]
impl crate::Writable for Csi2VmTiming3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VM_TIMING3 to value 0"]
impl crate::Resettable for Csi2VmTiming3Spec {
    const RESET_VALUE: u32 = 0;
}
