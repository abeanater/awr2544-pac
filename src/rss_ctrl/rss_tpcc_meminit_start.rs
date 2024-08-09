#[doc = "Register `RSS_TPCC_MEMINIT_START` reader"]
pub type R = crate::R<RssTpccMeminitStartSpec>;
#[doc = "Register `RSS_TPCC_MEMINIT_START` writer"]
pub type W = crate::W<RssTpccMeminitStartSpec>;
#[doc = "Field `tpcc_a_meminit_start` reader - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed."]
pub type TpccAMeminitStartR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_start` writer - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed."]
pub type TpccAMeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed."]
    #[inline(always)]
    pub fn tpcc_a_meminit_start(&self) -> TpccAMeminitStartR {
        TpccAMeminitStartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_start(&mut self) -> TpccAMeminitStartW<RssTpccMeminitStartSpec> {
        TpccAMeminitStartW::new(self, 0)
    }
}
#[doc = "RSS_TPCC_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_meminit_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_meminit_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTpccMeminitStartSpec;
impl crate::RegisterSpec for RssTpccMeminitStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tpcc_meminit_start::R`](R) reader structure"]
impl crate::Readable for RssTpccMeminitStartSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tpcc_meminit_start::W`](W) writer structure"]
impl crate::Writable for RssTpccMeminitStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPCC_MEMINIT_START to value 0"]
impl crate::Resettable for RssTpccMeminitStartSpec {
    const RESET_VALUE: u32 = 0;
}
