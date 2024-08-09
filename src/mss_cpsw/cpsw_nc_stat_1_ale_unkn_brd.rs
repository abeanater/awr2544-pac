#[doc = "Register `CPSW_NC_STAT_1_ALE_UNKN_BRD` reader"]
pub type R = crate::R<CpswNcStat1AleUnknBrdSpec>;
#[doc = "Register `CPSW_NC_STAT_1_ALE_UNKN_BRD` writer"]
pub type W = crate::W<CpswNcStat1AleUnknBrdSpec>;
#[doc = "Field `ALE_RECEIVE_UNKNOWN` reader - 31:0\\]
ALE Receive Unknown Broadcast"]
pub type AleReceiveUnknownR = crate::FieldReader<u32>;
#[doc = "Field `ALE_RECEIVE_UNKNOWN` writer - 31:0\\]
ALE Receive Unknown Broadcast"]
pub type AleReceiveUnknownW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Receive Unknown Broadcast"]
    #[inline(always)]
    pub fn ale_receive_unknown(&self) -> AleReceiveUnknownR {
        AleReceiveUnknownR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Receive Unknown Broadcast"]
    #[inline(always)]
    #[must_use]
    pub fn ale_receive_unknown(&mut self) -> AleReceiveUnknownW<CpswNcStat1AleUnknBrdSpec> {
        AleReceiveUnknownW::new(self, 0)
    }
}
#[doc = "ALE Receive Unknown Broadcast\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_unkn_brd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_unkn_brd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1AleUnknBrdSpec;
impl crate::RegisterSpec for CpswNcStat1AleUnknBrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_ale_unkn_brd::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1AleUnknBrdSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_ale_unkn_brd::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1AleUnknBrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_ALE_UNKN_BRD to value 0"]
impl crate::Resettable for CpswNcStat1AleUnknBrdSpec {
    const RESET_VALUE: u32 = 0;
}
