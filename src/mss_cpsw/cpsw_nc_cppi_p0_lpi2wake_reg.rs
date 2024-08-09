#[doc = "Register `CPSW_NC_CPPI_P0_LPI2WAKE_REG` reader"]
pub type R = crate::R<CpswNcCppiP0Lpi2wakeRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_LPI2WAKE_REG` writer"]
pub type W = crate::W<CpswNcCppiP0Lpi2wakeRegSpec>;
#[doc = "Field `PORT_0_EEE` reader - 23:0\\]
Port 0 EEE LPI to wake counter load value"]
pub type Port0EeeR = crate::FieldReader<u32>;
#[doc = "Field `PORT_0_EEE` writer - 23:0\\]
Port 0 EEE LPI to wake counter load value"]
pub type Port0EeeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Port 0 EEE LPI to wake counter load value"]
    #[inline(always)]
    pub fn port_0_eee(&self) -> Port0EeeR {
        Port0EeeR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Port 0 EEE LPI to wake counter load value"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_eee(&mut self) -> Port0EeeW<CpswNcCppiP0Lpi2wakeRegSpec> {
        Port0EeeW::new(self, 0)
    }
}
#[doc = "Port 0 EEE LPI to wake counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_lpi2wake_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_lpi2wake_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0Lpi2wakeRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0Lpi2wakeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_lpi2wake_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0Lpi2wakeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_lpi2wake_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0Lpi2wakeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_LPI2WAKE_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0Lpi2wakeRegSpec {
    const RESET_VALUE: u32 = 0;
}
