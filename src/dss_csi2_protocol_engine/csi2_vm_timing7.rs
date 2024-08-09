#[doc = "Register `CSI2_VM_TIMING7` reader"]
pub type R = crate::R<Csi2VmTiming7Spec>;
#[doc = "Register `CSI2_VM_TIMING7` writer"]
pub type W = crate::W<Csi2VmTiming7Spec>;
#[doc = "Field `EXIT_HS_MODE_LATENCY` reader - 15:0\\]
Defines the number of HS byte clock cycles necessary for exiting from HS mode. It corresponds to the maximum delay in number of HS byte clock from de-assertion of TxRequestHS signal until PPI link is in LP-11 state from which a new entrance to HS mode can be initiated which does not take more than ENTER_HS_MODE_LATENCY clock cycles. The supported values are from 0 to 65535"]
pub type ExitHsModeLatencyR = crate::FieldReader<u16>;
#[doc = "Field `EXIT_HS_MODE_LATENCY` writer - 15:0\\]
Defines the number of HS byte clock cycles necessary for exiting from HS mode. It corresponds to the maximum delay in number of HS byte clock from de-assertion of TxRequestHS signal until PPI link is in LP-11 state from which a new entrance to HS mode can be initiated which does not take more than ENTER_HS_MODE_LATENCY clock cycles. The supported values are from 0 to 65535"]
pub type ExitHsModeLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENTER_HS_MODE_LATENCY` reader - 31:16\\]
Defines the number of HS byte clock cycles necessary for entering to HS mode. It corresponds to the delay in number of HS clock cycles from assertion of TxRequestHS signal to 1 until assertion of TxReadyHS signal to 1. The supported values are from 0 to 65535 ."]
pub type EnterHsModeLatencyR = crate::FieldReader<u16>;
#[doc = "Field `ENTER_HS_MODE_LATENCY` writer - 31:16\\]
Defines the number of HS byte clock cycles necessary for entering to HS mode. It corresponds to the delay in number of HS clock cycles from assertion of TxRequestHS signal to 1 until assertion of TxReadyHS signal to 1. The supported values are from 0 to 65535 ."]
pub type EnterHsModeLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the number of HS byte clock cycles necessary for exiting from HS mode. It corresponds to the maximum delay in number of HS byte clock from de-assertion of TxRequestHS signal until PPI link is in LP-11 state from which a new entrance to HS mode can be initiated which does not take more than ENTER_HS_MODE_LATENCY clock cycles. The supported values are from 0 to 65535"]
    #[inline(always)]
    pub fn exit_hs_mode_latency(&self) -> ExitHsModeLatencyR {
        ExitHsModeLatencyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Defines the number of HS byte clock cycles necessary for entering to HS mode. It corresponds to the delay in number of HS clock cycles from assertion of TxRequestHS signal to 1 until assertion of TxReadyHS signal to 1. The supported values are from 0 to 65535 ."]
    #[inline(always)]
    pub fn enter_hs_mode_latency(&self) -> EnterHsModeLatencyR {
        EnterHsModeLatencyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the number of HS byte clock cycles necessary for exiting from HS mode. It corresponds to the maximum delay in number of HS byte clock from de-assertion of TxRequestHS signal until PPI link is in LP-11 state from which a new entrance to HS mode can be initiated which does not take more than ENTER_HS_MODE_LATENCY clock cycles. The supported values are from 0 to 65535"]
    #[inline(always)]
    #[must_use]
    pub fn exit_hs_mode_latency(&mut self) -> ExitHsModeLatencyW<Csi2VmTiming7Spec> {
        ExitHsModeLatencyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Defines the number of HS byte clock cycles necessary for entering to HS mode. It corresponds to the delay in number of HS clock cycles from assertion of TxRequestHS signal to 1 until assertion of TxReadyHS signal to 1. The supported values are from 0 to 65535 ."]
    #[inline(always)]
    #[must_use]
    pub fn enter_hs_mode_latency(&mut self) -> EnterHsModeLatencyW<Csi2VmTiming7Spec> {
        EnterHsModeLatencyW::new(self, 16)
    }
}
#[doc = "Defines the minimum number of HS bytes clock cycles that are required to allow for the delays in entering and exiting HS mode. The supported values are from 0 to 65535\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VmTiming7Spec;
impl crate::RegisterSpec for Csi2VmTiming7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vm_timing7::R`](R) reader structure"]
impl crate::Readable for Csi2VmTiming7Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vm_timing7::W`](W) writer structure"]
impl crate::Writable for Csi2VmTiming7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VM_TIMING7 to value 0"]
impl crate::Resettable for Csi2VmTiming7Spec {
    const RESET_VALUE: u32 = 0;
}
