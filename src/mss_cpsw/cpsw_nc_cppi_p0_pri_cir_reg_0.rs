#[doc = "Register `CPSW_NC_CPPI_P0_PRI_CIR_REG_0` reader"]
pub type R = crate::R<CpswNcCppiP0PriCirReg0Spec>;
#[doc = "Register `CPSW_NC_CPPI_P0_PRI_CIR_REG_0` writer"]
pub type W = crate::W<CpswNcCppiP0PriCirReg0Spec>;
#[doc = "Field `PRIORITY_N_CIR` reader - 27:0\\]
Priority N CIR"]
pub type PriorityNCirR = crate::FieldReader<u32>;
#[doc = "Field `PRIORITY_N_CIR` writer - 27:0\\]
Priority N CIR"]
pub type PriorityNCirW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Priority N CIR"]
    #[inline(always)]
    pub fn priority_n_cir(&self) -> PriorityNCirR {
        PriorityNCirR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Priority N CIR"]
    #[inline(always)]
    #[must_use]
    pub fn priority_n_cir(&mut self) -> PriorityNCirW<CpswNcCppiP0PriCirReg0Spec> {
        PriorityNCirW::new(self, 0)
    }
}
#[doc = "CPPI Port 0 Rx Priority P Committed Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_cir_reg_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_cir_reg_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0PriCirReg0Spec;
impl crate::RegisterSpec for CpswNcCppiP0PriCirReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_pri_cir_reg_0::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0PriCirReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_pri_cir_reg_0::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0PriCirReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_PRI_CIR_REG_0 to value 0"]
impl crate::Resettable for CpswNcCppiP0PriCirReg0Spec {
    const RESET_VALUE: u32 = 0;
}
