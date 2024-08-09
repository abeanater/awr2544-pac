#[doc = "Register `CPSW_NC_STAT_1_ALE_UNKN_MLT_BCNT` reader"]
pub type R = crate::R<CpswNcStat1AleUnknMltBcntSpec>;
#[doc = "Register `CPSW_NC_STAT_1_ALE_UNKN_MLT_BCNT` writer"]
pub type W = crate::W<CpswNcStat1AleUnknMltBcntSpec>;
#[doc = "Field `ALE_RECEIVE_UNKNOWN` reader - 31:0\\]
ALE Receive Unknown Multicast Bytecount"]
pub type AleReceiveUnknownR = crate::FieldReader<u32>;
#[doc = "Field `ALE_RECEIVE_UNKNOWN` writer - 31:0\\]
ALE Receive Unknown Multicast Bytecount"]
pub type AleReceiveUnknownW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Receive Unknown Multicast Bytecount"]
    #[inline(always)]
    pub fn ale_receive_unknown(&self) -> AleReceiveUnknownR {
        AleReceiveUnknownR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Receive Unknown Multicast Bytecount"]
    #[inline(always)]
    #[must_use]
    pub fn ale_receive_unknown(&mut self) -> AleReceiveUnknownW<CpswNcStat1AleUnknMltBcntSpec> {
        AleReceiveUnknownW::new(self, 0)
    }
}
#[doc = "ALE Receive Unknown Multicast Bytecount\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_unkn_mlt_bcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_unkn_mlt_bcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1AleUnknMltBcntSpec;
impl crate::RegisterSpec for CpswNcStat1AleUnknMltBcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_ale_unkn_mlt_bcnt::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1AleUnknMltBcntSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_ale_unkn_mlt_bcnt::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1AleUnknMltBcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_ALE_UNKN_MLT_BCNT to value 0"]
impl crate::Resettable for CpswNcStat1AleUnknMltBcntSpec {
    const RESET_VALUE: u32 = 0;
}
