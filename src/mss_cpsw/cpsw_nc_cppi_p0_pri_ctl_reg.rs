#[doc = "Register `CPSW_NC_CPPI_P0_PRI_CTL_REG` reader"]
pub type R = crate::R<CpswNcCppiP0PriCtlRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_PRI_CTL_REG` writer"]
pub type W = crate::W<CpswNcCppiP0PriCtlRegSpec>;
#[doc = "Field `RECEIVE_PRIORITY_TYPE` reader - 8:8\\]
Receive Priority Type"]
pub type ReceivePriorityTypeR = crate::BitReader;
#[doc = "Field `RECEIVE_PRIORITY_TYPE` writer - 8:8\\]
Receive Priority Type"]
pub type ReceivePriorityTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_PRIORITY_BASED` reader - 23:16\\]
Receive Priority Based Flow Control Enable (per priority)"]
pub type ReceivePriorityBasedR = crate::FieldReader;
#[doc = "Field `RECEIVE_PRIORITY_BASED` writer - 23:16\\]
Receive Priority Based Flow Control Enable (per priority)"]
pub type ReceivePriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Receive Priority Type"]
    #[inline(always)]
    pub fn receive_priority_type(&self) -> ReceivePriorityTypeR {
        ReceivePriorityTypeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Receive Priority Based Flow Control Enable (per priority)"]
    #[inline(always)]
    pub fn receive_priority_based(&self) -> ReceivePriorityBasedR {
        ReceivePriorityBasedR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Receive Priority Type"]
    #[inline(always)]
    #[must_use]
    pub fn receive_priority_type(&mut self) -> ReceivePriorityTypeW<CpswNcCppiP0PriCtlRegSpec> {
        ReceivePriorityTypeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Receive Priority Based Flow Control Enable (per priority)"]
    #[inline(always)]
    #[must_use]
    pub fn receive_priority_based(&mut self) -> ReceivePriorityBasedW<CpswNcCppiP0PriCtlRegSpec> {
        ReceivePriorityBasedW::new(self, 16)
    }
}
#[doc = "CPPI Port 0 Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_pri_ctl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_pri_ctl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0PriCtlRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0PriCtlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_pri_ctl_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0PriCtlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_pri_ctl_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0PriCtlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_PRI_CTL_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0PriCtlRegSpec {
    const RESET_VALUE: u32 = 0;
}
