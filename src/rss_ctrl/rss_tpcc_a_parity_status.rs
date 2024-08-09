#[doc = "Register `RSS_TPCC_A_PARITY_STATUS` reader"]
pub type R = crate::R<RssTpccAParityStatusSpec>;
#[doc = "Register `RSS_TPCC_A_PARITY_STATUS` writer"]
pub type W = crate::W<RssTpccAParityStatusSpec>;
#[doc = "Field `parity_addr` reader - 15:8\\]
TPCC Error Address at which Parity Error occurred"]
pub type ParityAddrR = crate::FieldReader;
#[doc = "Field `parity_addr` writer - 15:8\\]
TPCC Error Address at which Parity Error occurred"]
pub type ParityAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
TPCC Error Address at which Parity Error occurred"]
    #[inline(always)]
    pub fn parity_addr(&self) -> ParityAddrR {
        ParityAddrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
TPCC Error Address at which Parity Error occurred"]
    #[inline(always)]
    #[must_use]
    pub fn parity_addr(&mut self) -> ParityAddrW<RssTpccAParityStatusSpec> {
        ParityAddrW::new(self, 8)
    }
}
#[doc = "RSS_TPCC_A_PARITY_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_parity_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_parity_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTpccAParityStatusSpec;
impl crate::RegisterSpec for RssTpccAParityStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tpcc_a_parity_status::R`](R) reader structure"]
impl crate::Readable for RssTpccAParityStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tpcc_a_parity_status::W`](W) writer structure"]
impl crate::Writable for RssTpccAParityStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPCC_A_PARITY_STATUS to value 0"]
impl crate::Resettable for RssTpccAParityStatusSpec {
    const RESET_VALUE: u32 = 0;
}
