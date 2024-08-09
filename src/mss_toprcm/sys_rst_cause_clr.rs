#[doc = "Register `SYS_RST_CAUSE_CLR` reader"]
pub type R = crate::R<SysRstCauseClrSpec>;
#[doc = "Register `SYS_RST_CAUSE_CLR` writer"]
pub type W = crate::W<SysRstCauseClrSpec>;
#[doc = "Field `clear` reader - 0:0\\]
Write pulse bit field: System Reset Cause register Clear"]
pub type ClearR = crate::BitReader;
#[doc = "Field `clear` writer - 0:0\\]
Write pulse bit field: System Reset Cause register Clear"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write pulse bit field: System Reset Cause register Clear"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write pulse bit field: System Reset Cause register Clear"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<SysRstCauseClrSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "SYS_RST_CAUSE_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_rst_cause_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_rst_cause_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysRstCauseClrSpec;
impl crate::RegisterSpec for SysRstCauseClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_rst_cause_clr::R`](R) reader structure"]
impl crate::Readable for SysRstCauseClrSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_rst_cause_clr::W`](W) writer structure"]
impl crate::Writable for SysRstCauseClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_RST_CAUSE_CLR to value 0"]
impl crate::Resettable for SysRstCauseClrSpec {
    const RESET_VALUE: u32 = 0;
}
