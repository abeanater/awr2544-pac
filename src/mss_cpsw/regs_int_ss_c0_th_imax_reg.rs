#[doc = "Register `REGS_INT_SS_C0_TH_IMAX_REG` reader"]
pub type R = crate::R<RegsIntSsC0ThImaxRegSpec>;
#[doc = "Register `REGS_INT_SS_C0_TH_IMAX_REG` writer"]
pub type W = crate::W<RegsIntSsC0ThImaxRegSpec>;
#[doc = "Field `CORE_0_THOST` reader - 5:0\\]
Core 0 THost Interrupt Max Register Register"]
pub type Core0ThostR = crate::FieldReader;
#[doc = "Field `CORE_0_THOST` writer - 5:0\\]
Core 0 THost Interrupt Max Register Register"]
pub type Core0ThostW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Core 0 THost Interrupt Max Register Register"]
    #[inline(always)]
    pub fn core_0_thost(&self) -> Core0ThostR {
        Core0ThostR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Core 0 THost Interrupt Max Register Register"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_thost(&mut self) -> Core0ThostW<RegsIntSsC0ThImaxRegSpec> {
        Core0ThostW::new(self, 0)
    }
}
#[doc = "Core 0 THost Interrupt Max Register Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_th_imax_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_th_imax_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsIntSsC0ThImaxRegSpec;
impl crate::RegisterSpec for RegsIntSsC0ThImaxRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_int_ss_c0_th_imax_reg::R`](R) reader structure"]
impl crate::Readable for RegsIntSsC0ThImaxRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs_int_ss_c0_th_imax_reg::W`](W) writer structure"]
impl crate::Writable for RegsIntSsC0ThImaxRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_INT_SS_C0_TH_IMAX_REG to value 0"]
impl crate::Resettable for RegsIntSsC0ThImaxRegSpec {
    const RESET_VALUE: u32 = 0;
}
