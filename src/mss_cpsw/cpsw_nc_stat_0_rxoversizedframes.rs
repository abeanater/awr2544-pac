#[doc = "Register `CPSW_NC_STAT_0_RXOVERSIZEDFRAMES` reader"]
pub type R = crate::R<CpswNcStat0RxoversizedframesSpec>;
#[doc = "Register `CPSW_NC_STAT_0_RXOVERSIZEDFRAMES` writer"]
pub type W = crate::W<CpswNcStat0RxoversizedframesSpec>;
#[doc = "Field `TOTAL_NUMBER_OF` reader - 31:0\\]
Total number of oversized frames received"]
pub type TotalNumberOfR = crate::FieldReader<u32>;
#[doc = "Field `TOTAL_NUMBER_OF` writer - 31:0\\]
Total number of oversized frames received"]
pub type TotalNumberOfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Total number of oversized frames received"]
    #[inline(always)]
    pub fn total_number_of(&self) -> TotalNumberOfR {
        TotalNumberOfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Total number of oversized frames received"]
    #[inline(always)]
    #[must_use]
    pub fn total_number_of(&mut self) -> TotalNumberOfW<CpswNcStat0RxoversizedframesSpec> {
        TotalNumberOfW::new(self, 0)
    }
}
#[doc = "Total number of oversized frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_rxoversizedframes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_rxoversizedframes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat0RxoversizedframesSpec;
impl crate::RegisterSpec for CpswNcStat0RxoversizedframesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_0_rxoversizedframes::R`](R) reader structure"]
impl crate::Readable for CpswNcStat0RxoversizedframesSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_0_rxoversizedframes::W`](W) writer structure"]
impl crate::Writable for CpswNcStat0RxoversizedframesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_0_RXOVERSIZEDFRAMES to value 0"]
impl crate::Resettable for CpswNcStat0RxoversizedframesSpec {
    const RESET_VALUE: u32 = 0;
}
