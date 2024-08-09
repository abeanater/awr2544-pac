#[doc = "Register `CPSW_NC_TX_PRI0_MAXLEN_REG` reader"]
pub type R = crate::R<CpswNcTxPri0MaxlenRegSpec>;
#[doc = "Register `CPSW_NC_TX_PRI0_MAXLEN_REG` writer"]
pub type W = crate::W<CpswNcTxPri0MaxlenRegSpec>;
#[doc = "Field `TRANSMIT_PRIORITY_0` reader - 13:0\\]
Transmit Priority 0 Maximum Length"]
pub type TransmitPriority0R = crate::FieldReader<u16>;
#[doc = "Field `TRANSMIT_PRIORITY_0` writer - 13:0\\]
Transmit Priority 0 Maximum Length"]
pub type TransmitPriority0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Transmit Priority 0 Maximum Length"]
    #[inline(always)]
    pub fn transmit_priority_0(&self) -> TransmitPriority0R {
        TransmitPriority0R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Transmit Priority 0 Maximum Length"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_priority_0(&mut self) -> TransmitPriority0W<CpswNcTxPri0MaxlenRegSpec> {
        TransmitPriority0W::new(self, 0)
    }
}
#[doc = "Transmit Priority 0 Maximum Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_pri0_maxlen_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_pri0_maxlen_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcTxPri0MaxlenRegSpec;
impl crate::RegisterSpec for CpswNcTxPri0MaxlenRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_tx_pri0_maxlen_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcTxPri0MaxlenRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_tx_pri0_maxlen_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcTxPri0MaxlenRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_TX_PRI0_MAXLEN_REG to value 0"]
impl crate::Resettable for CpswNcTxPri0MaxlenRegSpec {
    const RESET_VALUE: u32 = 0;
}
