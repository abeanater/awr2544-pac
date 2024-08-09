#[doc = "Register `SYS_RST_CAUSE` reader"]
pub type R = crate::R<SysRstCauseSpec>;
#[doc = "Register `SYS_RST_CAUSE` writer"]
pub type W = crate::W<SysRstCauseSpec>;
#[doc = "Field `cause` reader - 4:0\\]
System Reset Cause register 5'b01001 - POR reset 5'b01010 - Warm reset due to MSS_WDT 5'b01100 - Warm reset due to TOP_RMC:WARM_RESET_CONFIG 5'b01000 - External Pad reset 5'b11000 - Warm reset due to HSM_WDT"]
pub type CauseR = crate::FieldReader;
#[doc = "Field `cause` writer - 4:0\\]
System Reset Cause register 5'b01001 - POR reset 5'b01010 - Warm reset due to MSS_WDT 5'b01100 - Warm reset due to TOP_RMC:WARM_RESET_CONFIG 5'b01000 - External Pad reset 5'b11000 - Warm reset due to HSM_WDT"]
pub type CauseW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
System Reset Cause register 5'b01001 - POR reset 5'b01010 - Warm reset due to MSS_WDT 5'b01100 - Warm reset due to TOP_RMC:WARM_RESET_CONFIG 5'b01000 - External Pad reset 5'b11000 - Warm reset due to HSM_WDT"]
    #[inline(always)]
    pub fn cause(&self) -> CauseR {
        CauseR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
System Reset Cause register 5'b01001 - POR reset 5'b01010 - Warm reset due to MSS_WDT 5'b01100 - Warm reset due to TOP_RMC:WARM_RESET_CONFIG 5'b01000 - External Pad reset 5'b11000 - Warm reset due to HSM_WDT"]
    #[inline(always)]
    #[must_use]
    pub fn cause(&mut self) -> CauseW<SysRstCauseSpec> {
        CauseW::new(self, 0)
    }
}
#[doc = "SYS_RST_CAUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_rst_cause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_rst_cause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysRstCauseSpec;
impl crate::RegisterSpec for SysRstCauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_rst_cause::R`](R) reader structure"]
impl crate::Readable for SysRstCauseSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_rst_cause::W`](W) writer structure"]
impl crate::Writable for SysRstCauseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_RST_CAUSE to value 0"]
impl crate::Resettable for SysRstCauseSpec {
    const RESET_VALUE: u32 = 0;
}
