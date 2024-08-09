#[doc = "Register `CPSW_NC_CPPI_P0_RX_MAXLEN_REG` reader"]
pub type R = crate::R<CpswNcCppiP0RxMaxlenRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_RX_MAXLEN_REG` writer"]
pub type W = crate::W<CpswNcCppiP0RxMaxlenRegSpec>;
#[doc = "Field `RX_MAXIMUM_FRAME` reader - 13:0\\]
Rx Maximum Frame Length"]
pub type RxMaximumFrameR = crate::FieldReader<u16>;
#[doc = "Field `RX_MAXIMUM_FRAME` writer - 13:0\\]
Rx Maximum Frame Length"]
pub type RxMaximumFrameW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Rx Maximum Frame Length"]
    #[inline(always)]
    pub fn rx_maximum_frame(&self) -> RxMaximumFrameR {
        RxMaximumFrameR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Rx Maximum Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn rx_maximum_frame(&mut self) -> RxMaximumFrameW<CpswNcCppiP0RxMaxlenRegSpec> {
        RxMaximumFrameW::new(self, 0)
    }
}
#[doc = "CPPI Port 0 Receive Frame Max Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_maxlen_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_maxlen_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0RxMaxlenRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0RxMaxlenRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_rx_maxlen_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0RxMaxlenRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_rx_maxlen_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0RxMaxlenRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_RX_MAXLEN_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0RxMaxlenRegSpec {
    const RESET_VALUE: u32 = 0;
}
