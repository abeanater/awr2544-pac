#[doc = "Register `TG0CTRL` reader"]
pub type R = crate::R<Tg0ctrlSpec>;
#[doc = "Register `TG0CTRL` writer"]
pub type W = crate::W<Tg0ctrlSpec>;
#[doc = "Field `PCURRENT` reader - 7:0\\]
transfer group pointer to current buffer. PCURRENT is read-only. PCURRENTx stores the address (0...127/255) of the buffer that is currently transferred or that will be transferred after a trigger event occurs (if PRSTx=0) or after the transfer group resumes from suspend to wait mode. If the transfer group switches mode from active transfer mode to ΓÇ£suspend to waitΓÇ¥, PCURRENTx keeps the address of the currently Suspended buffer. After the transfer group resumes from ΓÇ£suspend to waitΓÇ¥ mode the next buffer will be transferred. I.e. no buffer data is multiply transferred or not at all transferred due to ΓÇ£suspend to waitΓÇ¥ mode."]
pub type PcurrentR = crate::FieldReader;
#[doc = "Field `PCURRENT` writer - 7:0\\]
transfer group pointer to current buffer. PCURRENT is read-only. PCURRENTx stores the address (0...127/255) of the buffer that is currently transferred or that will be transferred after a trigger event occurs (if PRSTx=0) or after the transfer group resumes from suspend to wait mode. If the transfer group switches mode from active transfer mode to ΓÇ£suspend to waitΓÇ¥, PCURRENTx keeps the address of the currently Suspended buffer. After the transfer group resumes from ΓÇ£suspend to waitΓÇ¥ mode the next buffer will be transferred. I.e. no buffer data is multiply transferred or not at all transferred due to ΓÇ£suspend to waitΓÇ¥ mode."]
pub type PcurrentW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PSTART` reader - 15:8\\]
transfer group start address. PSTARTx stores the start address of the corresponding transfer group. The corresponding end address is inherently defined by the subsequent transfer groups start address minus one (PENDx\\[TGx\\]
= PSTARTx\\[TGx+1\\]-1). PSTARTx is copied into PCURRENTx when: o the transfer group is enabled o the end of the transfer group is reached during a transfer o a trigger event occurs while PRST is set to 1"]
pub type PstartR = crate::FieldReader;
#[doc = "Field `PSTART` writer - 15:8\\]
transfer group start address. PSTARTx stores the start address of the corresponding transfer group. The corresponding end address is inherently defined by the subsequent transfer groups start address minus one (PENDx\\[TGx\\]
= PSTARTx\\[TGx+1\\]-1). PSTARTx is copied into PCURRENTx when: o the transfer group is enabled o the end of the transfer group is reached during a transfer o a trigger event occurs while PRST is set to 1"]
pub type PstartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRIGSRC` reader - 19:16\\]
Trigger source. After reset the trigger sources of all transfer groups are disabled. Table 22. Trigger SourcesTRIGSRCx\\[3:0\\]
Type Description 0000b disabled 0001b EXT0 MibSPI external trigger source 0. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0010b EXT1 MibSPI external trigger source 1. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0011b EXT2 MibSPI external trigger source 2. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0100b EXT3 MibSPI external trigger source 3. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). ... ... ... 1110b EXT13 MibSPI external trigger source 13. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 1111b TICK MibSPI internal periodic event trigger. The tick counter can initiate periodic group transfers."]
pub type TrigsrcR = crate::FieldReader;
#[doc = "Field `TRIGSRC` writer - 19:16\\]
Trigger source. After reset the trigger sources of all transfer groups are disabled. Table 22. Trigger SourcesTRIGSRCx\\[3:0\\]
Type Description 0000b disabled 0001b EXT0 MibSPI external trigger source 0. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0010b EXT1 MibSPI external trigger source 1. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0011b EXT2 MibSPI external trigger source 2. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0100b EXT3 MibSPI external trigger source 3. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). ... ... ... 1110b EXT13 MibSPI external trigger source 13. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 1111b TICK MibSPI internal periodic event trigger. The tick counter can initiate periodic group transfers."]
pub type TrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIGEVT` reader - 23:20\\]
Type of trigger event. After reset, the trigger event types of all transfer groups are set to inactive TypesTRIGEVTx\\[3:0\\]
Type Description 0000b never 0001b rising edge A rising edge (0 to 1) at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0010b falling edge A falling edge (1 to 0) at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0011b both edges Rising and falling edges at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0100b reserved 0101b high-active Repetitive group transfer while trigger is high: While the selected trigger source (TRIGSRCx) is at a logic high level (1) the group transfer is continued and at the end of one group transfer restarted at the beginning. If ONESHOTx is set the transfer is performed only once. If the logic level changes to low (0) during an ongoing group transfer, the whole group transfer will be stopped. 0110b low-active Repetitive group transfer while trigger is low: While the selected trigger source (TRIGSRCx) is at a logic low level (0) the group transfer is continued and at the end of one restarted at the beginning. If ONESHOTx is set the transfer is performed only once. If the logic level changes to high (1) during an ongoing group transfer, the whole group transfer will be stopped. 0111b always A repetitive group transfer will be performed. Setting ONESHOTx allows a software controlled single transfer modea a. By setting the TRIGSRC to 0000b, the TRIGEVT to ALWAYS (0111b) the ONESHOT bit to 1. This allows a software control trigger on this Transfer Group. Then by setting the TGENA bit, the Transfer Group is immediately triggered. . If ONESHOTx is cleared a continuous mode for this Transfer Group is selected.1xxxb reserved"]
pub type TrigevtR = crate::FieldReader;
#[doc = "Field `TRIGEVT` writer - 23:20\\]
Type of trigger event. After reset, the trigger event types of all transfer groups are set to inactive TypesTRIGEVTx\\[3:0\\]
Type Description 0000b never 0001b rising edge A rising edge (0 to 1) at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0010b falling edge A falling edge (1 to 0) at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0011b both edges Rising and falling edges at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0100b reserved 0101b high-active Repetitive group transfer while trigger is high: While the selected trigger source (TRIGSRCx) is at a logic high level (1) the group transfer is continued and at the end of one group transfer restarted at the beginning. If ONESHOTx is set the transfer is performed only once. If the logic level changes to low (0) during an ongoing group transfer, the whole group transfer will be stopped. 0110b low-active Repetitive group transfer while trigger is low: While the selected trigger source (TRIGSRCx) is at a logic low level (0) the group transfer is continued and at the end of one restarted at the beginning. If ONESHOTx is set the transfer is performed only once. If the logic level changes to high (1) during an ongoing group transfer, the whole group transfer will be stopped. 0111b always A repetitive group transfer will be performed. Setting ONESHOTx allows a software controlled single transfer modea a. By setting the TRIGSRC to 0000b, the TRIGEVT to ALWAYS (0111b) the ONESHOT bit to 1. This allows a software control trigger on this Transfer Group. Then by setting the TGENA bit, the Transfer Group is immediately triggered. . If ONESHOTx is cleared a continuous mode for this Transfer Group is selected.1xxxb reserved"]
pub type TrigevtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - 27:24\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 27:24\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TGTD` reader - 28:28\\]
Transfer group triggered. This bit is read-only. 1 =The transfer group has been triggered and is either currently in service or waiting for servicing. 0 =The corresponding transfer group has not been triggered or is no more waiting for service. Use the ΓÇ£TG IN SERVICEΓÇ¥ field in LTGPEND register to determine the exact Transfer Group being currently serviced"]
pub type TgtdR = crate::BitReader;
#[doc = "Field `TGTD` writer - 28:28\\]
Transfer group triggered. This bit is read-only. 1 =The transfer group has been triggered and is either currently in service or waiting for servicing. 0 =The corresponding transfer group has not been triggered or is no more waiting for service. Use the ΓÇ£TG IN SERVICEΓÇ¥ field in LTGPEND register to determine the exact Transfer Group being currently serviced"]
pub type TgtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST` reader - 29:29\\]
transfer group Pointer Reset mode. With PRST, the way of resolving trigger events during an ongoing transfer from the concerned transfer group can be configured. This bit is meaningful only for Level Triggered Transfer Groups. Edge triggered TGs cannot be re-started before their completion by another edge. PRST bit will have no effect on this behavior. 1 =The corresponding transfer group pointer (PCURRENTx) will be reset to the start address (PSTARTx) when a valid trigger event occurs at the selected trigger source while a transfer from the same transfer group is ongoing. I.e. every trigger event resets PCURRENTx no matter whether the concerned transfer group is in transfer mode or not. The trigger events have priority over the ongoing transfer. 0 =If a trigger event occurs during a transfer from the concerned transfer group, the event is ignored and is not stored internally. The transfer group transfer has priority over additional trigger events."]
pub type PrstR = crate::BitReader;
#[doc = "Field `PRST` writer - 29:29\\]
transfer group Pointer Reset mode. With PRST, the way of resolving trigger events during an ongoing transfer from the concerned transfer group can be configured. This bit is meaningful only for Level Triggered Transfer Groups. Edge triggered TGs cannot be re-started before their completion by another edge. PRST bit will have no effect on this behavior. 1 =The corresponding transfer group pointer (PCURRENTx) will be reset to the start address (PSTARTx) when a valid trigger event occurs at the selected trigger source while a transfer from the same transfer group is ongoing. I.e. every trigger event resets PCURRENTx no matter whether the concerned transfer group is in transfer mode or not. The trigger events have priority over the ongoing transfer. 0 =If a trigger event occurs during a transfer from the concerned transfer group, the event is ignored and is not stored internally. The transfer group transfer has priority over additional trigger events."]
pub type PrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONESHOT` reader - 30:30\\]
Single transfer for this Transfer Group group. 1 =A transfer from the corresponding transfer group will be performed only once (= one shot) after a valid trigger event at the selected trigger source. After the transfer is finished the TGENAx control bit will be cleared by the MibSPI and therefore no additional transfer can be triggered before the host enables the transfer group again. This oneshot mode ensures that after one group transfer the host has enough time to read the received data and to provide new transmit data. 0 =The corresponding transfer group initiates a transfer every time a trigger event occurs and TGENA is set."]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - 30:30\\]
Single transfer for this Transfer Group group. 1 =A transfer from the corresponding transfer group will be performed only once (= one shot) after a valid trigger event at the selected trigger source. After the transfer is finished the TGENAx control bit will be cleared by the MibSPI and therefore no additional transfer can be triggered before the host enables the transfer group again. This oneshot mode ensures that after one group transfer the host has enough time to read the received data and to provide new transmit data. 0 =The corresponding transfer group initiates a transfer every time a trigger event occurs and TGENA is set."]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGENA` reader - 31:31\\]
Transfer Group Enable. 1 =The corresponding transfer group is enabled. If the correct event (TRIGEVTx) occurs at the selected source (TRIGSRCx) a group transfer is initiated if no higher priority transfer group is in active transfer mode or if one or more higher priority Transfer Groups are in transfer suspend mode. If higher priority Transfer Groups (TG) are in transfer mode, then the newly enabled TG will wait till all of the higher priority TG transfers are completed. 0 =The corresponding transfer group is disabled. Disabling a transfer group while a transfer is ongoing, will finish the ongoing buffer transfer but not the whole group transfer"]
pub type TgenaR = crate::BitReader;
#[doc = "Field `TGENA` writer - 31:31\\]
Transfer Group Enable. 1 =The corresponding transfer group is enabled. If the correct event (TRIGEVTx) occurs at the selected source (TRIGSRCx) a group transfer is initiated if no higher priority transfer group is in active transfer mode or if one or more higher priority Transfer Groups are in transfer suspend mode. If higher priority Transfer Groups (TG) are in transfer mode, then the newly enabled TG will wait till all of the higher priority TG transfers are completed. 0 =The corresponding transfer group is disabled. Disabling a transfer group while a transfer is ongoing, will finish the ongoing buffer transfer but not the whole group transfer"]
pub type TgenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
transfer group pointer to current buffer. PCURRENT is read-only. PCURRENTx stores the address (0...127/255) of the buffer that is currently transferred or that will be transferred after a trigger event occurs (if PRSTx=0) or after the transfer group resumes from suspend to wait mode. If the transfer group switches mode from active transfer mode to ΓÇ£suspend to waitΓÇ¥, PCURRENTx keeps the address of the currently Suspended buffer. After the transfer group resumes from ΓÇ£suspend to waitΓÇ¥ mode the next buffer will be transferred. I.e. no buffer data is multiply transferred or not at all transferred due to ΓÇ£suspend to waitΓÇ¥ mode."]
    #[inline(always)]
    pub fn pcurrent(&self) -> PcurrentR {
        PcurrentR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
transfer group start address. PSTARTx stores the start address of the corresponding transfer group. The corresponding end address is inherently defined by the subsequent transfer groups start address minus one (PENDx\\[TGx\\]
= PSTARTx\\[TGx+1\\]-1). PSTARTx is copied into PCURRENTx when: o the transfer group is enabled o the end of the transfer group is reached during a transfer o a trigger event occurs while PRST is set to 1"]
    #[inline(always)]
    pub fn pstart(&self) -> PstartR {
        PstartR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Trigger source. After reset the trigger sources of all transfer groups are disabled. Table 22. Trigger SourcesTRIGSRCx\\[3:0\\]
Type Description 0000b disabled 0001b EXT0 MibSPI external trigger source 0. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0010b EXT1 MibSPI external trigger source 1. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0011b EXT2 MibSPI external trigger source 2. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0100b EXT3 MibSPI external trigger source 3. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). ... ... ... 1110b EXT13 MibSPI external trigger source 13. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 1111b TICK MibSPI internal periodic event trigger. The tick counter can initiate periodic group transfers."]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Type of trigger event. After reset, the trigger event types of all transfer groups are set to inactive TypesTRIGEVTx\\[3:0\\]
Type Description 0000b never 0001b rising edge A rising edge (0 to 1) at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0010b falling edge A falling edge (1 to 0) at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0011b both edges Rising and falling edges at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0100b reserved 0101b high-active Repetitive group transfer while trigger is high: While the selected trigger source (TRIGSRCx) is at a logic high level (1) the group transfer is continued and at the end of one group transfer restarted at the beginning. If ONESHOTx is set the transfer is performed only once. If the logic level changes to low (0) during an ongoing group transfer, the whole group transfer will be stopped. 0110b low-active Repetitive group transfer while trigger is low: While the selected trigger source (TRIGSRCx) is at a logic low level (0) the group transfer is continued and at the end of one restarted at the beginning. If ONESHOTx is set the transfer is performed only once. If the logic level changes to high (1) during an ongoing group transfer, the whole group transfer will be stopped. 0111b always A repetitive group transfer will be performed. Setting ONESHOTx allows a software controlled single transfer modea a. By setting the TRIGSRC to 0000b, the TRIGEVT to ALWAYS (0111b) the ONESHOT bit to 1. This allows a software control trigger on this Transfer Group. Then by setting the TGENA bit, the Transfer Group is immediately triggered. . If ONESHOTx is cleared a continuous mode for this Transfer Group is selected.1xxxb reserved"]
    #[inline(always)]
    pub fn trigevt(&self) -> TrigevtR {
        TrigevtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Transfer group triggered. This bit is read-only. 1 =The transfer group has been triggered and is either currently in service or waiting for servicing. 0 =The corresponding transfer group has not been triggered or is no more waiting for service. Use the ΓÇ£TG IN SERVICEΓÇ¥ field in LTGPEND register to determine the exact Transfer Group being currently serviced"]
    #[inline(always)]
    pub fn tgtd(&self) -> TgtdR {
        TgtdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
transfer group Pointer Reset mode. With PRST, the way of resolving trigger events during an ongoing transfer from the concerned transfer group can be configured. This bit is meaningful only for Level Triggered Transfer Groups. Edge triggered TGs cannot be re-started before their completion by another edge. PRST bit will have no effect on this behavior. 1 =The corresponding transfer group pointer (PCURRENTx) will be reset to the start address (PSTARTx) when a valid trigger event occurs at the selected trigger source while a transfer from the same transfer group is ongoing. I.e. every trigger event resets PCURRENTx no matter whether the concerned transfer group is in transfer mode or not. The trigger events have priority over the ongoing transfer. 0 =If a trigger event occurs during a transfer from the concerned transfer group, the event is ignored and is not stored internally. The transfer group transfer has priority over additional trigger events."]
    #[inline(always)]
    pub fn prst(&self) -> PrstR {
        PrstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Single transfer for this Transfer Group group. 1 =A transfer from the corresponding transfer group will be performed only once (= one shot) after a valid trigger event at the selected trigger source. After the transfer is finished the TGENAx control bit will be cleared by the MibSPI and therefore no additional transfer can be triggered before the host enables the transfer group again. This oneshot mode ensures that after one group transfer the host has enough time to read the received data and to provide new transmit data. 0 =The corresponding transfer group initiates a transfer every time a trigger event occurs and TGENA is set."]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Transfer Group Enable. 1 =The corresponding transfer group is enabled. If the correct event (TRIGEVTx) occurs at the selected source (TRIGSRCx) a group transfer is initiated if no higher priority transfer group is in active transfer mode or if one or more higher priority Transfer Groups are in transfer suspend mode. If higher priority Transfer Groups (TG) are in transfer mode, then the newly enabled TG will wait till all of the higher priority TG transfers are completed. 0 =The corresponding transfer group is disabled. Disabling a transfer group while a transfer is ongoing, will finish the ongoing buffer transfer but not the whole group transfer"]
    #[inline(always)]
    pub fn tgena(&self) -> TgenaR {
        TgenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
transfer group pointer to current buffer. PCURRENT is read-only. PCURRENTx stores the address (0...127/255) of the buffer that is currently transferred or that will be transferred after a trigger event occurs (if PRSTx=0) or after the transfer group resumes from suspend to wait mode. If the transfer group switches mode from active transfer mode to ΓÇ£suspend to waitΓÇ¥, PCURRENTx keeps the address of the currently Suspended buffer. After the transfer group resumes from ΓÇ£suspend to waitΓÇ¥ mode the next buffer will be transferred. I.e. no buffer data is multiply transferred or not at all transferred due to ΓÇ£suspend to waitΓÇ¥ mode."]
    #[inline(always)]
    #[must_use]
    pub fn pcurrent(&mut self) -> PcurrentW<Tg0ctrlSpec> {
        PcurrentW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
transfer group start address. PSTARTx stores the start address of the corresponding transfer group. The corresponding end address is inherently defined by the subsequent transfer groups start address minus one (PENDx\\[TGx\\]
= PSTARTx\\[TGx+1\\]-1). PSTARTx is copied into PCURRENTx when: o the transfer group is enabled o the end of the transfer group is reached during a transfer o a trigger event occurs while PRST is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn pstart(&mut self) -> PstartW<Tg0ctrlSpec> {
        PstartW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Trigger source. After reset the trigger sources of all transfer groups are disabled. Table 22. Trigger SourcesTRIGSRCx\\[3:0\\]
Type Description 0000b disabled 0001b EXT0 MibSPI external trigger source 0. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0010b EXT1 MibSPI external trigger source 1. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0011b EXT2 MibSPI external trigger source 2. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 0100b EXT3 MibSPI external trigger source 3. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). ... ... ... 1110b EXT13 MibSPI external trigger source 13. Source has to be defined individually for each Microcontroller derivative (e.g. HET I/O channel, event pin, etc.). 1111b TICK MibSPI internal periodic event trigger. The tick counter can initiate periodic group transfers."]
    #[inline(always)]
    #[must_use]
    pub fn trigsrc(&mut self) -> TrigsrcW<Tg0ctrlSpec> {
        TrigsrcW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Type of trigger event. After reset, the trigger event types of all transfer groups are set to inactive TypesTRIGEVTx\\[3:0\\]
Type Description 0000b never 0001b rising edge A rising edge (0 to 1) at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0010b falling edge A falling edge (1 to 0) at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0011b both edges Rising and falling edges at the selected trigger source (TRIGSRCx) initiates a transfer from the corresponding transfer group 0100b reserved 0101b high-active Repetitive group transfer while trigger is high: While the selected trigger source (TRIGSRCx) is at a logic high level (1) the group transfer is continued and at the end of one group transfer restarted at the beginning. If ONESHOTx is set the transfer is performed only once. If the logic level changes to low (0) during an ongoing group transfer, the whole group transfer will be stopped. 0110b low-active Repetitive group transfer while trigger is low: While the selected trigger source (TRIGSRCx) is at a logic low level (0) the group transfer is continued and at the end of one restarted at the beginning. If ONESHOTx is set the transfer is performed only once. If the logic level changes to high (1) during an ongoing group transfer, the whole group transfer will be stopped. 0111b always A repetitive group transfer will be performed. Setting ONESHOTx allows a software controlled single transfer modea a. By setting the TRIGSRC to 0000b, the TRIGEVT to ALWAYS (0111b) the ONESHOT bit to 1. This allows a software control trigger on this Transfer Group. Then by setting the TGENA bit, the Transfer Group is immediately triggered. . If ONESHOTx is cleared a continuous mode for this Transfer Group is selected.1xxxb reserved"]
    #[inline(always)]
    #[must_use]
    pub fn trigevt(&mut self) -> TrigevtW<Tg0ctrlSpec> {
        TrigevtW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Tg0ctrlSpec> {
        NuW::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Transfer group triggered. This bit is read-only. 1 =The transfer group has been triggered and is either currently in service or waiting for servicing. 0 =The corresponding transfer group has not been triggered or is no more waiting for service. Use the ΓÇ£TG IN SERVICEΓÇ¥ field in LTGPEND register to determine the exact Transfer Group being currently serviced"]
    #[inline(always)]
    #[must_use]
    pub fn tgtd(&mut self) -> TgtdW<Tg0ctrlSpec> {
        TgtdW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
transfer group Pointer Reset mode. With PRST, the way of resolving trigger events during an ongoing transfer from the concerned transfer group can be configured. This bit is meaningful only for Level Triggered Transfer Groups. Edge triggered TGs cannot be re-started before their completion by another edge. PRST bit will have no effect on this behavior. 1 =The corresponding transfer group pointer (PCURRENTx) will be reset to the start address (PSTARTx) when a valid trigger event occurs at the selected trigger source while a transfer from the same transfer group is ongoing. I.e. every trigger event resets PCURRENTx no matter whether the concerned transfer group is in transfer mode or not. The trigger events have priority over the ongoing transfer. 0 =If a trigger event occurs during a transfer from the concerned transfer group, the event is ignored and is not stored internally. The transfer group transfer has priority over additional trigger events."]
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PrstW<Tg0ctrlSpec> {
        PrstW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Single transfer for this Transfer Group group. 1 =A transfer from the corresponding transfer group will be performed only once (= one shot) after a valid trigger event at the selected trigger source. After the transfer is finished the TGENAx control bit will be cleared by the MibSPI and therefore no additional transfer can be triggered before the host enables the transfer group again. This oneshot mode ensures that after one group transfer the host has enough time to read the received data and to provide new transmit data. 0 =The corresponding transfer group initiates a transfer every time a trigger event occurs and TGENA is set."]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> OneshotW<Tg0ctrlSpec> {
        OneshotW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Transfer Group Enable. 1 =The corresponding transfer group is enabled. If the correct event (TRIGEVTx) occurs at the selected source (TRIGSRCx) a group transfer is initiated if no higher priority transfer group is in active transfer mode or if one or more higher priority Transfer Groups are in transfer suspend mode. If higher priority Transfer Groups (TG) are in transfer mode, then the newly enabled TG will wait till all of the higher priority TG transfers are completed. 0 =The corresponding transfer group is disabled. Disabling a transfer group while a transfer is ongoing, will finish the ongoing buffer transfer but not the whole group transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tgena(&mut self) -> TgenaW<Tg0ctrlSpec> {
        TgenaW::new(self, 31)
    }
}
#[doc = "MibSPI Transfer Group Control Register The number of transfer groups is scalable by design up to a maximum of 16. Depending on the implementation the number of transfer groups and hence the number of transfer group control register may vary. Each transfer group can be configured via one dedicated control register. The register description below shows one exemplary control register(x) which is identical for all transfer groups. E.g. the control register for transfer group 2 is named ΓÇ£TG2CTRLΓÇ¥ and is located at address base0+98h+4*2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tg0ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg0ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tg0ctrlSpec;
impl crate::RegisterSpec for Tg0ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tg0ctrl::R`](R) reader structure"]
impl crate::Readable for Tg0ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tg0ctrl::W`](W) writer structure"]
impl crate::Writable for Tg0ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TG0CTRL to value 0"]
impl crate::Resettable for Tg0ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
