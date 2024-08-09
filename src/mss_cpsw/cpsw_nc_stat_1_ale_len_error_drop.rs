#[doc = "Register `CPSW_NC_STAT_1_ALE_LEN_ERROR_DROP` reader"]
pub type R = crate::R<CpswNcStat1AleLenErrorDropSpec>;
#[doc = "Register `CPSW_NC_STAT_1_ALE_LEN_ERROR_DROP` writer"]
pub type W = crate::W<CpswNcStat1AleLenErrorDropSpec>;
#[doc = "Field `ALE_LENGTH_ERROR` reader - 31:0\\]
ALE Length Error drop"]
pub type AleLengthErrorR = crate::FieldReader<u32>;
#[doc = "Field `ALE_LENGTH_ERROR` writer - 31:0\\]
ALE Length Error drop"]
pub type AleLengthErrorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Length Error drop"]
    #[inline(always)]
    pub fn ale_length_error(&self) -> AleLengthErrorR {
        AleLengthErrorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Length Error drop"]
    #[inline(always)]
    #[must_use]
    pub fn ale_length_error(&mut self) -> AleLengthErrorW<CpswNcStat1AleLenErrorDropSpec> {
        AleLengthErrorW::new(self, 0)
    }
}
#[doc = "ALE Length Error Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_len_error_drop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_len_error_drop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1AleLenErrorDropSpec;
impl crate::RegisterSpec for CpswNcStat1AleLenErrorDropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_ale_len_error_drop::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1AleLenErrorDropSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_ale_len_error_drop::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1AleLenErrorDropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_ALE_LEN_ERROR_DROP to value 0"]
impl crate::Resettable for CpswNcStat1AleLenErrorDropSpec {
    const RESET_VALUE: u32 = 0;
}
