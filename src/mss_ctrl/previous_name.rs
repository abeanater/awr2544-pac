#[doc = "Register `PREVIOUS_NAME` reader"]
pub type R = crate::R<PreviousNameSpec>;
#[doc = "Register `PREVIOUS_NAME` writer"]
pub type W = crate::W<PreviousNameSpec>;
#[doc = "Field `crc_mode` reader - 0:0\\]
1'b0 : CRC16CCITT 1'b1 : ETHERNET CRC32"]
pub type CrcModeR = crate::BitReader;
#[doc = "Field `crc_mode` writer - 0:0\\]
1'b0 : CRC16CCITT 1'b1 : ETHERNET CRC32"]
pub type CrcModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `crc_enable` reader - 4:4\\]
1'b0: CRC disable 1'b1: CRC enable"]
pub type CrcEnableR = crate::BitReader;
#[doc = "Field `crc_enable` writer - 4:4\\]
1'b0: CRC disable 1'b1: CRC enable"]
pub type CrcEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clr` reader - 8:8\\]
Write pulse bit field: Write 1'b1 whenever ping_pong_sel needs to be cleared."]
pub type ClrR = crate::BitReader;
#[doc = "Field `clr` writer - 8:8\\]
Write pulse bit field: Write 1'b1 whenever ping_pong_sel needs to be cleared."]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `set` reader - 12:12\\]
Write pulse bit field: Write 1'b1 whenever ping_pong_sel needs to be set."]
pub type SetR = crate::BitReader;
#[doc = "Field `set` writer - 12:12\\]
Write pulse bit field: Write 1'b1 whenever ping_pong_sel needs to be set."]
pub type SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPSW_CLK_STOP_REQ` reader - 16:16\\]
CPSW clock stop req"]
pub type CpswClkStopReqR = crate::BitReader;
#[doc = "Field `CPSW_CLK_STOP_REQ` writer - 16:16\\]
CPSW clock stop req"]
pub type CpswClkStopReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pp_switch_ovr` reader - 20:20\\]
Writing 1'b1 takes override on the CPWS ping-pong switch.Ping-pong switch doesn t happen even after reading the configured end-address."]
pub type PpSwitchOvrR = crate::BitReader;
#[doc = "Field `pp_switch_ovr` writer - 20:20\\]
Writing 1'b1 takes override on the CPWS ping-pong switch.Ping-pong switch doesn t happen even after reading the configured end-address."]
pub type PpSwitchOvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbg_pp_switch_en` reader - 24:24\\]
Writing 1'b1 will allow CPSW ping-pong switch to happen during debugger reads (through JTAG)."]
pub type DbgPpSwitchEnR = crate::BitReader;
#[doc = "Field `dbg_pp_switch_en` writer - 24:24\\]
Writing 1'b1 will allow CPSW ping-pong switch to happen during debugger reads (through JTAG)."]
pub type DbgPpSwitchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwreg2` reader - 31:25\\]
HW reserved Register"]
pub type Hwreg2R = crate::FieldReader;
#[doc = "Field `hwreg2` writer - 31:25\\]
HW reserved Register"]
pub type Hwreg2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0 : CRC16CCITT 1'b1 : ETHERNET CRC32"]
    #[inline(always)]
    pub fn crc_mode(&self) -> CrcModeR {
        CrcModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1'b0: CRC disable 1'b1: CRC enable"]
    #[inline(always)]
    pub fn crc_enable(&self) -> CrcEnableR {
        CrcEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Write pulse bit field: Write 1'b1 whenever ping_pong_sel needs to be cleared."]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Write pulse bit field: Write 1'b1 whenever ping_pong_sel needs to be set."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
CPSW clock stop req"]
    #[inline(always)]
    pub fn cpsw_clk_stop_req(&self) -> CpswClkStopReqR {
        CpswClkStopReqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 takes override on the CPWS ping-pong switch.Ping-pong switch doesn t happen even after reading the configured end-address."]
    #[inline(always)]
    pub fn pp_switch_ovr(&self) -> PpSwitchOvrR {
        PpSwitchOvrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 1'b1 will allow CPSW ping-pong switch to happen during debugger reads (through JTAG)."]
    #[inline(always)]
    pub fn dbg_pp_switch_en(&self) -> DbgPpSwitchEnR {
        DbgPpSwitchEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
HW reserved Register"]
    #[inline(always)]
    pub fn hwreg2(&self) -> Hwreg2R {
        Hwreg2R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0 : CRC16CCITT 1'b1 : ETHERNET CRC32"]
    #[inline(always)]
    #[must_use]
    pub fn crc_mode(&mut self) -> CrcModeW<PreviousNameSpec> {
        CrcModeW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1'b0: CRC disable 1'b1: CRC enable"]
    #[inline(always)]
    #[must_use]
    pub fn crc_enable(&mut self) -> CrcEnableW<PreviousNameSpec> {
        CrcEnableW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Write pulse bit field: Write 1'b1 whenever ping_pong_sel needs to be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<PreviousNameSpec> {
        ClrW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Write pulse bit field: Write 1'b1 whenever ping_pong_sel needs to be set."]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<PreviousNameSpec> {
        SetW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
CPSW clock stop req"]
    #[inline(always)]
    #[must_use]
    pub fn cpsw_clk_stop_req(&mut self) -> CpswClkStopReqW<PreviousNameSpec> {
        CpswClkStopReqW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 takes override on the CPWS ping-pong switch.Ping-pong switch doesn t happen even after reading the configured end-address."]
    #[inline(always)]
    #[must_use]
    pub fn pp_switch_ovr(&mut self) -> PpSwitchOvrW<PreviousNameSpec> {
        PpSwitchOvrW::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 1'b1 will allow CPSW ping-pong switch to happen during debugger reads (through JTAG)."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_pp_switch_en(&mut self) -> DbgPpSwitchEnW<PreviousNameSpec> {
        DbgPpSwitchEnW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
HW reserved Register"]
    #[inline(always)]
    #[must_use]
    pub fn hwreg2(&mut self) -> Hwreg2W<PreviousNameSpec> {
        Hwreg2W::new(self, 25)
    }
}
#[doc = "PREVIOUS_NAME\n\nYou can [`read`](crate::Reg::read) this register and get [`previous_name::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`previous_name::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PreviousNameSpec;
impl crate::RegisterSpec for PreviousNameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`previous_name::R`](R) reader structure"]
impl crate::Readable for PreviousNameSpec {}
#[doc = "`write(|w| ..)` method takes [`previous_name::W`](W) writer structure"]
impl crate::Writable for PreviousNameSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREVIOUS_NAME to value 0"]
impl crate::Resettable for PreviousNameSpec {
    const RESET_VALUE: u32 = 0;
}
