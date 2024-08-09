#[doc = "Register `TRIGGER_SET_STATUS_1` reader"]
pub type R = crate::R<TriggerSetStatus1Spec>;
#[doc = "Register `TRIGGER_SET_STATUS_1` writer"]
pub type W = crate::W<TriggerSetStatus1Spec>;
#[doc = "Field `trigger_set_status_1` reader - 31:0\\]
Debug register for trigger status\\[63:32\\]: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a CSI or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The mapping for 32 bits is as given below: {4'b0,CSI2A_FRAME_START\\[1:0\\],CSI2A_LINE_END\\[7:0\\],CSI2B_FRAME_START\\[1:0\\],CSI2B_LINE_END\\[7:0\\],FW2HWA_TRIGGER_CS,FW2HWA_TRIGGER_1,3'b0,FW2HWA_TRIGGER_0,1'b1}"]
pub type TriggerSetStatus1R = crate::FieldReader<u32>;
#[doc = "Field `trigger_set_status_1` writer - 31:0\\]
Debug register for trigger status\\[63:32\\]: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a CSI or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The mapping for 32 bits is as given below: {4'b0,CSI2A_FRAME_START\\[1:0\\],CSI2A_LINE_END\\[7:0\\],CSI2B_FRAME_START\\[1:0\\],CSI2B_LINE_END\\[7:0\\],FW2HWA_TRIGGER_CS,FW2HWA_TRIGGER_1,3'b0,FW2HWA_TRIGGER_0,1'b1}"]
pub type TriggerSetStatus1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Debug register for trigger status\\[63:32\\]: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a CSI or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The mapping for 32 bits is as given below: {4'b0,CSI2A_FRAME_START\\[1:0\\],CSI2A_LINE_END\\[7:0\\],CSI2B_FRAME_START\\[1:0\\],CSI2B_LINE_END\\[7:0\\],FW2HWA_TRIGGER_CS,FW2HWA_TRIGGER_1,3'b0,FW2HWA_TRIGGER_0,1'b1}"]
    #[inline(always)]
    pub fn trigger_set_status_1(&self) -> TriggerSetStatus1R {
        TriggerSetStatus1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Debug register for trigger status\\[63:32\\]: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a CSI or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The mapping for 32 bits is as given below: {4'b0,CSI2A_FRAME_START\\[1:0\\],CSI2A_LINE_END\\[7:0\\],CSI2B_FRAME_START\\[1:0\\],CSI2B_LINE_END\\[7:0\\],FW2HWA_TRIGGER_CS,FW2HWA_TRIGGER_1,3'b0,FW2HWA_TRIGGER_0,1'b1}"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_set_status_1(&mut self) -> TriggerSetStatus1W<TriggerSetStatus1Spec> {
        TriggerSetStatus1W::new(self, 0)
    }
}
#[doc = "TRIGGER_SET_STATUS_1\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger_set_status_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger_set_status_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSetStatus1Spec;
impl crate::RegisterSpec for TriggerSetStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger_set_status_1::R`](R) reader structure"]
impl crate::Readable for TriggerSetStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`trigger_set_status_1::W`](W) writer structure"]
impl crate::Writable for TriggerSetStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIGGER_SET_STATUS_1 to value 0"]
impl crate::Resettable for TriggerSetStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
