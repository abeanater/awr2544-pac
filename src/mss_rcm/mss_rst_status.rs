#[doc = "Register `MSS_RST_STATUS` reader"]
pub type R = crate::R<MssRstStatusSpec>;
#[doc = "Register `MSS_RST_STATUS` writer"]
pub type W = crate::W<MssRstStatusSpec>;
#[doc = "Field `cause` reader - 15:0\\]
Has the status because of which reset has happened. Bit0: POR Reset Bit1: Warm Reset Bit2: STC Reset Bit3 Reset for CR5A and MSS_CR5A_VIM using MSS_RCM::MSS_CR5SSA_RST_CTRL Bit4: RESERVED Bit5: Reset for CR5A only using MSS_RCM::MSS_CR5A_RST_CTRL Bit6: RESERVED Bit7: Reset for CR5A and MSS_CR5A_VIM caused because of reset request by debugger in CR5A Bit8: RESERVED Bit9: Reset for CR5SS by the RESET FSM using MSS_CTRL::R5_CONTROL_RESET_FSM_TRIGGER"]
pub type CauseR = crate::FieldReader<u16>;
#[doc = "Field `cause` writer - 15:0\\]
Has the status because of which reset has happened. Bit0: POR Reset Bit1: Warm Reset Bit2: STC Reset Bit3 Reset for CR5A and MSS_CR5A_VIM using MSS_RCM::MSS_CR5SSA_RST_CTRL Bit4: RESERVED Bit5: Reset for CR5A only using MSS_RCM::MSS_CR5A_RST_CTRL Bit6: RESERVED Bit7: Reset for CR5A and MSS_CR5A_VIM caused because of reset request by debugger in CR5A Bit8: RESERVED Bit9: Reset for CR5SS by the RESET FSM using MSS_CTRL::R5_CONTROL_RESET_FSM_TRIGGER"]
pub type CauseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Has the status because of which reset has happened. Bit0: POR Reset Bit1: Warm Reset Bit2: STC Reset Bit3 Reset for CR5A and MSS_CR5A_VIM using MSS_RCM::MSS_CR5SSA_RST_CTRL Bit4: RESERVED Bit5: Reset for CR5A only using MSS_RCM::MSS_CR5A_RST_CTRL Bit6: RESERVED Bit7: Reset for CR5A and MSS_CR5A_VIM caused because of reset request by debugger in CR5A Bit8: RESERVED Bit9: Reset for CR5SS by the RESET FSM using MSS_CTRL::R5_CONTROL_RESET_FSM_TRIGGER"]
    #[inline(always)]
    pub fn cause(&self) -> CauseR {
        CauseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Has the status because of which reset has happened. Bit0: POR Reset Bit1: Warm Reset Bit2: STC Reset Bit3 Reset for CR5A and MSS_CR5A_VIM using MSS_RCM::MSS_CR5SSA_RST_CTRL Bit4: RESERVED Bit5: Reset for CR5A only using MSS_RCM::MSS_CR5A_RST_CTRL Bit6: RESERVED Bit7: Reset for CR5A and MSS_CR5A_VIM caused because of reset request by debugger in CR5A Bit8: RESERVED Bit9: Reset for CR5SS by the RESET FSM using MSS_CTRL::R5_CONTROL_RESET_FSM_TRIGGER"]
    #[inline(always)]
    #[must_use]
    pub fn cause(&mut self) -> CauseW<MssRstStatusSpec> {
        CauseW::new(self, 0)
    }
}
#[doc = "MSS_RST_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_rst_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_rst_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssRstStatusSpec;
impl crate::RegisterSpec for MssRstStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_rst_status::R`](R) reader structure"]
impl crate::Readable for MssRstStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_rst_status::W`](W) writer structure"]
impl crate::Writable for MssRstStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_RST_STATUS to value 0"]
impl crate::Resettable for MssRstStatusSpec {
    const RESET_VALUE: u32 = 0;
}
