#[doc = "Register `CPTS_CONTROL_REG` reader"]
pub type R = crate::R<CptsControlRegSpec>;
#[doc = "Register `CPTS_CONTROL_REG` writer"]
pub type W = crate::W<CptsControlRegSpec>;
#[doc = "Field `TIME_SYNC_ENABLE` reader - 0:0\\]
Time sync enable"]
pub type TimeSyncEnableR = crate::BitReader;
#[doc = "Field `TIME_SYNC_ENABLE` writer - 0:0\\]
Time sync enable"]
pub type TimeSyncEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_TEST` reader - 1:1\\]
Interrupt test"]
pub type InterruptTestR = crate::BitReader;
#[doc = "Field `INTERRUPT_TEST` writer - 1:1\\]
Interrupt test"]
pub type InterruptTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_COMP_POLARITY` reader - 2:2\\]
TS_COMP polarity"]
pub type TsCompPolarityR = crate::BitReader;
#[doc = "Field `TS_COMP_POLARITY` writer - 2:2\\]
TS_COMP polarity"]
pub type TsCompPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_RECEIVE_TIMESTAMP` reader - 3:3\\]
Host Receive Timestamp Enable"]
pub type HostReceiveTimestampR = crate::BitReader;
#[doc = "Field `HOST_RECEIVE_TIMESTAMP` writer - 3:3\\]
Host Receive Timestamp Enable"]
pub type HostReceiveTimestampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQUENCE_ENABLE` reader - 4:4\\]
Sequence Enable"]
pub type SequenceEnableR = crate::BitReader;
#[doc = "Field `SEQUENCE_ENABLE` writer - 4:4\\]
Sequence Enable"]
pub type SequenceEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMESTAMP_MODE` reader - 5:5\\]
Timestamp mode"]
pub type TimestampModeR = crate::BitReader;
#[doc = "Field `TIMESTAMP_MODE` writer - 5:5\\]
Timestamp mode"]
pub type TimestampModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMESTAMP_COMPARE_TOGGLE` reader - 6:6\\]
Timestamp Compare Toggle mode: 0=TS_COMP is in non-toggle mode, 1=TS_COMP is in toggle mode"]
pub type TimestampCompareToggleR = crate::BitReader;
#[doc = "Field `TIMESTAMP_COMPARE_TOGGLE` writer - 6:6\\]
Timestamp Compare Toggle mode: 0=TS_COMP is in non-toggle mode, 1=TS_COMP is in toggle mode"]
pub type TimestampCompareToggleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMESTAMP_PPM_DIRECTION` reader - 7:7\\]
Timestamp PPM Direction"]
pub type TimestampPpmDirectionR = crate::BitReader;
#[doc = "Field `TIMESTAMP_PPM_DIRECTION` writer - 7:7\\]
Timestamp PPM Direction"]
pub type TimestampPpmDirectionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDWARE_PUSH_1` reader - 8:8\\]
Hardware push 1 enable"]
pub type HardwarePush1R = crate::BitReader;
#[doc = "Field `HARDWARE_PUSH_1` writer - 8:8\\]
Hardware push 1 enable"]
pub type HardwarePush1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDWARE_PUSH_2` reader - 9:9\\]
Hardware push 2 enable"]
pub type HardwarePush2R = crate::BitReader;
#[doc = "Field `HARDWARE_PUSH_2` writer - 9:9\\]
Hardware push 2 enable"]
pub type HardwarePush2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDWARE_PUSH_3` reader - 10:10\\]
Hardware push 3 enable"]
pub type HardwarePush3R = crate::BitReader;
#[doc = "Field `HARDWARE_PUSH_3` writer - 10:10\\]
Hardware push 3 enable"]
pub type HardwarePush3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDWARE_PUSH_4` reader - 11:11\\]
Hardware push 4 enable"]
pub type HardwarePush4R = crate::BitReader;
#[doc = "Field `HARDWARE_PUSH_4` writer - 11:11\\]
Hardware push 4 enable"]
pub type HardwarePush4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDWARE_PUSH_5` reader - 12:12\\]
Hardware push 5 enable"]
pub type HardwarePush5R = crate::BitReader;
#[doc = "Field `HARDWARE_PUSH_5` writer - 12:12\\]
Hardware push 5 enable"]
pub type HardwarePush5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDWARE_PUSH_6` reader - 13:13\\]
Hardware push 6 enable"]
pub type HardwarePush6R = crate::BitReader;
#[doc = "Field `HARDWARE_PUSH_6` writer - 13:13\\]
Hardware push 6 enable"]
pub type HardwarePush6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDWARE_PUSH_7` reader - 14:14\\]
Hardware push 7 enable"]
pub type HardwarePush7R = crate::BitReader;
#[doc = "Field `HARDWARE_PUSH_7` writer - 14:14\\]
Hardware push 7 enable"]
pub type HardwarePush7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDWARE_PUSH_8` reader - 15:15\\]
Hardware push 8 enable"]
pub type HardwarePush8R = crate::BitReader;
#[doc = "Field `HARDWARE_PUSH_8` writer - 15:15\\]
Hardware push 8 enable"]
pub type HardwarePush8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_PRODUCES_NO` reader - 16:16\\]
Receive Produces no Events"]
pub type ReceiveProducesNoR = crate::BitReader;
#[doc = "Field `RECEIVE_PRODUCES_NO` writer - 16:16\\]
Receive Produces no Events"]
pub type ReceiveProducesNoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_FOR_GENF` reader - 17:17\\]
Enable for GENF clear when length is zero"]
pub type EnableForGenfR = crate::BitReader;
#[doc = "Field `ENABLE_FOR_GENF` writer - 17:17\\]
Enable for GENF clear when length is zero"]
pub type EnableForGenfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_SYNC_OUTPUT_TIMESTAMP` reader - 31:28\\]
TS_SYNC output timestamp counter bit select"]
pub type TsSyncOutputTimestampR = crate::FieldReader;
#[doc = "Field `TS_SYNC_OUTPUT_TIMESTAMP` writer - 31:28\\]
TS_SYNC output timestamp counter bit select"]
pub type TsSyncOutputTimestampW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time sync enable"]
    #[inline(always)]
    pub fn time_sync_enable(&self) -> TimeSyncEnableR {
        TimeSyncEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt test"]
    #[inline(always)]
    pub fn interrupt_test(&self) -> InterruptTestR {
        InterruptTestR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TS_COMP polarity"]
    #[inline(always)]
    pub fn ts_comp_polarity(&self) -> TsCompPolarityR {
        TsCompPolarityR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Host Receive Timestamp Enable"]
    #[inline(always)]
    pub fn host_receive_timestamp(&self) -> HostReceiveTimestampR {
        HostReceiveTimestampR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sequence Enable"]
    #[inline(always)]
    pub fn sequence_enable(&self) -> SequenceEnableR {
        SequenceEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Timestamp mode"]
    #[inline(always)]
    pub fn timestamp_mode(&self) -> TimestampModeR {
        TimestampModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Timestamp Compare Toggle mode: 0=TS_COMP is in non-toggle mode, 1=TS_COMP is in toggle mode"]
    #[inline(always)]
    pub fn timestamp_compare_toggle(&self) -> TimestampCompareToggleR {
        TimestampCompareToggleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Timestamp PPM Direction"]
    #[inline(always)]
    pub fn timestamp_ppm_direction(&self) -> TimestampPpmDirectionR {
        TimestampPpmDirectionR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Hardware push 1 enable"]
    #[inline(always)]
    pub fn hardware_push_1(&self) -> HardwarePush1R {
        HardwarePush1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Hardware push 2 enable"]
    #[inline(always)]
    pub fn hardware_push_2(&self) -> HardwarePush2R {
        HardwarePush2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Hardware push 3 enable"]
    #[inline(always)]
    pub fn hardware_push_3(&self) -> HardwarePush3R {
        HardwarePush3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Hardware push 4 enable"]
    #[inline(always)]
    pub fn hardware_push_4(&self) -> HardwarePush4R {
        HardwarePush4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Hardware push 5 enable"]
    #[inline(always)]
    pub fn hardware_push_5(&self) -> HardwarePush5R {
        HardwarePush5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Hardware push 6 enable"]
    #[inline(always)]
    pub fn hardware_push_6(&self) -> HardwarePush6R {
        HardwarePush6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Hardware push 7 enable"]
    #[inline(always)]
    pub fn hardware_push_7(&self) -> HardwarePush7R {
        HardwarePush7R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Hardware push 8 enable"]
    #[inline(always)]
    pub fn hardware_push_8(&self) -> HardwarePush8R {
        HardwarePush8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Receive Produces no Events"]
    #[inline(always)]
    pub fn receive_produces_no(&self) -> ReceiveProducesNoR {
        ReceiveProducesNoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable for GENF clear when length is zero"]
    #[inline(always)]
    pub fn enable_for_genf(&self) -> EnableForGenfR {
        EnableForGenfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
TS_SYNC output timestamp counter bit select"]
    #[inline(always)]
    pub fn ts_sync_output_timestamp(&self) -> TsSyncOutputTimestampR {
        TsSyncOutputTimestampR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_sync_enable(&mut self) -> TimeSyncEnableW<CptsControlRegSpec> {
        TimeSyncEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt test"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_test(&mut self) -> InterruptTestW<CptsControlRegSpec> {
        InterruptTestW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TS_COMP polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ts_comp_polarity(&mut self) -> TsCompPolarityW<CptsControlRegSpec> {
        TsCompPolarityW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Host Receive Timestamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn host_receive_timestamp(&mut self) -> HostReceiveTimestampW<CptsControlRegSpec> {
        HostReceiveTimestampW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Sequence Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sequence_enable(&mut self) -> SequenceEnableW<CptsControlRegSpec> {
        SequenceEnableW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Timestamp mode"]
    #[inline(always)]
    #[must_use]
    pub fn timestamp_mode(&mut self) -> TimestampModeW<CptsControlRegSpec> {
        TimestampModeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Timestamp Compare Toggle mode: 0=TS_COMP is in non-toggle mode, 1=TS_COMP is in toggle mode"]
    #[inline(always)]
    #[must_use]
    pub fn timestamp_compare_toggle(&mut self) -> TimestampCompareToggleW<CptsControlRegSpec> {
        TimestampCompareToggleW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Timestamp PPM Direction"]
    #[inline(always)]
    #[must_use]
    pub fn timestamp_ppm_direction(&mut self) -> TimestampPpmDirectionW<CptsControlRegSpec> {
        TimestampPpmDirectionW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Hardware push 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_push_1(&mut self) -> HardwarePush1W<CptsControlRegSpec> {
        HardwarePush1W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Hardware push 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_push_2(&mut self) -> HardwarePush2W<CptsControlRegSpec> {
        HardwarePush2W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Hardware push 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_push_3(&mut self) -> HardwarePush3W<CptsControlRegSpec> {
        HardwarePush3W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Hardware push 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_push_4(&mut self) -> HardwarePush4W<CptsControlRegSpec> {
        HardwarePush4W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Hardware push 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_push_5(&mut self) -> HardwarePush5W<CptsControlRegSpec> {
        HardwarePush5W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Hardware push 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_push_6(&mut self) -> HardwarePush6W<CptsControlRegSpec> {
        HardwarePush6W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Hardware push 7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_push_7(&mut self) -> HardwarePush7W<CptsControlRegSpec> {
        HardwarePush7W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Hardware push 8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_push_8(&mut self) -> HardwarePush8W<CptsControlRegSpec> {
        HardwarePush8W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Receive Produces no Events"]
    #[inline(always)]
    #[must_use]
    pub fn receive_produces_no(&mut self) -> ReceiveProducesNoW<CptsControlRegSpec> {
        ReceiveProducesNoW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable for GENF clear when length is zero"]
    #[inline(always)]
    #[must_use]
    pub fn enable_for_genf(&mut self) -> EnableForGenfW<CptsControlRegSpec> {
        EnableForGenfW::new(self, 17)
    }
    #[doc = "Bits 28:31 - 31:28\\]
TS_SYNC output timestamp counter bit select"]
    #[inline(always)]
    #[must_use]
    pub fn ts_sync_output_timestamp(&mut self) -> TsSyncOutputTimestampW<CptsControlRegSpec> {
        TsSyncOutputTimestampW::new(self, 28)
    }
}
#[doc = "Time Sync Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsControlRegSpec;
impl crate::RegisterSpec for CptsControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_control_reg::R`](R) reader structure"]
impl crate::Readable for CptsControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_control_reg::W`](W) writer structure"]
impl crate::Writable for CptsControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_CONTROL_REG to value 0"]
impl crate::Resettable for CptsControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
