#[doc = "Register `CPSW_NC_STAT_0_OCTETFRAMES128T255` reader"]
pub type R = crate::R<CpswNcStat0Octetframes128t255Spec>;
#[doc = "Register `CPSW_NC_STAT_0_OCTETFRAMES128T255` writer"]
pub type W = crate::W<CpswNcStat0Octetframes128t255Spec>;
#[doc = "Field `TOTAL_NUMBER_OF` reader - 31:0\\]
Total number of frames of size 128 to 255 bytes received and transmitted"]
pub type TotalNumberOfR = crate::FieldReader<u32>;
#[doc = "Field `TOTAL_NUMBER_OF` writer - 31:0\\]
Total number of frames of size 128 to 255 bytes received and transmitted"]
pub type TotalNumberOfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Total number of frames of size 128 to 255 bytes received and transmitted"]
    #[inline(always)]
    pub fn total_number_of(&self) -> TotalNumberOfR {
        TotalNumberOfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Total number of frames of size 128 to 255 bytes received and transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn total_number_of(&mut self) -> TotalNumberOfW<CpswNcStat0Octetframes128t255Spec> {
        TotalNumberOfW::new(self, 0)
    }
}
#[doc = "Total number of frames of size 128 to 255 bytes received and transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_octetframes128t255::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_octetframes128t255::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat0Octetframes128t255Spec;
impl crate::RegisterSpec for CpswNcStat0Octetframes128t255Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_0_octetframes128t255::R`](R) reader structure"]
impl crate::Readable for CpswNcStat0Octetframes128t255Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_0_octetframes128t255::W`](W) writer structure"]
impl crate::Writable for CpswNcStat0Octetframes128t255Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_0_OCTETFRAMES128T255 to value 0"]
impl crate::Resettable for CpswNcStat0Octetframes128t255Spec {
    const RESET_VALUE: u32 = 0;
}
