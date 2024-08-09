#[doc = "Register `CPSW_NC_STAT_0_RX_TOP_OF_FIFO_DROP` reader"]
pub type R = crate::R<CpswNcStat0RxTopOfFifoDropSpec>;
#[doc = "Register `CPSW_NC_STAT_0_RX_TOP_OF_FIFO_DROP` writer"]
pub type W = crate::W<CpswNcStat0RxTopOfFifoDropSpec>;
#[doc = "Field `RECEIVE_TOP_OF` reader - 31:0\\]
Receive Top of FIFO Drop"]
pub type ReceiveTopOfR = crate::FieldReader<u32>;
#[doc = "Field `RECEIVE_TOP_OF` writer - 31:0\\]
Receive Top of FIFO Drop"]
pub type ReceiveTopOfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Receive Top of FIFO Drop"]
    #[inline(always)]
    pub fn receive_top_of(&self) -> ReceiveTopOfR {
        ReceiveTopOfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Receive Top of FIFO Drop"]
    #[inline(always)]
    #[must_use]
    pub fn receive_top_of(&mut self) -> ReceiveTopOfW<CpswNcStat0RxTopOfFifoDropSpec> {
        ReceiveTopOfW::new(self, 0)
    }
}
#[doc = "Receive Top of FIFO Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rx_top_of_fifo_drop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rx_top_of_fifo_drop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat0RxTopOfFifoDropSpec;
impl crate::RegisterSpec for CpswNcStat0RxTopOfFifoDropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_0_rx_top_of_fifo_drop::R`](R) reader structure"]
impl crate::Readable for CpswNcStat0RxTopOfFifoDropSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_0_rx_top_of_fifo_drop::W`](W) writer structure"]
impl crate::Writable for CpswNcStat0RxTopOfFifoDropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_0_RX_TOP_OF_FIFO_DROP to value 0"]
impl crate::Resettable for CpswNcStat0RxTopOfFifoDropSpec {
    const RESET_VALUE: u32 = 0;
}
