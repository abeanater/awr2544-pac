#[doc = "Register `STAT_CBUFF_REG1` reader"]
pub type R = crate::R<StatCbuffReg1Spec>;
#[doc = "Register `STAT_CBUFF_REG1` writer"]
pub type W = crate::W<StatCbuffReg1Spec>;
#[doc = "Field `S_LCLFSM_ERR` reader - 0:0\\]
TI Internal Feature. Debug only. LCL_FIFO_FSM_ERROR"]
pub type SLclfsmErrR = crate::BitReader;
#[doc = "Field `S_LCLFSM_ERR` writer - 0:0\\]
TI Internal Feature. Debug only. LCL_FIFO_FSM_ERROR"]
pub type SLclfsmErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_LCLPOP_ERR` reader - 1:1\\]
TI Internal Feature. Debug only. LCL_FIFO_POP_ERROR"]
pub type SLclpopErrR = crate::BitReader;
#[doc = "Field `S_LCLPOP_ERR` writer - 1:1\\]
TI Internal Feature. Debug only. LCL_FIFO_POP_ERROR"]
pub type SLclpopErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_LCLPUSH_ERR` reader - 2:2\\]
TI Internal Feature. Debug only. LCL_FIFO_PUSH_ERROR"]
pub type SLclpushErrR = crate::BitReader;
#[doc = "Field `S_LCLPUSH_ERR` writer - 2:2\\]
TI Internal Feature. Debug only. LCL_FIFO_PUSH_ERROR"]
pub type SLclpushErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1_UNUSED1` reader - 7:3\\]
RESERVED"]
pub type S1Unused1R = crate::FieldReader;
#[doc = "Field `S1_UNUSED1` writer - 7:3\\]
RESERVED"]
pub type S1Unused1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `S_CBPOP_ERR` reader - 8:8\\]
TI Internal Feature. Debug only. CBUFF_FIFO_POP_ERROR"]
pub type SCbpopErrR = crate::BitReader;
#[doc = "Field `S_CBPOP_ERR` writer - 8:8\\]
TI Internal Feature. Debug only. CBUFF_FIFO_POP_ERROR"]
pub type SCbpopErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CBPUSH_ERR` reader - 9:9\\]
TI Internal Feature. Debug only. CBUFF_FIFO_PUSH_ERROR"]
pub type SCbpushErrR = crate::BitReader;
#[doc = "Field `S_CBPUSH_ERR` writer - 9:9\\]
TI Internal Feature. Debug only. CBUFF_FIFO_PUSH_ERROR"]
pub type SCbpushErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CBFIFO_FULL` reader - 10:10\\]
TI Internal Feature. Debug only. CBUFF_FIFO Full Status ΓÇô Keep this masked, since full and empty will be normal conditions."]
pub type SCbfifoFullR = crate::BitReader;
#[doc = "Field `S_CBFIFO_FULL` writer - 10:10\\]
TI Internal Feature. Debug only. CBUFF_FIFO Full Status ΓÇô Keep this masked, since full and empty will be normal conditions."]
pub type SCbfifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CBFIFO_EMPTY` reader - 11:11\\]
TI Internal Feature. Debug only. CBUFF_FIFO Empty Status ΓÇô Keep this masked, since full and empty will be normal conditions."]
pub type SCbfifoEmptyR = crate::BitReader;
#[doc = "Field `S_CBFIFO_EMPTY` writer - 11:11\\]
TI Internal Feature. Debug only. CBUFF_FIFO Empty Status ΓÇô Keep this masked, since full and empty will be normal conditions."]
pub type SCbfifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1_UNUSED2` reader - 15:12\\]
RESERVED"]
pub type S1Unused2R = crate::FieldReader;
#[doc = "Field `S1_UNUSED2` writer - 15:12\\]
RESERVED"]
pub type S1Unused2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `S_CHIRP_ERR` reader - 16:16\\]
Indicates tha the chirpAvailable from ADCBuffer arrived before CBUFF has completed sending out the previous Chirp data."]
pub type SChirpErrR = crate::BitReader;
#[doc = "Field `S_CHIRP_ERR` writer - 16:16\\]
Indicates tha the chirpAvailable from ADCBuffer arrived before CBUFF has completed sending out the previous Chirp data."]
pub type SChirpErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_FRAME_ERR` reader - 17:17\\]
Indicates the FrameStart arrived before CBUFF has completed sending out data for all the Chirps programmed"]
pub type SFrameErrR = crate::BitReader;
#[doc = "Field `S_FRAME_ERR` writer - 17:17\\]
Indicates the FrameStart arrived before CBUFF has completed sending out data for all the Chirps programmed"]
pub type SFrameErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_PKTRCV_ERR` reader - 18:18\\]
TI Internal Feature. Debug only. If the packetReceived arrives at a wrong time. It should NOT be coming while in IDLE state (as no packet was sent before) and in HIBER state (where the next LL group is being evaluated)."]
pub type SPktrcvErrR = crate::BitReader;
#[doc = "Field `S_PKTRCV_ERR` writer - 18:18\\]
TI Internal Feature. Debug only. If the packetReceived arrives at a wrong time. It should NOT be coming while in IDLE state (as no packet was sent before) and in HIBER state (where the next LL group is being evaluated)."]
pub type SPktrcvErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CBFIFO_EMPTY_IN_FSM` reader - 19:19\\]
TI Internal Feature. Debug only. cbuff-fifo_empty - Keep this masked. Not relevant."]
pub type SCbfifoEmptyInFsmR = crate::BitReader;
#[doc = "Field `S_CBFIFO_EMPTY_IN_FSM` writer - 19:19\\]
TI Internal Feature. Debug only. cbuff-fifo_empty - Keep this masked. Not relevant."]
pub type SCbfifoEmptyInFsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CBFIFO_READY_IN_FSM` reader - 20:20\\]
TI Internal Feature. Debug only. cbuff-fifo_ready - Keep this masked. Not relevant."]
pub type SCbfifoReadyInFsmR = crate::BitReader;
#[doc = "Field `S_CBFIFO_READY_IN_FSM` writer - 20:20\\]
TI Internal Feature. Debug only. cbuff-fifo_ready - Keep this masked. Not relevant."]
pub type SCbfifoReadyInFsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1_UNUSED3` reader - "]
pub type S1Unused3R = crate::FieldReader<u16>;
#[doc = "Field `S1_UNUSED3` writer - "]
pub type S1Unused3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feature. Debug only. LCL_FIFO_FSM_ERROR"]
    #[inline(always)]
    pub fn s_lclfsm_err(&self) -> SLclfsmErrR {
        SLclfsmErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal Feature. Debug only. LCL_FIFO_POP_ERROR"]
    #[inline(always)]
    pub fn s_lclpop_err(&self) -> SLclpopErrR {
        SLclpopErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TI Internal Feature. Debug only. LCL_FIFO_PUSH_ERROR"]
    #[inline(always)]
    pub fn s_lclpush_err(&self) -> SLclpushErrR {
        SLclpushErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
RESERVED"]
    #[inline(always)]
    pub fn s1_unused1(&self) -> S1Unused1R {
        S1Unused1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
TI Internal Feature. Debug only. CBUFF_FIFO_POP_ERROR"]
    #[inline(always)]
    pub fn s_cbpop_err(&self) -> SCbpopErrR {
        SCbpopErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
TI Internal Feature. Debug only. CBUFF_FIFO_PUSH_ERROR"]
    #[inline(always)]
    pub fn s_cbpush_err(&self) -> SCbpushErrR {
        SCbpushErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
TI Internal Feature. Debug only. CBUFF_FIFO Full Status ΓÇô Keep this masked, since full and empty will be normal conditions."]
    #[inline(always)]
    pub fn s_cbfifo_full(&self) -> SCbfifoFullR {
        SCbfifoFullR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
TI Internal Feature. Debug only. CBUFF_FIFO Empty Status ΓÇô Keep this masked, since full and empty will be normal conditions."]
    #[inline(always)]
    pub fn s_cbfifo_empty(&self) -> SCbfifoEmptyR {
        SCbfifoEmptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
RESERVED"]
    #[inline(always)]
    pub fn s1_unused2(&self) -> S1Unused2R {
        S1Unused2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates tha the chirpAvailable from ADCBuffer arrived before CBUFF has completed sending out the previous Chirp data."]
    #[inline(always)]
    pub fn s_chirp_err(&self) -> SChirpErrR {
        SChirpErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates the FrameStart arrived before CBUFF has completed sending out data for all the Chirps programmed"]
    #[inline(always)]
    pub fn s_frame_err(&self) -> SFrameErrR {
        SFrameErrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
TI Internal Feature. Debug only. If the packetReceived arrives at a wrong time. It should NOT be coming while in IDLE state (as no packet was sent before) and in HIBER state (where the next LL group is being evaluated)."]
    #[inline(always)]
    pub fn s_pktrcv_err(&self) -> SPktrcvErrR {
        SPktrcvErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
TI Internal Feature. Debug only. cbuff-fifo_empty - Keep this masked. Not relevant."]
    #[inline(always)]
    pub fn s_cbfifo_empty_in_fsm(&self) -> SCbfifoEmptyInFsmR {
        SCbfifoEmptyInFsmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
TI Internal Feature. Debug only. cbuff-fifo_ready - Keep this masked. Not relevant."]
    #[inline(always)]
    pub fn s_cbfifo_ready_in_fsm(&self) -> SCbfifoReadyInFsmR {
        SCbfifoReadyInFsmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn s1_unused3(&self) -> S1Unused3R {
        S1Unused3R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feature. Debug only. LCL_FIFO_FSM_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn s_lclfsm_err(&mut self) -> SLclfsmErrW<StatCbuffReg1Spec> {
        SLclfsmErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal Feature. Debug only. LCL_FIFO_POP_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn s_lclpop_err(&mut self) -> SLclpopErrW<StatCbuffReg1Spec> {
        SLclpopErrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TI Internal Feature. Debug only. LCL_FIFO_PUSH_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn s_lclpush_err(&mut self) -> SLclpushErrW<StatCbuffReg1Spec> {
        SLclpushErrW::new(self, 2)
    }
    #[doc = "Bits 3:7 - 7:3\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn s1_unused1(&mut self) -> S1Unused1W<StatCbuffReg1Spec> {
        S1Unused1W::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
TI Internal Feature. Debug only. CBUFF_FIFO_POP_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn s_cbpop_err(&mut self) -> SCbpopErrW<StatCbuffReg1Spec> {
        SCbpopErrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
TI Internal Feature. Debug only. CBUFF_FIFO_PUSH_ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn s_cbpush_err(&mut self) -> SCbpushErrW<StatCbuffReg1Spec> {
        SCbpushErrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
TI Internal Feature. Debug only. CBUFF_FIFO Full Status ΓÇô Keep this masked, since full and empty will be normal conditions."]
    #[inline(always)]
    #[must_use]
    pub fn s_cbfifo_full(&mut self) -> SCbfifoFullW<StatCbuffReg1Spec> {
        SCbfifoFullW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
TI Internal Feature. Debug only. CBUFF_FIFO Empty Status ΓÇô Keep this masked, since full and empty will be normal conditions."]
    #[inline(always)]
    #[must_use]
    pub fn s_cbfifo_empty(&mut self) -> SCbfifoEmptyW<StatCbuffReg1Spec> {
        SCbfifoEmptyW::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn s1_unused2(&mut self) -> S1Unused2W<StatCbuffReg1Spec> {
        S1Unused2W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates tha the chirpAvailable from ADCBuffer arrived before CBUFF has completed sending out the previous Chirp data."]
    #[inline(always)]
    #[must_use]
    pub fn s_chirp_err(&mut self) -> SChirpErrW<StatCbuffReg1Spec> {
        SChirpErrW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates the FrameStart arrived before CBUFF has completed sending out data for all the Chirps programmed"]
    #[inline(always)]
    #[must_use]
    pub fn s_frame_err(&mut self) -> SFrameErrW<StatCbuffReg1Spec> {
        SFrameErrW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
TI Internal Feature. Debug only. If the packetReceived arrives at a wrong time. It should NOT be coming while in IDLE state (as no packet was sent before) and in HIBER state (where the next LL group is being evaluated)."]
    #[inline(always)]
    #[must_use]
    pub fn s_pktrcv_err(&mut self) -> SPktrcvErrW<StatCbuffReg1Spec> {
        SPktrcvErrW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
TI Internal Feature. Debug only. cbuff-fifo_empty - Keep this masked. Not relevant."]
    #[inline(always)]
    #[must_use]
    pub fn s_cbfifo_empty_in_fsm(&mut self) -> SCbfifoEmptyInFsmW<StatCbuffReg1Spec> {
        SCbfifoEmptyInFsmW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
TI Internal Feature. Debug only. cbuff-fifo_ready - Keep this masked. Not relevant."]
    #[inline(always)]
    #[must_use]
    pub fn s_cbfifo_ready_in_fsm(&mut self) -> SCbfifoReadyInFsmW<StatCbuffReg1Spec> {
        SCbfifoReadyInFsmW::new(self, 20)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    #[must_use]
    pub fn s1_unused3(&mut self) -> S1Unused3W<StatCbuffReg1Spec> {
        S1Unused3W::new(self, 21)
    }
}
#[doc = "STAT_CBUFF_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatCbuffReg1Spec;
impl crate::RegisterSpec for StatCbuffReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_cbuff_reg1::R`](R) reader structure"]
impl crate::Readable for StatCbuffReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`stat_cbuff_reg1::W`](W) writer structure"]
impl crate::Writable for StatCbuffReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT_CBUFF_REG1 to value 0"]
impl crate::Resettable for StatCbuffReg1Spec {
    const RESET_VALUE: u32 = 0;
}
