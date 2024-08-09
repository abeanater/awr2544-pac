#[doc = "Register `CPSW_NC_CPPI_P0_BLK_CNT_REG` reader"]
pub type R = crate::R<CpswNcCppiP0BlkCntRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_BLK_CNT_REG` writer"]
pub type W = crate::W<CpswNcCppiP0BlkCntRegSpec>;
#[doc = "Field `PORT_0_RECEIVE` reader - 5:0\\]
Port 0 Receive Block Count Usage"]
pub type Port0ReceiveR = crate::FieldReader;
#[doc = "Field `PORT_0_RECEIVE` writer - 5:0\\]
Port 0 Receive Block Count Usage"]
pub type Port0ReceiveW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PORT_0_TRANSMIT` reader - 12:8\\]
Port 0 Transmit Block Count Usage"]
pub type Port0TransmitR = crate::FieldReader;
#[doc = "Field `PORT_0_TRANSMIT` writer - 12:8\\]
Port 0 Transmit Block Count Usage"]
pub type Port0TransmitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Port 0 Receive Block Count Usage"]
    #[inline(always)]
    pub fn port_0_receive(&self) -> Port0ReceiveR {
        Port0ReceiveR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Port 0 Transmit Block Count Usage"]
    #[inline(always)]
    pub fn port_0_transmit(&self) -> Port0TransmitR {
        Port0TransmitR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Port 0 Receive Block Count Usage"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_receive(&mut self) -> Port0ReceiveW<CpswNcCppiP0BlkCntRegSpec> {
        Port0ReceiveW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Port 0 Transmit Block Count Usage"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_transmit(&mut self) -> Port0TransmitW<CpswNcCppiP0BlkCntRegSpec> {
        Port0TransmitW::new(self, 8)
    }
}
#[doc = "CPPI Port 0 FIFO Block Usage Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_blk_cnt_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_blk_cnt_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0BlkCntRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0BlkCntRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_blk_cnt_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0BlkCntRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_blk_cnt_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0BlkCntRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_BLK_CNT_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0BlkCntRegSpec {
    const RESET_VALUE: u32 = 0;
}
