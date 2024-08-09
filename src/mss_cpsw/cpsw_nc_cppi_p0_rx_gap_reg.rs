#[doc = "Register `CPSW_NC_CPPI_P0_RX_GAP_REG` reader"]
pub type R = crate::R<CpswNcCppiP0RxGapRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_RX_GAP_REG` writer"]
pub type W = crate::W<CpswNcCppiP0RxGapRegSpec>;
#[doc = "Field `PORT_0_RECEIVE_1` reader - 7:0\\]
Port 0 Receive Gap Enable"]
pub type Port0Receive1R = crate::FieldReader;
#[doc = "Field `PORT_0_RECEIVE_1` writer - 7:0\\]
Port 0 Receive Gap Enable"]
pub type Port0Receive1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PORT_0_RECEIVE` reader - 25:16\\]
Port 0 Receive Gap Count"]
pub type Port0ReceiveR = crate::FieldReader<u16>;
#[doc = "Field `PORT_0_RECEIVE` writer - 25:16\\]
Port 0 Receive Gap Count"]
pub type Port0ReceiveW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Port 0 Receive Gap Enable"]
    #[inline(always)]
    pub fn port_0_receive_1(&self) -> Port0Receive1R {
        Port0Receive1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Port 0 Receive Gap Count"]
    #[inline(always)]
    pub fn port_0_receive(&self) -> Port0ReceiveR {
        Port0ReceiveR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Port 0 Receive Gap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_receive_1(&mut self) -> Port0Receive1W<CpswNcCppiP0RxGapRegSpec> {
        Port0Receive1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Port 0 Receive Gap Count"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_receive(&mut self) -> Port0ReceiveW<CpswNcCppiP0RxGapRegSpec> {
        Port0ReceiveW::new(self, 16)
    }
}
#[doc = "Port 0 Receive Gap Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_rx_gap_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_rx_gap_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0RxGapRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0RxGapRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_rx_gap_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0RxGapRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_rx_gap_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0RxGapRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_RX_GAP_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0RxGapRegSpec {
    const RESET_VALUE: u32 = 0;
}
