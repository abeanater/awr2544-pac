#[doc = "Register `MSTIDDIAGCTRL` reader"]
pub type R = crate::R<MstiddiagctrlSpec>;
#[doc = "Register `MSTIDDIAGCTRL` writer"]
pub type W = crate::W<MstiddiagctrlSpec>;
#[doc = "Field `DIAG_MODE_EN` reader - 3:0\\]
MasterID compare logic diagnostic mode enable bits; 4-bit key for enabling the master-id registers compare logic. Readable in both user and privileged modes. Writable only in privileged mode 1010 = Master-id compare diagnostic mode is enabled. others = Master-id compare diagnostic mode is disabled."]
pub type DiagModeEnR = crate::FieldReader;
#[doc = "Field `DIAG_MODE_EN` writer - 3:0\\]
MasterID compare logic diagnostic mode enable bits; 4-bit key for enabling the master-id registers compare logic. Readable in both user and privileged modes. Writable only in privileged mode 1010 = Master-id compare diagnostic mode is enabled. others = Master-id compare diagnostic mode is disabled."]
pub type DiagModeEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved1` reader - 7:4\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 7:4\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIAG_CMP_VALUE` reader - 11:8\\]
MasterID diagnostic mode control register bits; 4-bit data which is compared with the master-id register of all defined frames during diagnostic mode. Any error in compare logic is indicated through AERROR output from PCR. Readable in both user and privileged modes. Reads the programmed value in diagnostic compare value field. Writable only in privileged mode"]
pub type DiagCmpValueR = crate::FieldReader;
#[doc = "Field `DIAG_CMP_VALUE` writer - 11:8\\]
MasterID diagnostic mode control register bits; 4-bit data which is compared with the master-id register of all defined frames during diagnostic mode. Any error in compare logic is indicated through AERROR output from PCR. Readable in both user and privileged modes. Reads the programmed value in diagnostic compare value field. Writable only in privileged mode"]
pub type DiagCmpValueW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved2` reader - 31:12\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `Reserved2` writer - 31:12\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MasterID compare logic diagnostic mode enable bits; 4-bit key for enabling the master-id registers compare logic. Readable in both user and privileged modes. Writable only in privileged mode 1010 = Master-id compare diagnostic mode is enabled. others = Master-id compare diagnostic mode is disabled."]
    #[inline(always)]
    pub fn diag_mode_en(&self) -> DiagModeEnR {
        DiagModeEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
MasterID diagnostic mode control register bits; 4-bit data which is compared with the master-id register of all defined frames during diagnostic mode. Any error in compare logic is indicated through AERROR output from PCR. Readable in both user and privileged modes. Reads the programmed value in diagnostic compare value field. Writable only in privileged mode"]
    #[inline(always)]
    pub fn diag_cmp_value(&self) -> DiagCmpValueR {
        DiagCmpValueR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
MasterID compare logic diagnostic mode enable bits; 4-bit key for enabling the master-id registers compare logic. Readable in both user and privileged modes. Writable only in privileged mode 1010 = Master-id compare diagnostic mode is enabled. others = Master-id compare diagnostic mode is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn diag_mode_en(&mut self) -> DiagModeEnW<MstiddiagctrlSpec> {
        DiagModeEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MstiddiagctrlSpec> {
        Reserved1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
MasterID diagnostic mode control register bits; 4-bit data which is compared with the master-id register of all defined frames during diagnostic mode. Any error in compare logic is indicated through AERROR output from PCR. Readable in both user and privileged modes. Reads the programmed value in diagnostic compare value field. Writable only in privileged mode"]
    #[inline(always)]
    #[must_use]
    pub fn diag_cmp_value(&mut self) -> DiagCmpValueW<MstiddiagctrlSpec> {
        DiagCmpValueW::new(self, 8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<MstiddiagctrlSpec> {
        Reserved2W::new(self, 12)
    }
}
#[doc = "MasterID Diagnostic Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstiddiagctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstiddiagctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstiddiagctrlSpec;
impl crate::RegisterSpec for MstiddiagctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstiddiagctrl::R`](R) reader structure"]
impl crate::Readable for MstiddiagctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mstiddiagctrl::W`](W) writer structure"]
impl crate::Writable for MstiddiagctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTIDDIAGCTRL to value 0"]
impl crate::Resettable for MstiddiagctrlSpec {
    const RESET_VALUE: u32 = 0;
}
