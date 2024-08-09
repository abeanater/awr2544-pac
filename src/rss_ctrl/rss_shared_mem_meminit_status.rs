#[doc = "Register `RSS_SHARED_MEM_MEMINIT_STATUS` reader"]
pub type R = crate::R<RssSharedMemMeminitStatusSpec>;
#[doc = "Register `RSS_SHARED_MEM_MEMINIT_STATUS` writer"]
pub type W = crate::W<RssSharedMemMeminitStatusSpec>;
#[doc = "Field `status` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
pub type StatusR = crate::BitReader;
#[doc = "Field `status` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<RssSharedMemMeminitStatusSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "RSS_SHARED_MEM_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_shared_mem_meminit_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_shared_mem_meminit_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssSharedMemMeminitStatusSpec;
impl crate::RegisterSpec for RssSharedMemMeminitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_shared_mem_meminit_status::R`](R) reader structure"]
impl crate::Readable for RssSharedMemMeminitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_shared_mem_meminit_status::W`](W) writer structure"]
impl crate::Writable for RssSharedMemMeminitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_SHARED_MEM_MEMINIT_STATUS to value 0"]
impl crate::Resettable for RssSharedMemMeminitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
