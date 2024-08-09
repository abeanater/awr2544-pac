#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_CONTROL_REG` reader"]
pub type R = crate::R<CpswCpdmaRegsCpdmaControlRegSpec>;
#[doc = "Register `CPSW_CPDMA_REGS_CPDMA_CONTROL_REG` writer"]
pub type W = crate::W<CpswCpdmaRegsCpdmaControlRegSpec>;
#[doc = "Field `CPDMA_FHOST_QUEUE` reader - 0:0\\]
CPDMA FHost Queue Priority Type"]
pub type CpdmaFhostQueueR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_QUEUE` writer - 0:0\\]
CPDMA FHost Queue Priority Type"]
pub type CpdmaFhostQueueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_THOST_OWNERSHIP` reader - 1:1\\]
CPDMA THost Ownership Write Bit Value"]
pub type CpdmaThostOwnershipR = crate::BitReader;
#[doc = "Field `CPDMA_THOST_OWNERSHIP` writer - 1:1\\]
CPDMA THost Ownership Write Bit Value"]
pub type CpdmaThostOwnershipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_THOST_OFFSET_LENGTH` reader - 2:2\\]
CPDMA THost Offset/Length Word Write Block"]
pub type CpdmaThostOffsetLengthR = crate::BitReader;
#[doc = "Field `CPDMA_THOST_OFFSET_LENGTH` writer - 2:2\\]
CPDMA THost Offset/Length Word Write Block"]
pub type CpdmaThostOffsetLengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_COMMAND_IDLE` reader - 3:3\\]
CPDMA Command Idle"]
pub type CpdmaCommandIdleR = crate::BitReader;
#[doc = "Field `CPDMA_COMMAND_IDLE` writer - 3:3\\]
CPDMA Command Idle"]
pub type CpdmaCommandIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_THOST_COPY` reader - 4:4\\]
CPDMA THost Copy Error Frames"]
pub type CpdmaThostCopyR = crate::BitReader;
#[doc = "Field `CPDMA_THOST_COPY` writer - 4:4\\]
CPDMA THost Copy Error Frames"]
pub type CpdmaThostCopyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_THOST_VLAN` reader - 5:5\\]
CPDMA THost VLAN Encapsulated"]
pub type CpdmaThostVlanR = crate::BitReader;
#[doc = "Field `CPDMA_THOST_VLAN` writer - 5:5\\]
CPDMA THost VLAN Encapsulated"]
pub type CpdmaThostVlanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_THOST_TIMESTAMP` reader - 6:6\\]
CPDMA THost TimeStamp Encapsulated"]
pub type CpdmaThostTimestampR = crate::BitReader;
#[doc = "Field `CPDMA_THOST_TIMESTAMP` writer - 6:6\\]
CPDMA THost TimeStamp Encapsulated"]
pub type CpdmaThostTimestampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_CHANNEL_THREAD` reader - 7:7\\]
CPDMA Channel Thread Override Enable"]
pub type CpdmaChannelThreadR = crate::BitReader;
#[doc = "Field `CPDMA_CHANNEL_THREAD` writer - 7:7\\]
CPDMA Channel Thread Override Enable"]
pub type CpdmaChannelThreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_OWNERSHIP` reader - 8:8\\]
CPDMA FHost Ownership Write Bit Value"]
pub type CpdmaFhostOwnershipR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_OWNERSHIP` writer - 8:8\\]
CPDMA FHost Ownership Write Bit Value"]
pub type CpdmaFhostOwnershipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CPDMA FHost Queue Priority Type"]
    #[inline(always)]
    pub fn cpdma_fhost_queue(&self) -> CpdmaFhostQueueR {
        CpdmaFhostQueueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA THost Ownership Write Bit Value"]
    #[inline(always)]
    pub fn cpdma_thost_ownership(&self) -> CpdmaThostOwnershipR {
        CpdmaThostOwnershipR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CPDMA THost Offset/Length Word Write Block"]
    #[inline(always)]
    pub fn cpdma_thost_offset_length(&self) -> CpdmaThostOffsetLengthR {
        CpdmaThostOffsetLengthR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
CPDMA Command Idle"]
    #[inline(always)]
    pub fn cpdma_command_idle(&self) -> CpdmaCommandIdleR {
        CpdmaCommandIdleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
CPDMA THost Copy Error Frames"]
    #[inline(always)]
    pub fn cpdma_thost_copy(&self) -> CpdmaThostCopyR {
        CpdmaThostCopyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
CPDMA THost VLAN Encapsulated"]
    #[inline(always)]
    pub fn cpdma_thost_vlan(&self) -> CpdmaThostVlanR {
        CpdmaThostVlanR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
CPDMA THost TimeStamp Encapsulated"]
    #[inline(always)]
    pub fn cpdma_thost_timestamp(&self) -> CpdmaThostTimestampR {
        CpdmaThostTimestampR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
CPDMA Channel Thread Override Enable"]
    #[inline(always)]
    pub fn cpdma_channel_thread(&self) -> CpdmaChannelThreadR {
        CpdmaChannelThreadR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
CPDMA FHost Ownership Write Bit Value"]
    #[inline(always)]
    pub fn cpdma_fhost_ownership(&self) -> CpdmaFhostOwnershipR {
        CpdmaFhostOwnershipR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CPDMA FHost Queue Priority Type"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_queue(&mut self) -> CpdmaFhostQueueW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaFhostQueueW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA THost Ownership Write Bit Value"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_ownership(
        &mut self,
    ) -> CpdmaThostOwnershipW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaThostOwnershipW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CPDMA THost Offset/Length Word Write Block"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_offset_length(
        &mut self,
    ) -> CpdmaThostOffsetLengthW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaThostOffsetLengthW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
CPDMA Command Idle"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_command_idle(&mut self) -> CpdmaCommandIdleW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaCommandIdleW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
CPDMA THost Copy Error Frames"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_copy(&mut self) -> CpdmaThostCopyW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaThostCopyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
CPDMA THost VLAN Encapsulated"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_vlan(&mut self) -> CpdmaThostVlanW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaThostVlanW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
CPDMA THost TimeStamp Encapsulated"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_thost_timestamp(
        &mut self,
    ) -> CpdmaThostTimestampW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaThostTimestampW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
CPDMA Channel Thread Override Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_channel_thread(
        &mut self,
    ) -> CpdmaChannelThreadW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaChannelThreadW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
CPDMA FHost Ownership Write Bit Value"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_ownership(
        &mut self,
    ) -> CpdmaFhostOwnershipW<CpswCpdmaRegsCpdmaControlRegSpec> {
        CpdmaFhostOwnershipW::new(self, 8)
    }
}
#[doc = "CPDMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_regs_cpdma_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_regs_cpdma_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaRegsCpdmaControlRegSpec;
impl crate::RegisterSpec for CpswCpdmaRegsCpdmaControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_regs_cpdma_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaRegsCpdmaControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_regs_cpdma_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaRegsCpdmaControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_REGS_CPDMA_CONTROL_REG to value 0"]
impl crate::Resettable for CpswCpdmaRegsCpdmaControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
