#[doc = "Register `CPSW_NC_CPPI_P0_PRI_EIR_REG_5` reader"]
pub type R = crate::R<CpswNcCppiP0PriEirReg5Spec>;
#[doc = "Register `CPSW_NC_CPPI_P0_PRI_EIR_REG_5` writer"]
pub type W = crate::W<CpswNcCppiP0PriEirReg5Spec>;
#[doc = "Field `PRIORITY_N_EIR` reader - 27:0\\]
Priority N EIR"]
pub type PriorityNEirR = crate::FieldReader<u32>;
#[doc = "Field `PRIORITY_N_EIR` writer - 27:0\\]
Priority N EIR"]
pub type PriorityNEirW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Priority N EIR"]
    #[inline(always)]
    pub fn priority_n_eir(&self) -> PriorityNEirR {
        PriorityNEirR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Priority N EIR"]
    #[inline(always)]
    #[must_use]
    pub fn priority_n_eir(&mut self) -> PriorityNEirW<CpswNcCppiP0PriEirReg5Spec> {
        PriorityNEirW::new(self, 0)
    }
}
#[doc = "CPPI Port 0 Rx Priority P Excess Information Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_eir_reg_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_eir_reg_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0PriEirReg5Spec;
impl crate::RegisterSpec for CpswNcCppiP0PriEirReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_pri_eir_reg_5::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0PriEirReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_pri_eir_reg_5::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0PriEirReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_PRI_EIR_REG_5 to value 0"]
impl crate::Resettable for CpswNcCppiP0PriEirReg5Spec {
    const RESET_VALUE: u32 = 0;
}
