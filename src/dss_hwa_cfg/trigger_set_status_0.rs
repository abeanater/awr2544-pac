#[doc = "Register `TRIGGER_SET_STATUS_0` reader"]
pub type R = crate::R<TriggerSetStatus0Spec>;
#[doc = "Register `TRIGGER_SET_STATUS_0` writer"]
pub type W = crate::W<TriggerSetStatus0Spec>;
#[doc = "Field `trigger_set_status_0` reader - 31:0\\]
Debug register for trigger status\\[31:0\\]: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a CSI or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The mapping for 32 bits is as given below: {DMA2HWA_TRIGGER\\[31:0\\]}"]
pub type TriggerSetStatus0R = crate::FieldReader<u32>;
#[doc = "Field `trigger_set_status_0` writer - 31:0\\]
Debug register for trigger status\\[31:0\\]: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a CSI or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The mapping for 32 bits is as given below: {DMA2HWA_TRIGGER\\[31:0\\]}"]
pub type TriggerSetStatus0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Debug register for trigger status\\[31:0\\]: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a CSI or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The mapping for 32 bits is as given below: {DMA2HWA_TRIGGER\\[31:0\\]}"]
    #[inline(always)]
    pub fn trigger_set_status_0(&self) -> TriggerSetStatus0R {
        TriggerSetStatus0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Debug register for trigger status\\[31:0\\]: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a CSI or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The mapping for 32 bits is as given below: {DMA2HWA_TRIGGER\\[31:0\\]}"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_set_status_0(&mut self) -> TriggerSetStatus0W<TriggerSetStatus0Spec> {
        TriggerSetStatus0W::new(self, 0)
    }
}
#[doc = "TRIGGER_SET_STATUS_0\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger_set_status_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger_set_status_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSetStatus0Spec;
impl crate::RegisterSpec for TriggerSetStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger_set_status_0::R`](R) reader structure"]
impl crate::Readable for TriggerSetStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`trigger_set_status_0::W`](W) writer structure"]
impl crate::Writable for TriggerSetStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIGGER_SET_STATUS_0 to value 0"]
impl crate::Resettable for TriggerSetStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
