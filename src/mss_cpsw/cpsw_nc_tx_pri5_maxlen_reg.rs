#[doc = "Register `CPSW_NC_TX_PRI5_MAXLEN_REG` reader"]
pub type R = crate::R<CpswNcTxPri5MaxlenRegSpec>;
#[doc = "Register `CPSW_NC_TX_PRI5_MAXLEN_REG` writer"]
pub type W = crate::W<CpswNcTxPri5MaxlenRegSpec>;
#[doc = "Field `TRANSMIT_PRIORITY_5` reader - 13:0\\]
Transmit Priority 5 Maximum Length"]
pub type TransmitPriority5R = crate::FieldReader<u16>;
#[doc = "Field `TRANSMIT_PRIORITY_5` writer - 13:0\\]
Transmit Priority 5 Maximum Length"]
pub type TransmitPriority5W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Transmit Priority 5 Maximum Length"]
    #[inline(always)]
    pub fn transmit_priority_5(&self) -> TransmitPriority5R {
        TransmitPriority5R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Transmit Priority 5 Maximum Length"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_priority_5(&mut self) -> TransmitPriority5W<CpswNcTxPri5MaxlenRegSpec> {
        TransmitPriority5W::new(self, 0)
    }
}
#[doc = "Transmit Priority 5 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri5_maxlen_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri5_maxlen_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcTxPri5MaxlenRegSpec;
impl crate::RegisterSpec for CpswNcTxPri5MaxlenRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_tx_pri5_maxlen_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcTxPri5MaxlenRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_tx_pri5_maxlen_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcTxPri5MaxlenRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_TX_PRI5_MAXLEN_REG to value 0"]
impl crate::Resettable for CpswNcTxPri5MaxlenRegSpec {
    const RESET_VALUE: u32 = 0;
}
