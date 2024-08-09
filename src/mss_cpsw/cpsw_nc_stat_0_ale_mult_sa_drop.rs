#[doc = "Register `CPSW_NC_STAT_0_ALE_MULT_SA_DROP` reader"]
pub type R = crate::R<CpswNcStat0AleMultSaDropSpec>;
#[doc = "Register `CPSW_NC_STAT_0_ALE_MULT_SA_DROP` writer"]
pub type W = crate::W<CpswNcStat0AleMultSaDropSpec>;
#[doc = "Field `ALE_MULTICAST_SOURCE` reader - 31:0\\]
ALE Multicast Source Address drop"]
pub type AleMulticastSourceR = crate::FieldReader<u32>;
#[doc = "Field `ALE_MULTICAST_SOURCE` writer - 31:0\\]
ALE Multicast Source Address drop"]
pub type AleMulticastSourceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Multicast Source Address drop"]
    #[inline(always)]
    pub fn ale_multicast_source(&self) -> AleMulticastSourceR {
        AleMulticastSourceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Multicast Source Address drop"]
    #[inline(always)]
    #[must_use]
    pub fn ale_multicast_source(&mut self) -> AleMulticastSourceW<CpswNcStat0AleMultSaDropSpec> {
        AleMulticastSourceW::new(self, 0)
    }
}
#[doc = "ALE Multicast Source Address Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_mult_sa_drop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_mult_sa_drop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat0AleMultSaDropSpec;
impl crate::RegisterSpec for CpswNcStat0AleMultSaDropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_0_ale_mult_sa_drop::R`](R) reader structure"]
impl crate::Readable for CpswNcStat0AleMultSaDropSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_0_ale_mult_sa_drop::W`](W) writer structure"]
impl crate::Writable for CpswNcStat0AleMultSaDropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_0_ALE_MULT_SA_DROP to value 0"]
impl crate::Resettable for CpswNcStat0AleMultSaDropSpec {
    const RESET_VALUE: u32 = 0;
}
