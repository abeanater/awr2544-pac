#[doc = "Register `CPSW_NC_CPPI_P0_FIFO_STATUS_REG` reader"]
pub type R = crate::R<CpswNcCppiP0FifoStatusRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_FIFO_STATUS_REG` writer"]
pub type W = crate::W<CpswNcCppiP0FifoStatusRegSpec>;
#[doc = "Field `PORT_0_FIFO` reader - 7:0\\]
Port 0 FIFO Status"]
pub type Port0FifoR = crate::FieldReader;
#[doc = "Field `PORT_0_FIFO` writer - 7:0\\]
Port 0 FIFO Status"]
pub type Port0FifoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Port 0 FIFO Status"]
    #[inline(always)]
    pub fn port_0_fifo(&self) -> Port0FifoR {
        Port0FifoR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Port 0 FIFO Status"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_fifo(&mut self) -> Port0FifoW<CpswNcCppiP0FifoStatusRegSpec> {
        Port0FifoW::new(self, 0)
    }
}
#[doc = "Port 0 FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_fifo_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_fifo_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0FifoStatusRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0FifoStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_fifo_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0FifoStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_fifo_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0FifoStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_FIFO_STATUS_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0FifoStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
