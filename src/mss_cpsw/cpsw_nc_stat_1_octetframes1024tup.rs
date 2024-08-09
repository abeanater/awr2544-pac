#[doc = "Register `CPSW_NC_STAT_1_OCTETFRAMES1024TUP` reader"]
pub type R = crate::R<CpswNcStat1Octetframes1024tupSpec>;
#[doc = "Register `CPSW_NC_STAT_1_OCTETFRAMES1024TUP` writer"]
pub type W = crate::W<CpswNcStat1Octetframes1024tupSpec>;
#[doc = "Field `TOTAL_NUMBER_OF` reader - 31:0\\]
Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted"]
pub type TotalNumberOfR = crate::FieldReader<u32>;
#[doc = "Field `TOTAL_NUMBER_OF` writer - 31:0\\]
Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted"]
pub type TotalNumberOfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted"]
    #[inline(always)]
    pub fn total_number_of(&self) -> TotalNumberOfR {
        TotalNumberOfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn total_number_of(&mut self) -> TotalNumberOfW<CpswNcStat1Octetframes1024tupSpec> {
        TotalNumberOfW::new(self, 0)
    }
}
#[doc = "Total number of frames of size 1024 to rx_maxlen bytes received and 1024 bytes or greater transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_octetframes1024tup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_octetframes1024tup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1Octetframes1024tupSpec;
impl crate::RegisterSpec for CpswNcStat1Octetframes1024tupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_octetframes1024tup::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1Octetframes1024tupSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_octetframes1024tup::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1Octetframes1024tupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_OCTETFRAMES1024TUP to value 0"]
impl crate::Resettable for CpswNcStat1Octetframes1024tupSpec {
    const RESET_VALUE: u32 = 0;
}
