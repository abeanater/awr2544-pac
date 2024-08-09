#[doc = "Register `CPSW_NC_STAT_1_RX_BOTTOM_OF_FIFO_DROP` reader"]
pub type R = crate::R<CpswNcStat1RxBottomOfFifoDropSpec>;
#[doc = "Register `CPSW_NC_STAT_1_RX_BOTTOM_OF_FIFO_DROP` writer"]
pub type W = crate::W<CpswNcStat1RxBottomOfFifoDropSpec>;
#[doc = "Field `RECEIVE_BOTTOM_OF` reader - 31:0\\]
Receive Bottom of FIFO Drop"]
pub type ReceiveBottomOfR = crate::FieldReader<u32>;
#[doc = "Field `RECEIVE_BOTTOM_OF` writer - 31:0\\]
Receive Bottom of FIFO Drop"]
pub type ReceiveBottomOfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Receive Bottom of FIFO Drop"]
    #[inline(always)]
    pub fn receive_bottom_of(&self) -> ReceiveBottomOfR {
        ReceiveBottomOfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Receive Bottom of FIFO Drop"]
    #[inline(always)]
    #[must_use]
    pub fn receive_bottom_of(&mut self) -> ReceiveBottomOfW<CpswNcStat1RxBottomOfFifoDropSpec> {
        ReceiveBottomOfW::new(self, 0)
    }
}
#[doc = "Receive Bottom of FIFO Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_rx_bottom_of_fifo_drop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_rx_bottom_of_fifo_drop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1RxBottomOfFifoDropSpec;
impl crate::RegisterSpec for CpswNcStat1RxBottomOfFifoDropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_rx_bottom_of_fifo_drop::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1RxBottomOfFifoDropSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_rx_bottom_of_fifo_drop::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1RxBottomOfFifoDropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_RX_BOTTOM_OF_FIFO_DROP to value 0"]
impl crate::Resettable for CpswNcStat1RxBottomOfFifoDropSpec {
    const RESET_VALUE: u32 = 0;
}
