#[doc = "Register `RSS_SHARED_MEM_MEMINIT_DONE` reader"]
pub type R = crate::R<RssSharedMemMeminitDoneSpec>;
#[doc = "Register `RSS_SHARED_MEM_MEMINIT_DONE` writer"]
pub type W = crate::W<RssSharedMemMeminitDoneSpec>;
#[doc = "Field `done` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
pub type DoneR = crate::BitReader;
#[doc = "Field `done` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<RssSharedMemMeminitDoneSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "RSS_SHARED_MEM_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_shared_mem_meminit_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_shared_mem_meminit_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssSharedMemMeminitDoneSpec;
impl crate::RegisterSpec for RssSharedMemMeminitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_shared_mem_meminit_done::R`](R) reader structure"]
impl crate::Readable for RssSharedMemMeminitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_shared_mem_meminit_done::W`](W) writer structure"]
impl crate::Writable for RssSharedMemMeminitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_SHARED_MEM_MEMINIT_DONE to value 0"]
impl crate::Resettable for RssSharedMemMeminitDoneSpec {
    const RESET_VALUE: u32 = 0;
}
