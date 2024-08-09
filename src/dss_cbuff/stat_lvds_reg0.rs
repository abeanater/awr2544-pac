#[doc = "Register `STAT_LVDS_REG0` reader"]
pub type R = crate::R<StatLvdsReg0Spec>;
#[doc = "Register `STAT_LVDS_REG0` writer"]
pub type W = crate::W<StatLvdsReg0Spec>;
#[doc = "Field `STAT_LVDS_REG0` reader - 31:0\\]
TI Internal Feature. Debug only. Clr is CLR_LVDS_REG0 and MASK is CFG_MASK_REG2 FSM_STAT_CODE: \\[3:0\\]
is for Ch0, \\[7:4\\]
is for Ch1, \\[11:8\\]
is for Ch2, \\[15:12\\]
is for ch3 ASYNC_FIFO_STATUS: \\[19:16\\]
is for Ch0, \\[23:20\\]
is for Ch1, \\[27:24\\]
is for Ch2, \\[32:28\\]
is for ch3 FSM_STATE_CODE : Using this the states can be decoded. ASYNC_FIFO_STATUS: 0 - POP_ERROR; 1 - PUSH_ERROR; 2- POP_EMPTY; 3 - PUSH_FULL. Set the mask for POP_EMPTY and PUSH_FULL. These are normal conditions and will keep happening and need not generate any interrupt"]
pub type StatLvdsReg0R = crate::FieldReader<u32>;
#[doc = "Field `STAT_LVDS_REG0` writer - 31:0\\]
TI Internal Feature. Debug only. Clr is CLR_LVDS_REG0 and MASK is CFG_MASK_REG2 FSM_STAT_CODE: \\[3:0\\]
is for Ch0, \\[7:4\\]
is for Ch1, \\[11:8\\]
is for Ch2, \\[15:12\\]
is for ch3 ASYNC_FIFO_STATUS: \\[19:16\\]
is for Ch0, \\[23:20\\]
is for Ch1, \\[27:24\\]
is for Ch2, \\[32:28\\]
is for ch3 FSM_STATE_CODE : Using this the states can be decoded. ASYNC_FIFO_STATUS: 0 - POP_ERROR; 1 - PUSH_ERROR; 2- POP_EMPTY; 3 - PUSH_FULL. Set the mask for POP_EMPTY and PUSH_FULL. These are normal conditions and will keep happening and need not generate any interrupt"]
pub type StatLvdsReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal Feature. Debug only. Clr is CLR_LVDS_REG0 and MASK is CFG_MASK_REG2 FSM_STAT_CODE: \\[3:0\\]
is for Ch0, \\[7:4\\]
is for Ch1, \\[11:8\\]
is for Ch2, \\[15:12\\]
is for ch3 ASYNC_FIFO_STATUS: \\[19:16\\]
is for Ch0, \\[23:20\\]
is for Ch1, \\[27:24\\]
is for Ch2, \\[32:28\\]
is for ch3 FSM_STATE_CODE : Using this the states can be decoded. ASYNC_FIFO_STATUS: 0 - POP_ERROR; 1 - PUSH_ERROR; 2- POP_EMPTY; 3 - PUSH_FULL. Set the mask for POP_EMPTY and PUSH_FULL. These are normal conditions and will keep happening and need not generate any interrupt"]
    #[inline(always)]
    pub fn stat_lvds_reg0(&self) -> StatLvdsReg0R {
        StatLvdsReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal Feature. Debug only. Clr is CLR_LVDS_REG0 and MASK is CFG_MASK_REG2 FSM_STAT_CODE: \\[3:0\\]
is for Ch0, \\[7:4\\]
is for Ch1, \\[11:8\\]
is for Ch2, \\[15:12\\]
is for ch3 ASYNC_FIFO_STATUS: \\[19:16\\]
is for Ch0, \\[23:20\\]
is for Ch1, \\[27:24\\]
is for Ch2, \\[32:28\\]
is for ch3 FSM_STATE_CODE : Using this the states can be decoded. ASYNC_FIFO_STATUS: 0 - POP_ERROR; 1 - PUSH_ERROR; 2- POP_EMPTY; 3 - PUSH_FULL. Set the mask for POP_EMPTY and PUSH_FULL. These are normal conditions and will keep happening and need not generate any interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stat_lvds_reg0(&mut self) -> StatLvdsReg0W<StatLvdsReg0Spec> {
        StatLvdsReg0W::new(self, 0)
    }
}
#[doc = "STAT_LVDS_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_lvds_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_lvds_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatLvdsReg0Spec;
impl crate::RegisterSpec for StatLvdsReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_lvds_reg0::R`](R) reader structure"]
impl crate::Readable for StatLvdsReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`stat_lvds_reg0::W`](W) writer structure"]
impl crate::Writable for StatLvdsReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT_LVDS_REG0 to value 0"]
impl crate::Resettable for StatLvdsReg0Spec {
    const RESET_VALUE: u32 = 0;
}
