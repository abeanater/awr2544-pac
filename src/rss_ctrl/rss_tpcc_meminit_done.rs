#[doc = "Register `RSS_TPCC_MEMINIT_DONE` reader"]
pub type R = crate::R<RssTpccMeminitDoneSpec>;
#[doc = "Register `RSS_TPCC_MEMINIT_DONE` writer"]
pub type W = crate::W<RssTpccMeminitDoneSpec>;
#[doc = "Field `tpcc_a_meminit_done` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
pub type TpccAMeminitDoneR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_done` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
pub type TpccAMeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
    #[inline(always)]
    pub fn tpcc_a_meminit_done(&self) -> TpccAMeminitDoneR {
        TpccAMeminitDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_done(&mut self) -> TpccAMeminitDoneW<RssTpccMeminitDoneSpec> {
        TpccAMeminitDoneW::new(self, 0)
    }
}
#[doc = "RSS_TPCC_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_meminit_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_meminit_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTpccMeminitDoneSpec;
impl crate::RegisterSpec for RssTpccMeminitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tpcc_meminit_done::R`](R) reader structure"]
impl crate::Readable for RssTpccMeminitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tpcc_meminit_done::W`](W) writer structure"]
impl crate::Writable for RssTpccMeminitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPCC_MEMINIT_DONE to value 0"]
impl crate::Resettable for RssTpccMeminitDoneSpec {
    const RESET_VALUE: u32 = 0;
}
